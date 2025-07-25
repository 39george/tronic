use anyhow::anyhow;
use bon::Builder;
use secrecy::SecretString;

use crate::Result;
use crate::client::builder::PermissionHandler;
use crate::contracts;
use crate::contracts::token::Token;
use crate::domain;
use crate::domain::Hash32;
use crate::domain::address::TronAddress;
use crate::domain::transaction::TransactionExtention;
use crate::domain::trx::Trx;
use crate::error;
use crate::error::Error;
use crate::listener::ListenerHandle;
use crate::provider::TronProvider;
use crate::signer::PrehashSigner;

pub mod builder;

pub enum Auth {
    Bearer { name: String, secret: SecretString },
    None,
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

        let (signature, recovery_id) =
            self.client.signer.sign_recoverable(txid, &ctx).await?;
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
    pub fn trc20_balance_of<T: Token>(
        &self,
        contract: TronAddress,
    ) -> builder::Trc20BalanceOfBuilder<'_, P, S, T> {
        builder::Trc20BalanceOfBuilder {
            client: self,
            contract: contracts::trc20::Trc20Contract::new(contract),
            owner: None,
        }
    }
    pub async fn trc20_transfer<T: Token>(
        &self,
        to: TronAddress,
        contract: TronAddress,
        amount: u64,
    ) -> builder::Trc20TransferBuilder<'_, P, S, T> {
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
    pub async fn account_permissions(
        &self,
        address: TronAddress,
    ) -> Result<PermissionHandler<'_, P, S>> {
        PermissionHandler::new(self, address).await
    }
    pub async fn energy_price(&self) -> Result<Trx> {
        let chain_parameters = self.provider.chain_parameters().await?;
        let energy_price = chain_parameters
            .get("getEnergyFee")
            .ok_or(anyhow!("not found getTransactionFee"))?;
        Ok((*energy_price).into())
    }
    pub async fn bandwidth_price(&self) -> Result<Trx> {
        let chain_parameters = self.provider.chain_parameters().await?;
        let bandwidth_unit_price = chain_parameters
            .get("getTransactionFee")
            .ok_or(anyhow!("not found getTransactionFee"))?;
        Ok((*bandwidth_unit_price).into())
    }
}

impl<P, S> std::ops::Deref for Client<P, S> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        &self.provider
    }
}
