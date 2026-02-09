use std::array::TryFromSliceError;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::time::Duration;

use eyre::{Context, ContextCompat, eyre};
use futures::StreamExt;
use prost::Message;
use time::OffsetDateTime;
use time::ext::NumericalDuration;

use crate::domain::account::AccountResourceUsage;
use crate::domain::account::{Account, AccountStatus};
use crate::domain::address::TronAddress;
use crate::domain::contract::TriggerSmartContract;
use crate::domain::estimate::{MissingResource, Resource, ResourceState};
use crate::domain::permission::Permission;
use crate::domain::transaction::{Transaction, TransactionInfo, TxCode};
use crate::domain::trx::Trx;
use crate::domain::{Hash32, RecoverableSignature};
use crate::error;
use crate::error::Error;
use crate::listener::ListenerError;
use crate::provider::TronProvider;
use crate::signer::PrehashSigner;
use crate::utility::generate_txid;
use crate::{Result, protocol, utility};
use crate::{domain, trx};

use super::Client;

pub struct AutoSigning;
pub struct ManualSigning;

#[derive(Clone, Copy, Debug)]
pub struct ActivationFeeCheck {
    pub address: TronAddress,
    pub fee: Trx,
}

// Todo: it's possible to implement PendingTransactionCache
// here to reduce api calls count, and make interaction faster.
// But expiration for entries is required. For example, for a BlockExtention
// 1-2 seconds should be enough.
#[allow(dead_code)]
struct Cache {
    estimated_energy: Trx,
    bandwidth_price: Trx,
    energy_price: Trx,
    account_balance: Trx,
    account_resources: AccountResourceUsage,
}

pub struct PendingTransaction<'a, P, S, M = AutoSigning> {
    pub(super) client: &'a Client<P, S>,
    pub(super) transaction: Transaction,
    pub(super) txid: Hash32,
    pub(super) _mode: PhantomData<M>,
    pub(super) owner: TronAddress,

    /// TRX which must be available by this transaction aside from bandwidth or energy fees
    pub(super) base_trx_required: Trx,

    /// If address does not exist at estimate time
    pub(super) activation_checks: Vec<ActivationFeeCheck>,

    pub(super) can_spend_trx_for_fee: bool,

    /// Cache energy in this PendingTransaction lifecycle
    pub(super) cached_energy: Mutex<Option<i64>>,
}

