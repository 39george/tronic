use std::array::TryFromSliceError;
use std::marker::PhantomData;
use std::time::Duration;

use anyhow::Context;
use futures::StreamExt;
use prost::Message;
use time::OffsetDateTime;
use time::ext::NumericalDuration;

use crate::domain::account::AccountResourceUsage;
use crate::domain::address::TronAddress;
use crate::domain::contract::TriggerSmartContract;
use crate::domain::estimate::{InsufficientResource, Resource, ResourceState};
use crate::domain::permission::Permission;
use crate::domain::transaction::{Transaction, TransactionInfo, TxCode};
use crate::domain::trx::Trx;
use crate::domain::{Hash32, RecoverableSignature};
use crate::error;
use crate::error::Error;
use crate::provider::TronProvider;
use crate::signer::PrehashSigner;
use crate::utility::generate_txid;
use crate::{Result, protocol, utility};
use crate::{domain, trx};

use super::Client;

pub struct AutoSigning;
pub struct ManualSigning;

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
    pub(super) additional_fee: Trx,
    pub(super) can_spend_trx_for_fee: bool,
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
        additional_fee: Trx,
        can_spend_trx_for_fee: bool,
    ) -> Result<Self> {
        let mut pending_transaction = Self {
            client,
            transaction,
            txid: Default::default(),
            _mode: PhantomData,
            owner,
            additional_fee,
            can_spend_trx_for_fee,
        };
        let energy = pending_transaction.estimate_energy().await?;
        let energy_price = client.energy_price().await?;
        pending_transaction.transaction.raw.fee_limit =
            (((energy as f64) * 1.5) as i64 * energy_price.to_sun()).into();
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
    pub async fn estimate_bandwidth(&self) -> Result<i64> {
        let raw = self.transaction.raw.clone();
        let contract = raw.contract.first().context("no contract")?;
        let permission_id = contract.permission_id;
        let signature_count = self
            .client
            .provider
            .get_account(self.owner)
            .await?
            .permission_by_id(permission_id)
            .context("no permission found")?
            .required_signatures()
            .context("insufficient keys for threshold")?;
        let txlen = protocol::transaction::Raw::from(raw).encode_to_vec().len();
        Ok(utility::estimate_bandwidth(txlen as i64, signature_count))
    }
    pub async fn estimate_energy(&self) -> Result<i64> {
        if let Some(contract) = self.transaction.raw.contract.first() {
            match contract.contract_type {
                domain::contract::ContractType::TriggerSmartContract(
                    ref contract,
                ) => {
                    let txext = self
                        .client
                        .provider
                        .trigger_constant_contract(contract.clone())
                        .await?;
                    return Ok(txext.energy_used);
                }
                domain::contract::ContractType::CreateSmartContract(
                    ref contract,
                ) => {
                    let bytecode = contract.new_contract.bytecode.clone();
                    let txext = self
                        .client
                        .provider
                        .trigger_constant_contract(TriggerSmartContract {
                            owner_address: contract.owner_address,
                            data: bytecode.into(),
                            ..Default::default()
                        })
                        .await?;
                    return Ok(txext.energy_used);
                }
                _ => (),
            }
        }
        Ok(0)
    }
    pub async fn estimate_transaction(&self) -> Result<ResourceState> {
        let (resources, balance, bandwidth, energy) = tokio::try_join!(
            self.client.provider.get_account_resources(self.owner),
            self.client.trx_balance().address(self.owner).get(),
            self.estimate_bandwidth(),
            self.estimate_energy()
        )?;
        let required = Resource {
            bandwidth,
            energy,
            trx: self.additional_fee,
        };
        ResourceState::estimate(self.client, &resources, required, balance)
            .await
    }
    pub(crate) async fn validate_transaction(&self) -> Result<()> {
        let txid = self.txid;
        let signers = self
            .transaction
            .signature
            .iter()
            .map(|s| s.recover_address(&txid))
            .collect::<Result<Vec<_>>>()?;
        if !self.extract_permission().await?.enough_sign_weight(signers) {
            return Err(Error::PreconditionFailed("not enough weight".into()));
        }
        if self.transaction.raw.expiration < OffsetDateTime::now_utc() {
            return Err(Error::Expired(self.transaction.raw.expiration));
        }
        let resource_state = self.estimate_transaction().await?;
        if let Some(InsufficientResource {
            missing,
            suggested_trx_topup,
            account_balance,
        }) = &resource_state.insufficient
        {
            if self.can_spend_trx_for_fee
                && missing.len() == suggested_trx_topup.len()
                && suggested_trx_topup.iter().map(|(_, trx)| *trx).sum::<Trx>()
                    <= *account_balance
            {
                return Ok(());
            }
            return Err(Error::InsufficientResources(resource_state));
        }
        Ok(())
    }
    async fn extract_permission(&self) -> Result<Permission> {
        let permission = self
            .client
            .account_permissions(self.owner)
            .await?
            .permission_by_id(
                self.transaction
                    .raw
                    .contract
                    .first()
                    .context("no contract found")?
                    .permission_id,
            )
            .context("no permission found")?;
        Ok(permission)
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
    ) -> Result<TransactionInfo>
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
            self.additional_fee += trx!(1.0 TRX);
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
        let permission = self.extract_permission().await?;
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
    ) -> Result<TransactionInfo>
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
        let transaction = protocol::Transaction::from(self.transaction.clone())
            .encode_to_vec();
        let tron_address = self.owner.as_bytes();
        let fee_bytes = self.additional_fee.to_sun().to_le_bytes().to_vec();

        let mut serialized =
            Vec::with_capacity(32 + 21 + size_of::<Trx>() + transaction.len());
        serialized.extend_from_slice(self.txid.as_ref()); // 32 bytes
        serialized.extend_from_slice(tron_address); // 21 bytes
        serialized.extend_from_slice(&fee_bytes); // 8 bytes
        serialized.push(self.can_spend_trx_for_fee as u8); // 1 byte
        serialized.extend_from_slice(&transaction); // Variable length
        serialized
    }
    pub fn try_deserialize(
        client: &'a Client<P, S>,
        data: &[u8],
    ) -> Result<Self> {
        // Minimum data length: 32 (txid) + 21 (address) + 8 (Trx) + 1 (min protobuf)
        if data.len() < 62 {
            return Err(Error::InvalidInput(format!(
                "min data length is 62, got {}",
                data.len()
            )));
        }

        let (txid_bytes, remaining) = data.split_at(32);
        let (address_bytes, remaining) = remaining.split_at(21);
        let (fee_bytes, remaining) = remaining.split_at(8);
        let (can_spend_byte, transaction_data) = remaining.split_at(1);

        let txid: Hash32 =
            txid_bytes.try_into().map_err(Error::InvalidInput)?;
        let owner = TronAddress::new(
            *<&[u8; 21]>::try_from(address_bytes)
                .map_err(|e| Error::InvalidInput(e.to_string()))?,
        )?;
        let additional_fee = i64::from_le_bytes(fee_bytes.try_into().map_err(
            |e: TryFromSliceError| Error::InvalidInput(e.to_string()),
        )?)
        .into();
        let can_spend_trx_for_fee = can_spend_byte[0] != 0;

        let transaction: domain::transaction::Transaction =
            protocol::Transaction::decode(transaction_data)?
                .try_into()
                .map_err(Error::ProtoConv)?;

        Ok(Self {
            client,
            txid,
            owner,
            transaction,
            additional_fee,
            _mode: PhantomData,
            can_spend_trx_for_fee,
        })
    }
}

async fn transaction_receipt<P, S>(
    confirmations: i32,
    client: Client<P, S>,
    txid: Hash32,
) -> std::result::Result<TransactionInfo, Error>
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
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    // If we had initial confirmation but now get an error, that's bad
                    if initial_tx_info.is_some() {
                        return Err(e);
                    }
                    // Otherwise just continue waiting
                }
            }
        }
    }

    Err(Error::TransactionTimeout)
}
