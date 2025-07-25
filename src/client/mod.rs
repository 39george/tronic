use anyhow::anyhow;
use bon::Builder;
use secrecy::SecretString;

use crate::Result;
use crate::contracts::token::Token;
use crate::domain::address::TronAddress;
use crate::domain::trx::Trx;
use crate::listener::ListenerHandle;
use crate::provider::TronProvider;
use crate::signer::PrehashSigner;

use builder::PermissionHandler;

pub mod builder;
pub mod pending;

pub enum Auth {
    Bearer { name: String, secret: SecretString },
    None,
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
    pub fn signer_address(&self) -> Option<TronAddress> {
        self.signer.address()
    }
    pub fn send_trx(&self) -> builder::TransferBuilder<'_, P, S> {
        builder::Transfer::with_client(self)
    }
    pub fn trx_balance(&self) -> builder::TrxBalanceBuilder<'_, P, S> {
        builder::TrxBalance::with_client(self)
    }
    pub fn trc20_balance_of<T: Token>(
        &self,
    ) -> builder::Trc20BalanceOfBuilder<'_, P, S, T> {
        builder::Trc20BalanceOf::with_client(self)
    }
    pub async fn trc20_transfer<T: Token>(
        &self,
    ) -> builder::Trc20TransferBuilder<'_, P, S, T> {
        builder::Trc20Transfer::with_client(self)
    }
    pub async fn listener(&self) -> ListenerHandle {
        let listener = crate::listener::Listener::new(self.to_owned());
        listener.run().await
    }
    pub async fn account_permissions(
        &self,
        address: TronAddress,
    ) -> Result<PermissionHandler<'_, P, S>>
    where
        crate::error::Error: From<S::Error>,
    {
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
