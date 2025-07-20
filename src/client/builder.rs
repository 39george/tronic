use anyhow::anyhow;

use crate::Result;
use crate::contracts;
use crate::domain::address::TronAddress;
use crate::domain::trx::Trx;
use crate::error::Error;
use crate::signer::PrehashSigner;
use crate::trx;

use super::Client;
use super::PendingTransaction;
use super::TronProvider;

pub struct TrxBalanceBuilder<'a, P, S> {
    pub(super) client: &'a Client<P, S>,
    pub(super) address: Option<TronAddress>,
}

impl<'a, P, S> TrxBalanceBuilder<'a, P, S>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
{
    pub fn from(mut self, address: TronAddress) -> Self {
        self.address = Some(address);
        self
    }

    pub async fn get(self) -> Result<Trx> {
        let address = self
            .address
            .or_else(|| self.client.signer.address())
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing `from` address"))
            })?;

        let account = self.client.provider.get_account(address).await?;

        Ok(account.balance.into())
    }
}

pub struct TransferBuilder<'a, P, S> {
    pub(super) client: &'a Client<P, S>,
    pub(super) to: TronAddress,
    pub(super) amount: Trx,
    pub(super) from: Option<TronAddress>,
}

impl<'a, P, S> TransferBuilder<'a, P, S>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    Error: From<S::Error>,
{
    pub fn from(mut self, address: TronAddress) -> Self {
        self.from = Some(address);
        self
    }

    pub async fn build(self) -> Result<PendingTransaction<'a, P, S>> {
        let from = self
            .from
            .or_else(|| self.client.signer.address())
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing `from` address"))
            })?;

        let extention = self
            .client
            .provider
            .trasnfer_contract(from, self.to, self.amount)
            .await?;
        Ok(PendingTransaction {
            client: self.client,
            txext: extention,
        })
    }
}

pub struct Trc20TransferBuilder<'a, P, S> {
    pub(super) client: &'a Client<P, S>,
    pub(super) contract: TronAddress,
    pub(super) to: TronAddress,
    pub(super) amount: u64,
    pub(super) from: Option<TronAddress>,
}

impl<'a, P, S> Trc20TransferBuilder<'a, P, S>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    Error: From<S::Error>,
{
    pub fn from(mut self, address: TronAddress) -> Self {
        self.from = Some(address);
        self
    }

    pub async fn build(self) -> Result<PendingTransaction<'a, P, S>> {
        let from = self
            .from
            .or_else(|| self.client.signer.address())
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing `from` address"))
            })?;

        let call = contracts::trc20_transfer(self.to, self.amount);
        let extention = self
            .client
            .provider
            .trigger_smart_contract(from, self.contract, call)
            .await?;
        Ok(PendingTransaction {
            client: self.client,
            txext: extention,
        })
    }
}

pub struct Trc20BalanceOfBuilder<'a, P, S> {
    pub(super) client: &'a Client<P, S>,
    pub(super) contract: TronAddress,
    pub(super) owner: Option<TronAddress>,
}

impl<'a, P, S> Trc20BalanceOfBuilder<'a, P, S>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
{
    pub fn owner(mut self, address: TronAddress) -> Self {
        self.owner = Some(address);
        self
    }

    pub async fn get(self) -> Result<u64> {
        let owner = self
            .owner
            .or_else(|| self.client.signer.address())
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing `owner` address"))
            })?;

        let call = contracts::trc20_balance_of(owner);
        let mut extention = self
            .client
            .provider
            .trigger_constant_contract(owner, self.contract, call)
            .await?;
        let balance = if let Some(result) = extention.constant_result.pop() {
            if result.len() == 32 {
                let balance_bytes: [u8; 32] = result.try_into().unwrap(); // We sure in length
                alloy_primitives::U256::from_be_bytes(balance_bytes)
            } else {
                return Err(anyhow!("unexpected constant result length").into());
            }
        } else {
            return Err(anyhow::anyhow!("no constant result returned",).into());
        };

        Ok(balance.to())
    }
}
