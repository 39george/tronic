use std::marker::PhantomData;

use anyhow::Context;
use prost::Message;
use time::OffsetDateTime;
use time::ext::NumericalDuration;

use crate::domain::Hash32;
use crate::domain::account::AccountResourceUsage;
use crate::domain::address::TronAddress;
use crate::domain::estimate::{InsufficientResource, Resource, ResourceState};
use crate::domain::permission::Permission;
use crate::domain::transaction::Transaction;
use crate::domain::trx::Trx;
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
            if let domain::contract::ContractType::TriggerSmartContract(
                ref contract,
            ) = contract.contract_type
            {
                let txext = self
                    .client
                    .provider
                    .trigger_constant_contract(contract.clone())
                    .await?;
                return Ok(txext.energy_used);
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
                    < *account_balance
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
        let (signature, recovery_id) =
            signer.sign_recoverable(&self.txid, ctx).await?;
        let recoverable_signature =
            domain::RecoverableSignature::new(signature, recovery_id);

        self.transaction.signature.push(recoverable_signature);

        self.validate_transaction().await?;

        self.client
            .provider
            .broadcast_transaction(self.transaction)
            .await?;

        Ok(self.txid)
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
    pub async fn sign(&mut self, signer: &S, ctx: &S::Ctx) -> Result<()> {
        let txid = &self.txid;

        let (signature, recovery_id) =
            signer.sign_recoverable(txid, ctx).await?;
        let recoverable_signature =
            domain::RecoverableSignature::new(signature, recovery_id);

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
            return Err(Error::InvalidInput(
                "address is not in permission".into(),
            ));
        }

        self.transaction.signature.push(recoverable_signature);
        Ok(())
    }
    pub async fn broadcast(self) -> Result<Hash32> {
        let txid = self.txid;

        self.validate_transaction().await?;

        self.client
            .provider
            .broadcast_transaction(self.transaction)
            .await
            .unwrap();
        Ok(txid)
    }
    /// Expiration is limited to 24 hours
    pub async fn expiration(
        &mut self,
        expiration: time::Duration,
    ) -> Result<()> {
        let timestamp = self.transaction.raw.timestamp;
        let new_expiration = timestamp.saturating_add(expiration);
        if new_expiration > timestamp.saturating_add(24.hours()) {
            return Err(Error::InvalidInput(
                "expiration is limited to 24 hours".into(),
            ));
        }
        self.transaction.raw.expiration = new_expiration;
        self.refresh_txid().await?;
        Ok(())
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
        data: Vec<u8>,
    ) -> Option<Self> {
        // Minimum data length: 32 (txid) + 21 (address) + 8 (Trx) + 1 (min protobuf)
        if data.len() < 62 {
            return None;
        }

        let (txid_bytes, remaining) = data.split_at(32);
        let (address_bytes, remaining) = remaining.split_at(21);
        let (fee_bytes, remaining) = remaining.split_at(8);
        let (can_spend_byte, transaction_data) = remaining.split_at(1);

        let txid: Hash32 = txid_bytes.try_into().ok()?;
        let owner =
            TronAddress::new(*<&[u8; 21]>::try_from(address_bytes).ok()?)
                .ok()?;
        let additional_fee =
            i64::from_le_bytes(fee_bytes.try_into().ok()?).into();
        let can_spend_trx_for_fee = can_spend_byte[0] != 0;

        let transaction: domain::transaction::Transaction =
            protocol::Transaction::decode(transaction_data).ok()?.into();

        Some(Self {
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
