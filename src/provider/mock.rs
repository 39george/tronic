use std::collections::HashMap;

use eyre::eyre;

use crate::Result;
use crate::contracts::AbiEncode;
use crate::domain::address::TronAddress;
use crate::domain::trx::{self, Trx};
use crate::domain::{self, Hash32};
use crate::error::Error;

#[derive(Clone, Copy, Debug)]
pub struct MockProvider;

impl MockProvider {
    pub async fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl crate::provider::TronProvider for MockProvider {
    async fn trasnfer_contract(
        &self,
        _: domain::address::TronAddress,
        _: domain::address::TronAddress,
        _: trx::Trx,
    ) -> Result<domain::transaction::TransactionExtention> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn trigger_smart_contract<A: AbiEncode + Send>(
        &self,
        _: TronAddress,
        _: TronAddress,
        _: A,
    ) -> Result<domain::transaction::TransactionExtention> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn broadcast_transaction(
        &self,
        _: domain::transaction::Transaction,
    ) -> Result<()> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn estimate_energy(
        &self,
        _: domain::contract::TriggerSmartContract,
    ) -> Result<i64> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_account(
        &self,
        _: TronAddress,
    ) -> Result<domain::account::Account> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_account_resources(
        &self,
        _: TronAddress,
    ) -> Result<domain::account::AccountResourceUsage> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn trigger_constant_contract(
        &self,
        _: domain::contract::TriggerSmartContract,
    ) -> Result<domain::transaction::TransactionExtention> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_now_block(&self) -> Result<domain::block::BlockExtention> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn account_permission_update(
        &self,
        _: domain::contract::AccountPermissionUpdateContract,
    ) -> Result<domain::transaction::TransactionExtention> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_transaction_by_id(
        &self,
        _: Hash32,
    ) -> Result<domain::transaction::Transaction> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_transaction_info(
        &self,
        _: Hash32,
    ) -> Result<domain::transaction::TransactionInfo> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn chain_parameters(&self) -> Result<HashMap<String, i64>> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn freeze_balance(
        &self,
        _: domain::contract::FreezeBalanceV2Contract,
    ) -> Result<domain::transaction::TransactionExtention> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn unfreeze_balance(
        &self,
        _: domain::contract::UnfreezeBalanceV2Contract,
    ) -> Result<domain::transaction::TransactionExtention> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_reward(&self, _address: TronAddress) -> Result<Trx> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_delegated_resource(
        &self,
        _: TronAddress,
        _: TronAddress,
    ) -> Result<Vec<domain::account::DelegatedResource>> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_delegated_resource_account(
        &self,
        _: TronAddress,
    ) -> Result<domain::account::DelegatedResourceAccountIndex> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
    async fn get_block_by_number(
        &self,
        _block_num: i64,
    ) -> Result<domain::block::BlockExtention> {
        Err(Error::Unexpected(eyre!("mock provider")))
    }
}
