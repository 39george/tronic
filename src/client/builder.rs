use anyhow::anyhow;
use derivative::Derivative;

use crate::Result;
use crate::contracts;
use crate::contracts::token::Token;
use crate::domain::account::Account;
use crate::domain::address::TronAddress;
use crate::domain::contract::AccountPermissionUpdateContract;
use crate::domain::permission::Permission;
use crate::domain::trx::Trx;
use crate::error::Error;
use crate::signer::PrehashSigner;

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

        Ok(account.balance)
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

pub struct Trc20TransferBuilder<'a, P, S, T> {
    pub(super) client: &'a Client<P, S>,
    pub(super) contract: contracts::trc20::Trc20Contract<T>,
    pub(super) to: TronAddress,
    pub(super) amount: u64,
    pub(super) from: Option<TronAddress>,
}

impl<'a, P, S, T> Trc20TransferBuilder<'a, P, S, T>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    Error: From<S::Error>,
    T: Token + Send,
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

        let call = self.contract.transfer(self.to, self.amount);
        let extention = self
            .client
            .provider
            .trigger_smart_contract(from, self.contract.address(), call)
            .await?;
        Ok(PendingTransaction {
            client: self.client,
            txext: extention,
        })
    }
}

pub struct Trc20BalanceOfBuilder<'a, P, S, T> {
    pub(super) client: &'a Client<P, S>,
    pub(super) contract: contracts::trc20::Trc20Contract<T>,
    pub(super) owner: Option<TronAddress>,
}

impl<'a, P, S, T> Trc20BalanceOfBuilder<'a, P, S, T>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    T: Token,
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

        let call = self.contract.balance_of(owner);
        let mut extention = self
            .client
            .provider
            .trigger_constant_contract(owner, self.contract.address(), call)
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

#[derive(Derivative)]
#[derivative(Debug)]
pub struct PermissionHandler<'a, P, S> {
    #[derivative(Debug = "ignore")]
    pub(super) client: &'a Client<P, S>,
    pub(super) account: Account,
    pub(super) permission_update: AccountPermissionUpdateContract,
}

impl<'a, P, S> PermissionHandler<'a, P, S>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
{
    pub(super) async fn new(
        client: &'a Client<P, S>,
        owner_address: TronAddress,
    ) -> Result<Self> {
        let account = client.get_account(owner_address).await?;
        Ok(PermissionHandler {
            client,
            account,
            permission_update: AccountPermissionUpdateContract {
                owner_address,
                owner: None,
                witness: None,
                actives: Vec::new(),
            },
        })
    }
    pub fn owner(&self) -> &Permission {
        &self.account.owner_permission
    }
    pub fn witness(&self) -> Option<&Permission> {
        self.account.witness_permission.as_ref()
    }
    pub fn actives(&self) -> &[Permission] {
        &self.account.active_permission
    }
    pub fn set_owner(&mut self, p: Permission) {
        self.permission_update.owner = Some(p);
    }
    pub fn set_witness(&mut self, p: Permission) {
        self.permission_update.witness = Some(p);
    }
    pub fn set_actives(&mut self, p: Vec<Permission>) {
        self.permission_update.actives = p;
    }
    pub async fn update_permission(
        self,
    ) -> Result<PendingTransaction<'a, P, S>> {
        // Validate that at least one permission is being modified
        let has_changes = {
            let current_owner = self.account.owner_permission.clone();
            let current_witness = self.account.witness_permission.clone();
            let current_actives = self.account.active_permission.clone();

            // Check if any permission differs from current state
            (self.permission_update.owner.as_ref() != Some(&current_owner))
                || (self.permission_update.witness != current_witness)
                || (self.permission_update.actives != current_actives)
        };

        if !has_changes {
            return Err(Error::InvalidInput(
                "No permission changes detected".into(),
            ));
        }

        // Validate required fields (TRON rules)
        if self.permission_update.owner.is_none() {
            return Err(Error::InvalidInput(
                "Owner permission must be specified".into(),
            ));
        }

        if self.permission_update.actives.is_empty() {
            return Err(Error::InvalidInput(
                "At least one active permission must be specified".into(),
            ));
        }

        let txext = self
            .client
            .account_permission_update(self.permission_update)
            .await?;
        Ok(PendingTransaction {
            client: self.client,
            txext,
        })
    }
}