impl<'a, P, S, M> PendingTransaction<'a, P, S, M>
where
    P: TronProvider,
    S: PrehashSigner,
    error::Error: From<S::Error>,
{
    pub async fn new(
        client: &'a Client<P, S>,
        transaction: Transaction,
        owner: TronAddress,
        base_trx_required: Trx,
        activation_checks: Vec<ActivationFeeCheck>,
        can_spend_trx_for_fee: bool,
    ) -> Result<Self> {
        let mut pending_transaction = Self {
            client,
            transaction,
            txid: Default::default(),
            _mode: PhantomData,
            owner,
            base_trx_required,
            activation_checks,
            can_spend_trx_for_fee,
            cached_energy: Mutex::new(None),
        };

        pending_transaction.update_fee_limit().await?;
        pending_transaction.refresh_txid().await?;
        Ok(pending_transaction)
    }
    async fn refresh_txid(&mut self) -> Result<()> {
        if !self.transaction.signature.is_empty() {
            return Err(Error::PreconditionFailed(
                "can't update txid for signed transaction".into(),
            ));
        }
        let latest_block = self.client.provider.get_now_block().await?;
        latest_block.fill_header_info_in_transaction(&mut self.transaction);
        let txid = generate_txid(
            &protocol::transaction::Raw::from(self.transaction.raw.clone())
                .encode_to_vec(),
        );
        self.txid = txid;
        Ok(())
    }
    fn ensure_unsigned(&self) -> Result<()> {
        if !self.transaction.signature.is_empty() {
            return Err(Error::PreconditionFailed(
                "operation is only allowed for unsigned transaction".into(),
            ));
        }
        Ok(())
    }
    fn only_fee_resources_missing(missing: &[MissingResource]) -> bool {
        missing.iter().all(|m| {
            matches!(
                m,
                MissingResource::Energy { .. }
                    | MissingResource::Bandwidth { .. }
            )
        })
    }
    fn can_cover_missing_with_trx_fee(&self, state: &ResourceState) -> bool {
        if state.insufficient.is_none() {
            return true;
        }
        if !self.can_spend_trx_for_fee {
            return false;
        }
        let ins = state.insufficient.as_ref().unwrap();
        if !Self::only_fee_resources_missing(&ins.missing) {
            return false;
        }
        let suggested: Trx =
            ins.suggested_trx_topup.iter().map(|(_, t)| *t).sum();
        suggested <= state.remaining.trx
    }
    fn fee_limit_fallback(&self) -> Trx {
        if let Some(c) = self.transaction.raw.contract.first() {
            match c.contract_type {
                domain::contract::ContractType::CreateSmartContract(_) => {
                    trx!(1000.0 TRX)
                }
                domain::contract::ContractType::TriggerSmartContract(_) => {
                    trx!(200.0 TRX)
                }
                _ => trx!(50.0 TRX),
            }
        } else {
            trx!(50.0 TRX)
        }
    }
    async fn update_fee_limit(&mut self) -> Result<()> {
        let fee_limit_trx = match self.estimate_energy_cached().await {
            Ok(Some(energy)) => {
                let energy_price = self.client.energy_price().await?;
                let sun =
                    (((energy as f64) * 1.5) as i64) * energy_price.to_sun();
                Trx::from_sun(sun)
            }
            Ok(None) => self.fee_limit_fallback(),
            Err(e) => return Err(e),
        };

        self.transaction.raw.fee_limit = fee_limit_trx.to_sun().into();
        Ok(())
    }
    /// Re-estimate energy, update fee_limit, refresh txid. Only for unsigned tx.
    pub async fn reset_estimates(&mut self) -> Result<()> {
        self.ensure_unsigned()?;

        let mut guard = self
            .cached_energy
            .lock()
            .map_err(|e| eyre!("failed to acquire mutex: {e:#?}"))?;
        *guard = None;
        drop(guard);

        self.update_fee_limit().await?;
        self.refresh_txid().await?;
        Ok(())
    }
    fn estimate_bandwidth_with_permission(
        &self,
        permission: &Permission,
    ) -> Result<i64> {
        let raw = self.transaction.raw.clone();
        let signature_count = permission
            .required_signatures()
            .context("insufficient keys for threshold")?;
        let txlen = protocol::transaction::Raw::from(raw).encode_to_vec().len();
        Ok(utility::estimate_bandwidth(txlen as i64, signature_count))
    }
    async fn activation_fee(&self) -> Result<Trx> {
        let mut total = Trx::ZERO;
        for c in &self.activation_checks {
            let st = self.client.check_account(c.address).await?;
            if matches!(st, AccountStatus::NotExists) {
                total += c.fee;
            }
        }
        Ok(total)
    }
    async fn required_trx(&self) -> Result<Trx> {
        Ok(self.base_trx_required + self.activation_fee().await?)
    }
    async fn estimate_energy_cached(&self) -> Result<Option<i64>> {
        let mut guard = self
            .cached_energy
            .lock()
            .map_err(|e| eyre!("failed to acquire mutex: {e:#?}"))?;
        if let Some(v) = *guard {
            return Ok(Some(v));
        }
        let v = self.estimate_energy().await;
        *guard = v;
        Ok(v)
    }
    async fn estimate_transaction_with_account(
        &self,
        account: &Account,
    ) -> Result<ResourceState> {
        let permission_id = self
            .transaction
            .raw
            .contract
            .first()
            .context("no contract found")?
            .permission_id;

        let permission = account
            .permission_by_id(permission_id)
            .context("no permission found")?;

        let bandwidth = self.estimate_bandwidth_with_permission(&permission)?;

        let (resources, energy, required_trx, energy_price) = tokio::try_join!(
            self.client.provider.get_account_resources(self.owner),
            self.estimate_energy_cached(),
            self.required_trx(),
            self.client.energy_price()
        )?;

        let required = Resource {
            bandwidth,
            energy: energy.unwrap_or_else(|| {
                let fee_limit_sun = self.transaction.raw.fee_limit.to_sun();
                let price_sun = energy_price.to_sun().max(1);
                (fee_limit_sun + price_sun - 1) / price_sun
            }),
            trx: required_trx,
        };

        ResourceState::estimate(
            self.client,
            &resources,
            required,
            account.balance,
        )
        .await
    }
    /// Validate without requiring signatures
    pub(crate) async fn validate_unsigned(&self) -> Result<()> {
        if self.transaction.raw.expiration < OffsetDateTime::now_utc() {
            return Err(Error::Expired(self.transaction.raw.expiration));
        }
        let account = self.client.provider.get_account(self.owner).await?;
        let state = self.estimate_transaction_with_account(&account).await?;
        if !self.can_cover_missing_with_trx_fee(&state) {
            return Err(Error::InsufficientResources(state));
        }
        Ok(())
    }
    pub async fn estimate_bandwidth(&self) -> Result<i64> {
        let raw = self.transaction.raw.clone();
        let contract = raw.contract.first().wrap_err("no contract")?;
        let permission_id = contract.permission_id;

        let account = self.client.provider.get_account(self.owner).await?;
        let permission = account
            .permission_by_id(permission_id)
            .wrap_err("no permission found")?;

        self.estimate_bandwidth_with_permission(&permission)
    }
    pub async fn estimate_energy(&self) -> Option<i64> {
        let safe_call = |c: TriggerSmartContract| async move {
            match self.client.provider.trigger_constant_contract(c).await {
                Ok(txext) => Some(txext.energy_used),
                Err(e) => {
                    tracing::warn!(
                        ?e,
                        "energy estimation failed, fallback to fee_limit"
                    );
                    None
                }
            }
        };
        if let Some(contract) = self.transaction.raw.contract.first() {
            match contract.contract_type {
                domain::contract::ContractType::TriggerSmartContract(ref c) => {
                    let energy = safe_call(c.clone()).await;
                    return energy;
                }
                domain::contract::ContractType::CreateSmartContract(
                    ref contract,
                ) => {
                    let bytecode = contract.new_contract.bytecode.clone();
                    let energy = safe_call(TriggerSmartContract {
                        owner_address: contract.owner_address,
                        data: bytecode.into(),
                        call_token_value: contract.call_token_value,
                        ..Default::default()
                    })
                    .await;
                    return energy;
                }
                domain::contract::ContractType::TransferContract(_) => {
                    return Some(0);
                }
                _ => (),
            }
        }
        None
    }
    pub async fn estimate_transaction(&self) -> Result<ResourceState> {
        let account = self.client.provider.get_account(self.owner).await?;
        self.estimate_transaction_with_account(&account).await
    }
    pub(crate) async fn validate_transaction(&self) -> Result<()> {
        let txid = self.txid;

        let signers = self
            .transaction
            .signature
            .iter()
            .map(|s| s.recover_address(&txid))
            .collect::<Result<Vec<_>>>()?;

        let account = self.client.provider.get_account(self.owner).await?;

        let permission_id = self
            .transaction
            .raw
            .contract
            .first()
            .context("no contract found")?
            .permission_id;

        let permission = account
            .permission_by_id(permission_id)
            .context("no permission found")?;

        if !permission.enough_sign_weight(signers) {
            return Err(Error::PreconditionFailed("not enough weight".into()));
        }

        if self.transaction.raw.expiration < OffsetDateTime::now_utc() {
            return Err(Error::Expired(self.transaction.raw.expiration));
        }

        let state = self.estimate_transaction_with_account(&account).await?;

        if !self.can_cover_missing_with_trx_fee(&state) {
            return Err(Error::InsufficientResources(state));
        }

        Ok(())
    }
    pub fn txid(&self) -> Hash32 {
        self.txid
    }
    pub fn transaction(&self) -> Transaction {
        self.transaction.clone()
    }
    /// Expiration is limited to 24 hours
    pub async fn set_expiration(
        mut self,
        expiration: time::Duration,
    ) -> Result<Self> {
        let timestamp = self.transaction.raw.timestamp;
        let new_expiration = timestamp.saturating_add(expiration);
        if new_expiration > timestamp.saturating_add(24.hours()) {
            return Err(Error::InvalidInput(
                "expiration is limited to 24 hours".into(),
            ));
        }
        self.transaction.raw.expiration = new_expiration;
        self.refresh_txid().await?;
        Ok(self)
    }
}

