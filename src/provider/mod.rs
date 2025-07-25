use std::collections::HashMap;

use crate::Result;
use crate::contracts::AbiEncode;
use crate::domain::address::TronAddress;
use crate::domain::trx::Trx;
use crate::domain::{self, Hash32};

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
    // async fn energy_price(&self) -> Result<domain::trx::Trx>;
    // async fn bandwidth_price(&self) -> Result<domain::trx::Trx>;
    async fn get_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::Account>;
    async fn get_account_resources(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::AccountResourceUsage>;
    async fn trigger_constant_contract<A: AbiEncode + Send>(
        &self,
        owner: TronAddress,
        contract: TronAddress,
        call: A,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn get_now_block(&self) -> Result<domain::block::BlockExtention>;
    async fn account_permission_update(
        &self,
        contract: domain::contract::AccountPermissionUpdateContract,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn get_transaction_by_id(
        &self,
        txid: Hash32,
    ) -> Result<domain::transaction::Transaction>;
    async fn get_transaction_info(
        &self,
        txid: Hash32,
    ) -> Result<domain::transaction::TransactionInfo>;
    async fn chain_parameters(&self) -> Result<HashMap<String, i64>>;

    // async fn calculate_fee(&self, transaction: &Transaction) -> Result<Fee>;

    //     async fn get_contract_abi(
    //     &self,
    //     contract_address: TronAddress
    // ) -> Result<ContractAbi>;

    // async fn get_contract_info(
    //     &self,
    //     contract_address: TronAddress
    // ) -> Result<ContractInfo>;

    // async fn get_block_by_number(&self, block_num: i64) -> Result<Block>;

    // async fn get_block_by_id(&self, block_id: [u8; 32]) -> Result<Block>;

    // async fn get_token_info(&self, token_id: String) -> Result<TokenInfo>;

    // async fn get_token_list(&self) -> Result<Vec<TokenInfo>>;

    // async fn get_chain_parameters(&self) -> Result<Vec<ChainParameter>>;

    // async fn get_node_info(&self) -> Result<NodeInfo>;
}
