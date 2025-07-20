use anyhow::anyhow;
use bon::Builder;
use secrecy::SecretString;

use crate::Result;
use crate::contracts;
use crate::domain;
use crate::domain::address::TronAddress;
use crate::domain::transaction::TransactionExtention;
use crate::domain::transaction::TxId;
use crate::domain::trx::Trx;
use crate::error;
use crate::error::Error;
use crate::signer::PrehashSigner;
use crate::trx;

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
    async fn trigger_smart_contract<C: alloy_sol_types::SolCall + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: C,
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
    pub async fn broadcast(self, ctx: S::Ctx) -> Result<TxId> {
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

        Ok(txid.clone().into())
    }
}

#[derive(Builder)]
pub struct Client<P, S> {
    provider: P,
    signer: S,
}

impl<P, S> Client<P, S>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    S::Error: std::fmt::Debug,
{
    pub async fn send_trx(
        &self,
        from: TronAddress,
        to: TronAddress,
        amount: Trx,
    ) -> Result<PendingTransaction<P, S>> {
        let extention =
            self.provider.trasnfer_contract(from, to, amount).await?;
        Ok(PendingTransaction {
            client: self,
            txext: extention,
        })
    }
    pub async fn trx_balance(&self, _address: TronAddress) -> Result<Trx> {
        Ok(trx! {1 TRX})
    }
    pub async fn trc20_balance_of(
        &self,
        address: TronAddress,
        contract_address: TronAddress,
    ) -> Result<PendingTransaction<P, S>> {
        let call = contracts::trc20_balance_of(address);
        let extention = self
            .provider
            .trigger_smart_contract(address, contract_address, call)
            .await?;
        Ok(PendingTransaction {
            client: self,
            txext: extention,
        })
    }
    pub async fn trc20_transfer(
        &self,
        address: TronAddress,
        contract_address: TronAddress,
        amount: u64,
    ) -> Result<PendingTransaction<P, S>> {
        let call = contracts::trc20_transfer(address, amount);
        let extention = self
            .provider
            .trigger_smart_contract(address, contract_address, call)
            .await?;
        Ok(PendingTransaction {
            client: self,
            txext: extention,
        })
    }
}