impl<'a, P, S> PendingTransaction<'a, P, S, AutoSigning>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    error::Error: From<S::Error>,
{
    pub async fn broadcast(mut self, ctx: &S::Ctx) -> Result<Hash32> {
        self.validate_unsigned().await?;

        let signer =
            self.client
                .signer
                .as_ref()
                .ok_or(Error::PreconditionFailed(
                    "no signer set for automatic signing".into(),
                ))?;
        let recoverable_signature =
            signer.sign_recoverable(&self.txid, ctx).await?;

        self.transaction.signature.push(recoverable_signature);

        self.validate_transaction().await?;

        self.client
            .provider
            .broadcast_transaction(self.transaction)
            .await?;

        Ok(self.txid)
    }
    /// Wait for confirmations and get transaction info
    pub async fn broadcast_get_receipt(
        self,
        ctx: &S::Ctx,
        confirmations: i32,
    ) -> std::result::Result<TransactionInfo, ListenerError>
    where
        P: Clone + Send + Sync + 'static,
        S: Send + Sync + 'static,
        <S as crate::signer::PrehashSigner>::Error: std::fmt::Debug,
    {
        let client = self.client.to_owned();
        let txid = self.broadcast(ctx).await?;
        transaction_receipt(confirmations, client, txid).await
    }
}

