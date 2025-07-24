use anyhow::anyhow;
use bon::Builder;
use secrecy::SecretString;

use crate::Result;
use crate::contracts;
use crate::contracts::AbiEncode;
use crate::domain;
use crate::domain::Hash32;
use crate::domain::address::TronAddress;
use crate::domain::transaction::TransactionExtention;
use crate::domain::trx::Trx;
use crate::error;
use crate::error::Error;
use crate::listener::ListenerHandle;
use crate::signer::PrehashSigner;

pub mod builder;

pub enum Auth {
    Bearer { name: String, secret: SecretString },
    None,
}

#[async_trait::async_trait]
pub trait TronProvider {
    async fn trasnfer_contract(
        &self,
        owner: TronAddress,
        to: TronAddress,
        amount: Trx,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn trigger_smart_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn broadcast_transaction(
        &self,
        transaction: domain::transaction::Transaction,
    ) -> Result<()>;
    async fn estimate_energy(
        &self,
        contract: domain::contract::TriggerSmartContract,
    ) -> Result<i64>;
    async fn energy_price(&self) -> Result<domain::trx::Trx>;
    async fn bandwidth_price(&self) -> Result<domain::trx::Trx>;
    async fn get_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::Account>;
    async fn trigger_constant_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn get_now_block(&self) -> Result<domain::block::BlockExtention>;
}

pub struct PendingTransaction<'a, P, S> {
    client: &'a Client<P, S>,
    txext: TransactionExtention,
}

impl<'a, P, S> PendingTransaction<'a, P, S>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    error::Error: From<S::Error>,
{
    pub async fn broadcast(self, ctx: S::Ctx) -> Result<Hash32> {
        let txid = &self.txext.txid;

        let signature = self.client.signer.sign(txid, &ctx).await?;
        let recovery_id = self.client.signer.recovery_id(txid, &signature)?;
        let recoverable_signature =
            domain::RecoverableSignature::new(signature, recovery_id);

        let mut transaction = self
            .txext
            .transaction
            .ok_or(Error::Unexpected(anyhow!("no transaction in txext")))?;
        transaction.signature.push(recoverable_signature);

        self.client
            .provider
            .broadcast_transaction(transaction)
            .await?;

        Ok(txid.to_owned())
    }
}

#[derive(Builder, Clone)]
pub struct Client<P, S> {
    pub(crate) provider: P,
    signer: S,
}

impl<P, S> Client<P, S>
where
    P: TronProvider + Clone + Send + Sync + 'static,
    S: PrehashSigner + Clone + Send + Sync + 'static,
    S::Error: std::fmt::Debug,
{
    pub async fn send_trx(
        &self,
        to: TronAddress,
        amount: Trx,
    ) -> builder::TransferBuilder<'_, P, S> {
        builder::TransferBuilder {
            client: self,
            to,
            amount,
            from: None,
        }
    }
    pub fn trx_balance(&self) -> builder::TrxBalanceBuilder<'_, P, S> {
        builder::TrxBalanceBuilder {
            client: self,
            address: None,
        }
    }
    pub fn trc20_balance_of(
        &self,
        contract: TronAddress,
    ) -> builder::Trc20BalanceOfBuilder<'_, P, S> {
        builder::Trc20BalanceOfBuilder {
            client: self,
            contract: contracts::trc20::Trc20Contract::new(contract),
            owner: None,
        }
    }
    pub async fn trc20_transfer(
        &self,
        to: TronAddress,
        contract: TronAddress,
        amount: u64,
    ) -> builder::Trc20TransferBuilder<'_, P, S> {
        builder::Trc20TransferBuilder {
            client: self,
            to,
            amount,
            from: None,
            contract: contracts::trc20::Trc20Contract::new(contract),
        }
    }
    pub async fn listener(&self) -> ListenerHandle {
        let listener = crate::listener::Listener::new(self.to_owned());
        listener.run().await
    }
}
