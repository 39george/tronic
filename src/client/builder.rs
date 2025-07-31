use anyhow::anyhow;
use derivative::Derivative;
use time::OffsetDateTime;

use crate::Result;
use crate::contracts;
use crate::contracts::AbiEncode;
use crate::contracts::token::Token;
use crate::domain::Message;
use crate::domain::account::Account;
use crate::domain::account::AccountStatus;
use crate::domain::address::TronAddress;
use crate::domain::contract::AccountPermissionUpdateContract;
use crate::domain::contract::CancelAllUnfreezeV2Contract;
use crate::domain::contract::Contract;
use crate::domain::contract::DelegateResourceContract;
use crate::domain::contract::FreezeBalanceV2Contract;
use crate::domain::contract::ResourceCode;
use crate::domain::contract::TransferContract;
use crate::domain::contract::TriggerSmartContract;
use crate::domain::contract::UnDelegateResourceContract;
use crate::domain::contract::UnfreezeBalanceV2Contract;
use crate::domain::contract::WithdrawExpireUnfreezeContract;
use crate::domain::permission::Permission;
use crate::domain::permission::PermissionParams;
use crate::domain::transaction::Transaction;
use crate::domain::trx::Trx;
use crate::error::Error;
use crate::signer::PrehashSigner;
use crate::trx;

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
    S: PrehashSigner,
{
    pub async fn get(self) -> Result<Trx> {
        let trx_balance = self.build_internal();
        let address = trx_balance
            .address
            .or_else(|| {
                trx_balance.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!(
                    "missing address to check trx balance for"
                ))
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
    pub(super) can_spend_trx_for_fee: Option<bool>,
}

impl<'a, P, S, State: transfer_builder::IsComplete>
    TransferBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let transfer = self.build_internal();
        let owner = transfer
            .owner
            .or_else(|| {
                transfer.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing owner address"))
            })?;

        // Check balance
        {
            let balance =
                transfer.client.trx_balance().address(owner).get().await?;
            if transfer.amount > balance {
                return Err(Error::InsufficientBalance {
                    balance,
                    need: transfer.amount,
                });
            }
        }

        let latest_block = transfer.client.provider.get_now_block().await?;
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
        let check = transfer.client.check_account(transfer.to).await?;
        let additional_fee = if matches!(check, AccountStatus::NotExists) {
            trx!(1.0 TRX)
        } else {
            Trx::ZERO
        };
        PendingTransaction::new(
            transfer.client,
            transaction,
            owner,
            additional_fee,
            transfer.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
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
    pub(super) amount: T,
    pub(super) owner: Option<TronAddress>,
    pub(super) memo: Option<Message>,
    pub(super) call_value: Option<Trx>,
    pub(super) call_token_value: Option<Trx>,
    pub(super) token_id: Option<i64>,
    pub(super) can_spend_trx_for_fee: Option<bool>,
}

impl<'a, P, S, T, State: trc20_transfer_builder::IsComplete>
    Trc20TransferBuilder<'a, P, S, T, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
    T: Token,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let transfer = self.build_internal();
        let owner = transfer
            .owner
            .or_else(|| {
                transfer.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing owner address"))
            })?;
        let call = transfer.contract.transfer(transfer.to, transfer.amount);
        let contract_address = transfer.contract.address();

        // Check balance
        {
            let balance = Trc20BalanceOf::with_client(transfer.client)
                .contract(transfer.contract)
                .owner(owner)
                .get()
                .await?;
            if transfer.amount > balance {
                return Err(Error::InsufficientTokenBalance {
                    balance: balance.into(),
                    need: transfer.amount.into(),
                    token: T::symbol(),
                });
            }
        }

        let latest_block =
            transfer.client.provider.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::TriggerSmartContract(
                        TriggerSmartContract {
                            owner_address: owner,
                            contract_address,
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
        let check = transfer.client.check_account(transfer.to).await?;
        let additional_fee = if matches!(check, AccountStatus::NotExists) {
            trx!(0.1 TRX)
        } else {
            Trx::ZERO
        };
        PendingTransaction::new(
            transfer.client,
            transaction,
            owner,
            additional_fee,
            transfer.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
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
    S: PrehashSigner,
    T: Token,
{
    pub async fn get(self) -> Result<T> {
        let balance_of = self.build_internal();
        let owner = balance_of
            .owner
            .or_else(|| {
                balance_of.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!(
                    "missing address to check trc20 balance for"
                ))
            })?;

        let call = balance_of.contract.balance_of(owner);
        let trigger = TriggerSmartContract {
            owner_address: owner,
            contract_address: balance_of.contract.address(),
            data: call.encode().into(),
            ..Default::default()
        };
        let mut extention = balance_of
            .client
            .provider
            .trigger_constant_contract(trigger)
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
    S: PrehashSigner,
    Error: From<S::Error>,
{
    pub(super) async fn new(
        client: &'a Client<P, S>,
        owner: TronAddress,
    ) -> Result<Self> {
        let account = client.provider.get_account(owner).await?;
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

        let latest_block = self.client.provider.get_now_block().await.unwrap();
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

        PendingTransaction::new(
            self.client,
            transaction,
            self.owner,
            trx!(100.0 TRX),
            true,
        )
        .await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct FreezeBalance<'a, P, S> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) owner: Option<TronAddress>,
    pub(super) amount: Trx,
    pub(super) resource: ResourceCode,
    pub(super) can_spend_trx_for_fee: Option<bool>,
}

impl<'a, P, S, State: freeze_balance_builder::IsComplete>
    FreezeBalanceBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let freeze = self.build_internal();
        let owner = freeze
            .owner
            .or_else(|| freeze.client.signer.as_ref().and_then(|s| s.address()))
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing owner address"))
            })?;

        let latest_block =
            freeze.client.provider.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::FreezeBalanceV2Contract(
                        FreezeBalanceV2Contract {
                            owner_address: owner,
                            frozen_balance: freeze.amount,
                            resource: freeze.resource,
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            Message::default()
        );
        PendingTransaction::new(
            freeze.client,
            transaction,
            owner,
            freeze.amount,
            freeze.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct UnfreezeBalance<'a, P, S> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) owner: Option<TronAddress>,
    pub(super) amount: Trx,
    pub(super) resource: ResourceCode,
    pub(super) can_spend_trx_for_fee: Option<bool>,
}

impl<'a, P, S, State: unfreeze_balance_builder::IsComplete>
    UnfreezeBalanceBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let unfreeze = self.build_internal();
        let owner = unfreeze
            .owner
            .or_else(|| {
                unfreeze.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing owner address"))
            })?;

        let account = unfreeze.client.provider.get_account(owner).await?;
        let frozen_sum: Trx = account
            .frozen_v2
            .iter()
            .filter(|f| f.freeze_type.eq(&unfreeze.resource))
            .map(|f| f.amount)
            .sum();
        if frozen_sum < unfreeze.amount {
            return Err(Error::InsufficientFrozen {
                frozen: frozen_sum,
                trying_to_unfreeze: unfreeze.amount,
            });
        }

        let latest_block =
            unfreeze.client.provider.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::UnfreezeBalanceV2Contract(
                        UnfreezeBalanceV2Contract {
                            owner_address: owner,
                            unfreeze_balance: unfreeze.amount,
                            resource: unfreeze.resource,
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            Message::default()
        );
        PendingTransaction::new(
            unfreeze.client,
            transaction,
            owner,
            unfreeze.amount,
            unfreeze.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct CancelAllUnfreeze<'a, P, S> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) owner: Option<TronAddress>,
    pub(super) can_spend_trx_for_fee: Option<bool>,
}

impl<'a, P, S, State: cancel_all_unfreeze_builder::IsComplete>
    CancelAllUnfreezeBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let unfreeze = self.build_internal();
        let owner = unfreeze
            .owner
            .or_else(|| {
                unfreeze.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing owner address"))
            })?;

        let account = unfreeze.client.provider.get_account(owner).await?;
        if account.unfrozen_v2.is_empty() {
            return Err(Error::PreconditionFailed(
                "no unfreeze balance to cancel".into(),
            ));
        }

        let latest_block =
            unfreeze.client.provider.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::CancelAllUnfreezeV2Contract(
                        CancelAllUnfreezeV2Contract {
                            owner_address: owner,
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            Message::default()
        );
        PendingTransaction::new(
            unfreeze.client,
            transaction,
            owner,
            Trx::ZERO,
            unfreeze.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Delegate<'a, P, S> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) owner: Option<TronAddress>,
    pub(super) amount: Trx,
    pub(super) receiver: TronAddress,
    pub(super) resource: ResourceCode,
    pub(super) can_spend_trx_for_fee: Option<bool>,
    pub(super) lock_period: Option<time::Duration>,
}

impl<'a, P, S, State: delegate_builder::IsComplete>
    DelegateBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let delegate = self.build_internal();
        let owner = delegate
            .owner
            .or_else(|| {
                delegate.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing owner address"))
            })?;

        // TODO: Check has enough resources

        let latest_block =
            delegate.client.provider.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::DelegateResourceContract(
                        DelegateResourceContract {
                            owner_address: owner,
                            resource: delegate.resource,
                            balance: delegate.amount,
                            receiver_address: delegate.receiver,
                            lock_period: delegate.lock_period,
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            Message::default()
        );
        PendingTransaction::new(
            delegate.client,
            transaction,
            owner,
            delegate.amount,
            delegate.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct Undelegate<'a, P, S> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) owner: Option<TronAddress>,
    pub(super) amount: Trx,
    pub(super) receiver: TronAddress,
    pub(super) resource: ResourceCode,
    pub(super) can_spend_trx_for_fee: Option<bool>,
}

impl<'a, P, S, State: undelegate_builder::IsComplete>
    UndelegateBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let undelegate = self.build_internal();
        let owner = undelegate
            .owner
            .or_else(|| {
                undelegate.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing owner address"))
            })?;

        let total_delegated: Trx = undelegate
            .client
            .provider
            .get_delegated_resource(owner, undelegate.receiver)
            .await?
            .iter()
            .map(|r| match undelegate.resource {
                ResourceCode::Bandwidth => r.frozen_balance_for_bandwidth,
                ResourceCode::Energy => r.frozen_balance_for_energy,
                _ => Trx::ZERO,
            })
            .sum();
        if total_delegated < undelegate.amount {
            return Err(Error::PreconditionFailed(
                "not enough delegated trx to undelegate".into(),
            ));
        }

        let latest_block =
            undelegate.client.provider.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::UnDelegateResourceContract(
                        UnDelegateResourceContract {
                            owner_address: owner,
                            resource: undelegate.resource,
                            balance: undelegate.amount,
                            receiver_address: undelegate.receiver,
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            Message::default()
        );
        PendingTransaction::new(
            undelegate.client,
            transaction,
            owner,
            undelegate.amount,
            undelegate.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
    }
}

#[derive(bon::Builder)]
#[builder(start_fn = with_client)]
#[builder(finish_fn(vis = "", name = build_internal))]
pub struct WithdrawUnfreeze<'a, P, S> {
    #[builder(start_fn)]
    pub(super) client: &'a Client<P, S>,
    pub(super) owner: Option<TronAddress>,
    pub(super) resource: ResourceCode,
    pub(super) can_spend_trx_for_fee: Option<bool>,
}

impl<'a, P, S, State: withdraw_unfreeze_builder::IsComplete>
    WithdrawUnfreezeBuilder<'a, P, S, State>
where
    P: TronProvider,
    S: PrehashSigner,
    Error: From<S::Error>,
{
    pub async fn build<M>(self) -> Result<PendingTransaction<'a, P, S, M>> {
        let withdraw = self.build_internal();
        let owner = withdraw
            .owner
            .or_else(|| {
                withdraw.client.signer.as_ref().and_then(|s| s.address())
            })
            .ok_or_else(|| {
                Error::Unexpected(anyhow!("missing owner address"))
            })?;

        let account = withdraw.client.provider.get_account(owner).await?;
        let unfrozen_sum: Trx = account
            .unfrozen_v2
            .iter()
            .filter(|f| {
                f.unfreeze_type.eq(&withdraw.resource)
                    && f.unfreeze_expire_time.le(&OffsetDateTime::now_utc())
            })
            .map(|f| f.unfreeze_amount)
            .sum();
        if unfrozen_sum == Trx::ZERO {
            return Err(Error::PreconditionFailed(format!(
                "no {:?} unfrozen to withdraw",
                withdraw.resource
            )));
        }

        let latest_block =
            withdraw.client.provider.get_now_block().await.unwrap();
        let transaction = Transaction::new(
            Contract {
                contract_type:
                    crate::domain::contract::ContractType::WithdrawExpireUnfreezeContract(
                        WithdrawExpireUnfreezeContract {
                            owner_address: owner,
                        },
                    ),
                ..Default::default()
            },
            &latest_block,
            Message::default()
        );
        PendingTransaction::new(
            withdraw.client,
            transaction,
            owner,
            Trx::ZERO,
            withdraw.can_spend_trx_for_fee.unwrap_or_default(),
        )
        .await
    }
}
