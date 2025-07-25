use anyhow::{Context, anyhow};
use bon::Builder;
use prost::Message;
use secrecy::SecretString;

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
use crate::{Result, protocol, utility};

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
    pub async fn estimate_bandwidth(&self) -> Result<i64> {
        let raw = self
            .txext
            .transaction
            .as_ref()
            .context("no transaction found")?
            .raw
            .as_ref()
            .context("transaction raw part is empty")?
            .clone();
        let contract = raw.contract.first().context("no contract")?;
        let permission_id = contract.permission_id;
        let signature_count = self
            .client
            .get_account(self.client.signer.address().ok_or(
                Error::InvalidInput(
                    "no signer to check permissions for".into(),
                ),
            )?)
            .await?
            .permission_by_id(permission_id)
            .context("no permission found")?
            .required_signatures()
            .context("insufficient keys for threshold")?;
        let txlen = protocol::transaction::Raw::from(raw).encode_to_vec().len();
        Ok(utility::estimate_bandwidth(txlen as i64, signature_count))
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
