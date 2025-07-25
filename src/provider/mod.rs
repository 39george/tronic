use crate::Result;
use crate::contracts::AbiEncode;
use crate::domain;
use crate::domain::address::TronAddress;
use crate::domain::trx::Trx;

pub mod grpc;

#[async_trait::async_trait]
pub trait TronProvider {
    async fn trasnfer_contract(
        &self,
        owner: TronAddress,
        to: TronAddress,
        amount: Trx,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn trigger_smart_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
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
    async fn get_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::Account>;
    async fn trigger_constant_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn get_now_block(&self) -> Result<domain::block::BlockExtention>;
}
