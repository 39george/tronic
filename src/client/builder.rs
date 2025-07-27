use anyhow::anyhow;
use derivative::Derivative;

use crate::Result;
use crate::contracts;
use crate::contracts::AbiEncode;
use crate::contracts::token::Token;
use crate::domain::Message;
use crate::domain::account::Account;
use crate::domain::address::TronAddress;
use crate::domain::contract::AccountPermissionUpdateContract;
use crate::domain::contract::Contract;
use crate::domain::contract::TransferContract;
use crate::domain::contract::TriggerSmartContract;
use crate::domain::permission::Permission;
use crate::domain::permission::PermissionParams;
use crate::domain::transaction::Transaction;
use crate::domain::trx::Trx;
use crate::error::Error;
use crate::signer::PrehashSigner;

use super::Client;
use super::TronProvider;
use super::pending::PendingTransaction;

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct TrxBalance<'a, P, S> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) address: Option<TronAddress>,
}

impl<'a, P, S, State: trx_balance_builder::IsComplete>
    TrxBalanceBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
{
    pub async fn get(self) -> Result<Trx> {
        let trx_balance = self.build_internal();
        let address = trx_balance
            .address
            .or_else(|| trx_balance.client.signer.address())
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing `from` address"))
            })?;

        let account = trx_balance.client.provider.get_account(address).await?;

        Ok(account.balance)
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Transfer<'a, P, S> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) to: TronAddress,
    pub(super) amount: Trx,
    pub(super) owner: Option<TronAddress>,
    pub(super) memo: Option<Message>,
}

impl<'a, P, S, State: transfer_builder::IsComplete>
    TransferBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    Error: From<S::Error>,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let transfer = self.build_internal();
        let owner = transfer
            .owner
            .or_else(|| transfer.client.signer.address())
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing `from` address"))
            })?;
        let latest_block = transfer.client.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::TransferContract(
                        TransferContract {
                            owner_address: owner,
                            to_address: transfer.to,
                            amount: transfer.amount,
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            transfer.memo.unwrap_or_default(),
        );
        PendingTransaction::new(transfer.client, transaction, owner).await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Trc20Transfer<'a, P, S, T> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) contract: contracts::trc20::Trc20Contract<T>,
    pub(super) to: TronAddress,
    pub(super) amount: u64,
    pub(super) owner: Option<TronAddress>,
    pub(super) memo: Option<Message>,
    pub(super) call_value: Option<Trx>,
    pub(super) call_token_value: Option<Trx>,
    pub(super) token_id: Option<i64>,
}

impl<'a, P, S, T, State: trc20_transfer_builder::IsComplete>
    Trc20TransferBuilder<'a, P, S, T, State>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    Error: From<S::Error>,
    T: Token + Send,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let transfer = self.build_internal();
        let owner = transfer
            .owner
            .or_else(|| transfer.client.signer.address())
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing `from` address"))
            })?;
        let latest_block = transfer.client.get_now_block().await.unwrap();
        let call = transfer.contract.transfer(transfer.to, transfer.amount);
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::TriggerSmartContract(
                        TriggerSmartContract {
                            owner_address: owner,
                            contract_address: transfer.contract.address(),
                            call_value: transfer.call_value.unwrap_or_default(),
                            data: call.encode().into(),
                            call_token_value: transfer
                                .call_token_value
                                .unwrap_or_default(),
                            token_id: transfer.token_id.unwrap_or_default(),
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            transfer.memo.unwrap_or_default(),
        );

        PendingTransaction::new(transfer.client, transaction, owner).await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Trc20BalanceOf<'a, P, S, T> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) contract: contracts::trc20::Trc20Contract<T>,
    pub(super) owner: Option<TronAddress>,
}

impl<'a, P, S, T, State: trc20_balance_of_builder::IsComplete>
    Trc20BalanceOfBuilder<'a, P, S, T, State>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    T: Token,
{
    pub async fn get(self) -> Result<T> {
        let balance_of = self.build_internal();
        let owner = balance_of
            .owner
            .or_else(|| balance_of.client.signer.address())
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing `owner` address"))
            })?;

        let call = balance_of.contract.balance_of(owner);
        let mut extention = balance_of
            .client
            .provider
            .trigger_constant_contract(
                owner,
                balance_of.contract.address(),
                call,
            )
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

        Ok(T::from(balance))
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct PermissionHandler<'a, P, S> {
    #[derivative(Debug = "ignore")]
    pub(super) client: &'a Client<P, S>,
    pub(super) account: Account,
    pub(super) permission_update: AccountPermissionUpdateContract,
    pub(super) owner: TronAddress,
}

impl<'a, P, S> PermissionHandler<'a, P, S>
where
    P: TronProvider,
    S: PrehashSigner + Clone,
    Error: From<S::Error>,
{
    pub(super) async fn new(
        client: &'a Client<P, S>,
        owner: TronAddress,
    ) -> Result<Self> {
        let account = client.get_account(owner).await?;
        Ok(PermissionHandler {
            client,
            permission_update: AccountPermissionUpdateContract {
                owner_address: owner,
                owner: Some(account.owner_permission.clone()),
                witness: account.witness_permission.clone(),
                actives: account.active_permission.clone(),
            },
            account,
            owner,
        })
    }
    pub fn owner(&self) -> Permission {
        self.account.owner_permission.clone()
    }
    pub fn witness(&self) -> Option<Permission> {
        self.account.witness_permission.clone()
    }
    pub fn actives(&self) -> Vec<Permission> {
        self.account.active_permission.clone()
    }
    pub fn permission_by_id(&self, id: i32) -> Option<Permission> {
        match id {
            0 => Some(self.owner()),
            1 => self.witness(),
            id => {
                let active_idx = (id - 2) as usize;
                self.actives().get(active_idx).cloned()
            }
        }
    }
    pub fn set_owner(&mut self, p: PermissionParams) -> Result<()> {
        let permission = Permission::owner().params(p).call();
        permission.can_meet_threshold()?;
        self.permission_update.owner = Some(permission);
        Ok(())
    }
    pub fn set_witness(&mut self, p: PermissionParams) -> Result<()> {
        let permission = Permission::witness().params(p).call();
        permission.can_meet_threshold()?;
        self.permission_update.witness = Some(permission);
        Ok(())
    }
    pub fn set_actives(&mut self, p: Vec<PermissionParams>) -> Result<()> {
        let permission = Permission::actives().params(p).call();
        permission.iter().try_for_each(|p| p.can_meet_threshold())?;
        self.permission_update.actives = permission;
        Ok(())
    }
    pub async fn update_permission<M>(
        self,
    ) -> Result<PendingTransaction<'a, P, S, M>> {
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
                "no permission changes detected".into(),
            ));
        }

        // Validate required fields (TRON rules)
        if self.permission_update.owner.is_none() {
            return Err(Error::InvalidInput(
                "owner permission must be specified".into(),
            ));
        }

        if self.permission_update.actives.is_empty() {
            return Err(Error::InvalidInput(
                "at least one active permission must be specified".into(),
            ));
        }

        let latest_block = self.client.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::AccountPermissionUpdateContract(
                    self.permission_update
                    ),
                ..Default::default()
            },
            &latest_block,
            Message::default(),
        );

        PendingTransaction::new(self.client, transaction, self.owner).await
    }
}
