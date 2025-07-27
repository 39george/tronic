use std::marker::PhantomData;

use anyhow::Context;
use prost::Message;
use time::OffsetDateTime;
use time::ext::NumericalDuration;

use crate::domain;
use crate::domain::Hash32;
use crate::domain::address::TronAddress;
use crate::domain::permission::Permission;
use crate::domain::transaction::Transaction;
use crate::error;
use crate::error::Error;
use crate::provider::TronProvider;
use crate::signer::PrehashSigner;
use crate::utility::generate_txid;
use crate::{Result, protocol, utility};

use super::Client;

pub struct AutoSigning;
pub struct ManualSigning;

pub struct PendingTransaction<'a, P, S, M = AutoSigning> {
    pub(super) client: &'a Client<P, S>,
    pub(super) transaction: Transaction,
    pub(super) txid: Hash32,
    pub(super) _mode: PhantomData<M>,
    pub(super) owner: TronAddress,
}

impl<'a, P, S, M> PendingTransaction<'a, P, S, M>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    error::Error: From<S::Error>,
{
    pub async fn new(
        client: &'a Client<P, S>,
        transaction: Transaction,
        owner: TronAddress,
    ) -> Result<Self> {
        let mut pending_transaction = Self {
            client,
            transaction,
            txid: Default::default(),
            _mode: PhantomData,
            owner,
        };
        pending_transaction.refresh_txid().await?;
        Ok(pending_transaction)
    }
    async fn refresh_txid(&mut self) -> Result<()> {
        if !self.transaction.signature.is_empty() {
            return Err(Error::PreconditionFailed(
                "can't update txid for signed transaction".into(),
            ));
        }
        let latest_block = self.client.get_now_block().await?;
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
            .get_account(self.owner)
            .await?
            .permission_by_id(permission_id)
            .context("no permission found")?
            .required_signatures()
            .context("insufficient keys for threshold")?;
        let txlen = protocol::transaction::Raw::from(raw).encode_to_vec().len();
        Ok(utility::estimate_bandwidth(txlen as i64, signature_count))
    }
}

impl<'a, P, S> PendingTransaction<'a, P, S, AutoSigning>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
    error::Error: From<S::Error>,
{
    pub async fn broadcast(mut self, ctx: &S::Ctx) -> Result<Hash32> {
        let (signature, recovery_id) =
            self.client.signer.sign_recoverable(&self.txid, ctx).await?;
        let recoverable_signature =
            domain::RecoverableSignature::new(signature, recovery_id);

        self.transaction.signature.push(recoverable_signature);

        self.client
            .provider
            .broadcast_transaction(self.transaction)
            .await?;

        Ok(self.txid)
    }
}

impl<'a, P, S> PendingTransaction<'a, P, S, ManualSigning>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
    error::Error: From<S::Error>,
{
    pub async fn set_permission(&mut self, id: i32) -> Result<()> {
        let _ = self
            .client
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
        // SETUP FEELIMIT, energy handle
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
        let mut txid = Vec::<u8>::from(self.txid);
        txid.extend_from_slice(tron_address);
        txid.extend_from_slice(&transaction);
        txid
    }
    pub fn try_deserialize(
        client: &'a Client<P, S>,
        data: Vec<u8>,
    ) -> Option<Self> {
        // Minimum data length: 32 (txid) + 21 (TronAddress) + 1 (minimal protobuf message)
        if data.len() < 54 {
            return None;
        }

        let (txid_bytes, remaining) = data.split_at(32);
        let (address_bytes, transaction_data) = remaining.split_at(21);

        let txid: Hash32 = txid_bytes.try_into().ok()?;
        let owner =
            TronAddress::new(*<&[u8; 21]>::try_from(address_bytes).ok()?)
                .ok()?;

        let transaction: domain::transaction::Transaction =
            protocol::Transaction::decode(transaction_data).ok()?.into();

        Some(Self {
            client,
            txid,
            owner,
            transaction,
            _mode: PhantomData,
        })
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