impl<'a, P, S> PendingTransaction<'a, P, S, ManualSigning>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    error::Error: From<S::Error>,
{
    pub async fn set_permission(&mut self, id: i32) -> Result<()> {
        let permission = self
            .client
            .provider
            .get_account(self.owner)
            .await?
            .permission_by_id(id)
            .ok_or(Error::NotFound("no permission with id found".into()))?;
        self.transaction
            .raw
            .contract
            .first_mut()
            .context("no contract part found")?
            .permission_id = id;
        self.refresh_txid().await?;

        if permission.keys.len() > 1 {
            // Multisig fee
            self.base_trx_required += trx!(1.0 TRX);
        }

        Ok(())
    }
    pub async fn sign(
        &mut self,
        signer: &S,
        ctx: &S::Ctx,
    ) -> Result<RecoverableSignature> {
        let txid = &self.txid;

        let recoverable_signature = signer.sign_recoverable(txid, ctx).await?;

        // Check signatures
        let signing_addr = recoverable_signature.recover_address(txid)?;
        if self
            .transaction
            .signature
            .iter()
            .map(|s| s.recover_address(txid))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .any(|a| a.eq(&signing_addr))
        {
            return Err(Error::PreconditionFailed(
                "address already signer".into(),
            ));
        }
        // Check address contained in permission
        let permission_id = self
            .transaction
            .raw
            .contract
            .first()
            .context("no contract found")?
            .permission_id;

        let permission = self
            .client
            .provider
            .get_account(self.owner)
            .await?
            .permission_by_id(permission_id)
            .context("no permission found")?;
        if !permission.contains(signing_addr) {
            return Err(Error::InvalidInput(format!(
                "{signing_addr} is not in permission"
            )));
        }

        self.transaction
            .signature
            .push(recoverable_signature.clone());
        Ok(recoverable_signature)
    }
    /// Signs a raw transaction hash using an external signing function without performing
    /// standard validation checks.
    ///
    /// This method is specifically designed for custom signing scenarios where:
    /// - The signature is generated externally (e.g., through MPC protocol)
    /// - You need to bypass standard permission and signature validation
    /// - You want to inject a pre-computed signature into the transaction
    ///
    /// # Security Notes
    ///
    /// ⚠️ **USE WITH CAUTION**: This method skips critical security validations:
    /// - Does not verify the signer has permission to sign this transaction
    /// - Does not check for duplicate signatures from the same address
    /// - Does not validate signature recovery against transaction permissions
    ///
    /// Only use this method when you have performed these validations externally
    /// and are confident in the signing process (e.g., in MPC protocols).
    ///
    /// # Parameters
    ///
    /// * `signer` - The verifying key that will be used to validate signature recovery
    /// * `with` - Async closure that receives the transaction hash and returns a signature
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The signature recovery fails
    /// - The external signing function returns an error
    pub async fn sign_tx_unchecked<F, Fut>(
        &mut self,
        signer: k256::ecdsa::VerifyingKey,
        with: F,
    ) -> Result<()>
    where
        F: FnOnce(Hash32) -> Fut,
        Fut: Future<Output = Result<k256::ecdsa::Signature>>,
    {
        let signature = with(self.txid()).await?;
        let recovery_id = k256::ecdsa::RecoveryId::trial_recovery_from_prehash(
            &signer,
            self.txid().as_ref(),
            &signature,
        )?;
        let signature = RecoverableSignature::new(signature, recovery_id);

        self.transaction.signature.push(signature);
        Ok(())
    }
    pub async fn broadcast(self) -> Result<Hash32> {
        let txid = self.txid;

        self.validate_transaction().await?;

        self.client
            .provider
            .broadcast_transaction(self.transaction)
            .await?;
        Ok(txid)
    }
    /// Wait for confirmations and get transaction info
    pub async fn broadcast_get_receipt(
        self,
        confirmations: i32,
    ) -> std::result::Result<TransactionInfo, ListenerError>
    where
        P: Clone + Send + Sync + 'static,
        S: Send + Sync + 'static,
        <S as crate::signer::PrehashSigner>::Error: std::fmt::Debug,
    {
        let client = self.client.to_owned();
        let txid = self.broadcast().await?;
        transaction_receipt(confirmations, client, txid).await
    }
    pub fn serialize(&self) -> Vec<u8> {
        const MAGIC: &[u8; 4] = b"PTX1";

        let transaction = protocol::Transaction::from(self.transaction.clone())
            .encode_to_vec();

        let mut out = Vec::with_capacity(
            4 + 32
                + 21
                + 8
                + 1
                + 1
                + self.activation_checks.len() * (21 + 8)
                + transaction.len(),
        );

        out.extend_from_slice(MAGIC);
        out.extend_from_slice(self.txid.as_ref());
        out.extend_from_slice(self.owner.as_bytes());
        out.extend_from_slice(&self.base_trx_required.to_sun().to_le_bytes());
        out.push(self.can_spend_trx_for_fee as u8);

        out.push(self.activation_checks.len() as u8);
        for c in &self.activation_checks {
            out.extend_from_slice(c.address.as_bytes());
            out.extend_from_slice(&c.fee.to_sun().to_le_bytes());
        }

        out.extend_from_slice(&transaction);
        out
    }
    pub fn try_deserialize(
        client: &'a Client<P, S>,
        data: &[u8],
    ) -> Result<Self> {
        const MAGIC: &[u8; 4] = b"PTX1";

        // MIN: 4 + 32 + 21 + 8 + 1 + 1
        if data.len() < 67 {
            return Err(Error::InvalidInput(format!(
                "min data length is 67, got {}",
                data.len()
            )));
        }

        if &data[..4] != MAGIC {
            return Err(Error::InvalidInput(
                "invalid pending tx format".into(),
            ));
        }

        let mut cursor = 4;

        let txid: Hash32 = data[cursor..cursor + 32]
            .try_into()
            .map_err(Error::InvalidInput)?;
        cursor += 32;

        let owner = TronAddress::new(
            *<&[u8; 21]>::try_from(&data[cursor..cursor + 21])
                .map_err(|e| Error::InvalidInput(e.to_string()))?,
        )?;
        cursor += 21;

        let base_trx_required: Trx =
            i64::from_le_bytes(data[cursor..cursor + 8].try_into().map_err(
                |e: TryFromSliceError| Error::InvalidInput(e.to_string()),
            )?)
            .into();
        cursor += 8;

        let can_spend_trx_for_fee = data[cursor] != 0;
        cursor += 1;

        let checks_count = data[cursor] as usize;
        cursor += 1;

        let need = cursor + checks_count * (21 + 8);
        if data.len() < need {
            return Err(Error::InvalidInput(format!(
                "not enough bytes for activation checks: need {}, got {}",
                need,
                data.len()
            )));
        }

        let mut activation_checks = Vec::with_capacity(checks_count);
        for _ in 0..checks_count {
            let addr = TronAddress::new(
                *<&[u8; 21]>::try_from(&data[cursor..cursor + 21])
                    .map_err(|e| Error::InvalidInput(e.to_string()))?,
            )?;
            cursor += 21;

            let fee: Trx = i64::from_le_bytes(
                data[cursor..cursor + 8].try_into().map_err(
                    |e: TryFromSliceError| Error::InvalidInput(e.to_string()),
                )?,
            )
            .into();
            cursor += 8;

            activation_checks.push(ActivationFeeCheck { address: addr, fee });
        }

        let transaction_data = &data[cursor..];

        let transaction: domain::transaction::Transaction =
            protocol::Transaction::decode(transaction_data)?
                .try_into()
                .map_err(Error::ProtoConv)?;

        Ok(Self {
            client,
            transaction,
            txid,
            _mode: PhantomData,
            owner,
            base_trx_required,
            activation_checks,
            can_spend_trx_for_fee,
            cached_energy: Mutex::new(None),
        })
    }
}

