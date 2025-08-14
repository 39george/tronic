use std::collections::HashMap;

use derivative::Derivative;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;
use time::ext::NumericalDuration;

use crate::domain::HexMessage;
use crate::domain::Message;
use crate::domain::address::TronAddress;
use crate::domain::block::BlockExtention;
use crate::domain::trx::Trx;

use super::Hash32;
use super::RecoverableSignature;
use super::RefBlockBytes;
use super::RefBlockHash;
use super::contract::Contract;

#[derive(Debug, Clone, PartialEq)]
pub struct MarketOrderDetail {
    pub maker_order_id: Hash32,
    pub taker_order_id: Hash32,
    pub fill_sell_quantity: i64,
    pub fill_buy_quantity: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContractResult {
    Default = 0,
    Success = 1,
    Revert = 2,
    BadJumpDestination = 3,
    OutOfMemory = 4,
    PrecompiledContract = 5,
    StackTooSmall = 6,
    StackTooLarge = 7,
    IllegalOperation = 8,
    StackOverflow = 9,
    OutOfEnergy = 10,
    OutOfTime = 11,
    JvmStackOverFlow = 12,
    Unknown = 13,
    TransferFailed = 14,
    InvalidCode = 15,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionResult {
    pub fee: i64,
    pub ret: TxCode,
    pub contract_ret: ContractResult,
    pub asset_issue_id: String,
    pub withdraw_amount: i64,
    pub unfreeze_amount: i64,
    pub exchange_received_amount: i64,
    pub exchange_inject_another_amount: i64,
    pub exchange_withdraw_another_amount: i64,
    pub exchange_id: i64,
    pub shielded_transaction_fee: i64,
    pub order_id: Vec<u8>,
    pub order_details: Vec<MarketOrderDetail>,
    pub withdraw_expire_amount: i64,
    pub cancel_unfreeze_v2_amount: HashMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountId {
    pub name: Message,
    pub address: TronAddress,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authority {
    pub account: AccountId,
    pub permission_name: Message,
}

#[derive(Derivative, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derivative(Default)]
pub struct RawTransaction {
    pub ref_block_bytes: RefBlockBytes,
    pub ref_block_num: i64,
    pub ref_block_hash: RefBlockHash,
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub expiration: OffsetDateTime,
    pub data: Message,
    pub contract: Vec<Contract>,
    pub scripts: Vec<u8>,
    #[derivative(Default(value = "OffsetDateTime::UNIX_EPOCH"))]
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub timestamp: OffsetDateTime,
    pub fee_limit: Trx,
    pub auths: Vec<Authority>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub raw: RawTransaction,
    pub signature: Vec<RecoverableSignature>,
    pub result: Vec<TransactionResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionExtention {
    pub transaction: Option<Transaction>,
    pub txid: Hash32,
    pub constant_result: Vec<Vec<u8>>,
    pub energy_used: i64,
    pub energy_penalty: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Log {
    pub address: Vec<u8>,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResourceReceipt {
    pub energy_usage: i64,
    pub energy_fee: i64,
    pub origin_energy_usage: i64,
    pub energy_usage_total: i64,
    pub net_usage: i64,
    pub net_fee: Trx,
    pub result: ContractResult,
    pub energy_penalty_total: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallValueInfo {
    /// trx (TBD: or token) value
    pub call_value: i64,
    /// TBD: tokenName, trx should be empty
    pub token_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InternalTransaction {
    /// internalTransaction identity, the root InternalTransaction hash
    /// should equals to root transaction id.
    pub hash: Hash32,
    /// the one send trx (TBD: or token) via function
    pub caller_address: TronAddress,
    /// the one recieve trx (TBD: or token) via function
    pub transfer_to_address: TronAddress,
    pub call_value_info: Vec<CallValueInfo>,
    pub note: Message,
    pub rejected: bool,
    pub extra: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TxCode {
    Sucess = 0,
    Failed = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionInfo {
    pub id: Hash32,
    pub fee: Trx,
    pub block_number: i64,
    pub block_time_stamp: OffsetDateTime,
    pub contract_result: Vec<HexMessage>,
    pub contract_address: TronAddress,
    pub receipt: Option<ResourceReceipt>,
    pub log: Vec<Log>,
    pub result: TxCode,
    pub res_message: Message,
    pub asset_issue_id: String,
    pub withdraw_amount: Trx,
    pub unfreeze_amount: Trx,
    pub internal_transactions: Vec<InternalTransaction>,
    pub exchange_received_amount: i64,
    pub exchange_inject_another_amount: i64,
    pub exchange_withdraw_another_amount: i64,
    pub exchange_id: i64,
    pub shielded_transaction_fee: Trx,
    pub order_id: HexMessage,
    pub order_details: Vec<MarketOrderDetail>,
    pub packing_fee: Trx,
    pub withdraw_expire_amount: i64,
    pub cancel_unfreeze_v2_amount: HashMap<String, Trx>,
}

impl Transaction {
    pub fn get_contract(&self) -> Option<Contract> {
        self.raw.contract.last().cloned()
    }
    pub fn new(
        contract: Contract,
        latest_block: &BlockExtention,
        memo: Message,
    ) -> Self {
        let mut transaction = Transaction::default();
        transaction.raw.timestamp = OffsetDateTime::now_utc();
        transaction.raw.data = memo;
        transaction.raw.contract.push(contract);
        latest_block.fill_header_info_in_transaction(&mut transaction);
        // Setup default expiration
        transaction.raw.expiration =
            transaction.raw.timestamp.saturating_add(60.seconds());
        transaction
    }
}

impl TransactionExtention {
    pub fn get_contract(&self) -> Option<Contract> {
        self.transaction.as_ref().and_then(|t| t.get_contract())
    }
}

impl ContractResult {
    /// Returns None if ok, and Some(BadResult) if not ok
    pub fn is_err(&self) -> Option<ContractResult> {
        match self {
            ContractResult::Success => None,
            err => Some(err.clone()),
        }
    }
}
