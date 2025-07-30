use std::time::Duration;

use anyhow::anyhow;
use secrecy::SecretString;

use crate::Result;
use crate::contracts::token::Token;
use crate::domain::account::AccountStatus;
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

#[derive(bon::Builder, Clone)]
pub struct Client<P, S> {
    pub(crate) provider: P,
    signer: Option<S>,
}

impl<P, S> Client<P, S>
where
    P: TronProvider,
    S: PrehashSigner,
{
    pub fn provider(&self) -> &P {
        &self.provider
    }
    pub fn signer_address(&self) -> Option<TronAddress> {
        self.signer.as_ref().and_then(|s| s.address())
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
    pub fn trc20_transfer<T: Token>(
        &self,
    ) -> builder::Trc20TransferBuilder<'_, P, S, T> {
        builder::Trc20Transfer::with_client(self)
    }
    pub fn freeze_balance(&self) -> builder::FreezeBalanceBuilder<'_, P, S> {
        builder::FreezeBalance::with_client(self)
    }
    pub fn unfreeze_balance(
        &self,
    ) -> builder::UnfreezeBalanceBuilder<'_, P, S> {
        builder::UnfreezeBalance::with_client(self)
    }
    pub fn cancel_all_unfreeze(
        &self,
    ) -> builder::CancelAllUnfreezeBuilder<'_, P, S> {
        builder::CancelAllUnfreeze::with_client(self)
    }
    pub async fn listener(
        &self,
        block_poll_interval: Duration,
    ) -> ListenerHandle
    where
        P: Clone + Send + Sync + 'static,
        S: Clone + Send + Sync + 'static,
        S::Error: std::fmt::Debug,
    {
        let listener = crate::listener::Listener::new(
            self.to_owned(),
            block_poll_interval,
        );
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
    pub async fn check_account(
        &self,
        address: TronAddress,
    ) -> Result<AccountStatus> {
        let account = self.provider.get_account(address).await?;
        Ok(account.status())
    }
}