async fn transaction_receipt<P, S>(
    confirmations: i32,
    client: Client<P, S>,
    txid: Hash32,
) -> std::result::Result<TransactionInfo, ListenerError>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    error::Error: From<S::Error>,
    <S as crate::signer::PrehashSigner>::Error: std::fmt::Debug,
{
    let listener =
        crate::listener::Listener::new(client.clone(), Duration::from_secs(3));
    let mut block_stream = listener.block_stream();

    // Track the latest block number we've seen
    let mut last_block_number = 0;
    let mut confirmation_count = 0;
    let mut initial_tx_info: Option<TransactionInfo> = None;

    while let Some(block_ext) = block_stream.next().await {
        let block_ext = block_ext?;
        // Only check if we got a new block
        if block_ext.block_header.raw_data.number > last_block_number {
            last_block_number = block_ext.block_header.raw_data.number;

            match client.provider().get_transaction_info(txid).await {
                Ok(tx_info)
                    if tx_info
                        .block_time_stamp
                        .eq(&OffsetDateTime::UNIX_EPOCH) =>
                {
                    continue;
                }
                Ok(tx_info) => {
                    // First check if transaction is in a block
                    if tx_info.block_number > 0 {
                        // Store the initial transaction info if we haven't already
                        if initial_tx_info.is_none() {
                            initial_tx_info = Some(tx_info.clone());
                        }

                        // Then check if the transaction was successful
                        match tx_info.result {
                            TxCode::Sucess => {
                                confirmation_count += 1;

                                // Return when we reach the required confirmations
                                if confirmation_count >= confirmations {
                                    return Ok(tx_info);
                                }
                            }
                            TxCode::Failed => {
                                return Err(Error::Transaction {
                                    txid,
                                    result: tx_info.result,
                                    msg: tx_info.res_message,
                                }
                                .into());
                            }
                        }
                    }
                }
                Err(e) => {
                    // If we had initial confirmation but now get an error, that's bad
                    if initial_tx_info.is_some() {
                        return Err(e.into());
                    }
                    // Otherwise just continue waiting
                }
            }
        }
    }

    Err(Error::TransactionTimeout.into())
}
