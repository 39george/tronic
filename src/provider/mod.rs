use std::collections::HashMap;
use std::time::Duration;

use crate::Result;
use crate::contracts::AbiEncode;
use crate::domain::address::TronAddress;
use crate::domain::trx::Trx;
use crate::domain::{self, Hash32};
use crate::protocol::ProtoConvError;

pub mod grpc;
#[cfg(feature = "mock-provider")]
pub mod mock;

#[derive(Clone)]
pub struct RateLimit {
    pub limit: u64,
    pub duration: Duration,
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("failed to convert TransactionInfo {txid:?}: {source}")]
pub struct TxInfoConvError {
    /// txid of the element that failed to convert
    pub txid: Hash32,
    #[source]
    pub source: ProtoConvError,
}

#[async_trait::async_trait]
pub trait TronProvider {
    async fn transfer_contract(
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
    async fn get_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::Account>;
    async fn get_account_resources(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::AccountResourceUsage>;
    async fn trigger_constant_contract(
        &self,
        contract: domain::contract::TriggerSmartContract,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn get_now_block(&self) -> Result<domain::block::BlockExtention>;
    async fn get_block_by_number(
        &self,
        block_num: i64,
    ) -> Result<domain::block::BlockExtention>;
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

    /// Receipts of *all* transactions of a block in a single request
    /// (`Wallet.GetTransactionInfoByBlockNum`).
    ///
    /// * The outer `Result` is a transport failure of the whole call.
    /// * Each element is `Ok(TransactionInfo)` or `Err(TxInfoConvError)` with
    ///   the txid of the broken element: one bad receipt does not lose the
    ///   block, and skip-or-fail stays with the caller.
    /// * An empty block yields an empty vector, not an error.
    async fn get_transaction_info_by_block_num(
        &self,
        block_num: i64,
    ) -> Result<
        Vec<
            std::result::Result<
                domain::transaction::TransactionInfo,
                TxInfoConvError,
            >,
        >,
    >;

    /// Number of the latest *solidified* block.
    ///
    /// Implementations must guarantee the returned number never exceeds
    /// `get_now_block()` on the same endpoint; on nile/mainnet the lag is
    /// ~19 blocks (2/3 of 27 SRs).
    async fn get_solid_block_number(&self) -> Result<i64>;

    async fn chain_parameters(&self) -> Result<HashMap<String, i64>>;
    async fn freeze_balance(
        &self,
        contract: domain::contract::FreezeBalanceV2Contract,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn unfreeze_balance(
        &self,
        contract: domain::contract::UnfreezeBalanceV2Contract,
    ) -> Result<domain::transaction::TransactionExtention>;
    async fn get_reward(&self, address: TronAddress) -> Result<Trx>;
    async fn get_delegated_resource(
        &self,
        from_address: TronAddress,
        to_address: TronAddress,
    ) -> Result<Vec<domain::account::DelegatedResource>>;
    async fn get_delegated_resource_account(
        &self,
        address: TronAddress,
    ) -> Result<domain::account::DelegatedResourceAccountIndex>;
    async fn get_contract(
        &self,
        address: TronAddress,
    ) -> Result<domain::contract::SmartContract>;

    // async fn calculate_fee(&self, transaction: &Transaction) -> Result<Fee>;

    //     async fn get_contract_abi(
    //     &self,
    //     contract_address: TronAddress
    // ) -> Result<ContractAbi>;

    // async fn get_contract_info(
    //     &self,
    //     contract_address: TronAddress
    // ) -> Result<ContractInfo>;

    // async fn get_block_by_id(&self, block_id: [u8; 32]) -> Result<Block>;

    // async fn get_token_info(&self, token_id: String) -> Result<TokenInfo>;

    // async fn get_token_list(&self) -> Result<Vec<TokenInfo>>;

    // async fn get_chain_parameters(&self) -> Result<Vec<ChainParameter>>;

    // async fn get_node_info(&self) -> Result<NodeInfo>;
}
