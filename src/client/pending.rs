use std::marker::PhantomData;

use anyhow::Context;
use prost::Message;

use crate::domain;
use crate::domain::Hash32;
use crate::domain::address::TronAddress;
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

        self.transaction.signature.push(recoverable_signature);
        Ok(())
    }
    pub async fn broadcast(self) -> Result<Hash32> {
        self.client
            .provider
            .broadcast_transaction(self.transaction)
            .await
            .unwrap();
        Ok(self.txid)
    }
}
