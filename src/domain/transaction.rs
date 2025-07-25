use std::{collections::HashMap, mem};

use time::OffsetDateTime;

use crate::domain::Message;
use crate::domain::address::TronAddress;
use crate::domain::trx::Trx;

use super::Hash32;
use super::RecoverableSignature;
use super::RefBlockBytes;
use super::RefBlockHash;
use super::contract::Contract;

#[derive(Debug, Clone, PartialEq)]
pub struct MarketOrderDetail {
    pub maker_order_id: Vec<u8>,
    pub taker_order_id: Vec<u8>,
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
    pub ret: i32,
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

#[derive(Debug, Clone, PartialEq)]
pub struct AccountId {
    pub name: Message,
    pub address: TronAddress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Authority {
    pub account: AccountId,
    pub permission_name: Message,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawTransaction {
    pub ref_block_bytes: RefBlockBytes,
    pub ref_block_num: i64,
    pub ref_block_hash: RefBlockHash,
    pub expiration: OffsetDateTime,
    pub data: Message,
    pub contract: Vec<Contract>,
    pub scripts: Vec<u8>,
    pub timestamp: OffsetDateTime,
    pub fee_limit: Trx,
    pub auths: Vec<Authority>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub raw: Option<RawTransaction>,
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

impl Transaction {
    /// Estimates the bandwidth consumption of the transaction in bytes
    pub fn estimate_bandwidth(&self) -> usize {
        // Size of raw transaction data (approximate)
        let raw_data_size = if let Some(ref raw) = self.raw {
            mem::size_of_val(&raw.ref_block_bytes)
                + mem::size_of_val(&raw.ref_block_num)
                + mem::size_of_val(&raw.ref_block_hash)
                + mem::size_of_val(&raw.expiration.unix_timestamp())
                + mem::size_of_val(&raw.data)
                + mem::size_of_val(&raw.scripts)
                + mem::size_of_val(&raw.timestamp.unix_timestamp())
                + mem::size_of_val(&i64::from(raw.fee_limit))
        } else {
            0
        };

        // Signature size (fixed 67 bytes per signature as per TronWeb example)
        let signature_size = 67; // k256::ecdsa::Signature is 64 bytes, but Tron uses 67

        // Transaction result size (fixed 64 bytes as per examples)
        let result_size = 64;

        // Total bandwidth is sum of all components
        raw_data_size + signature_size + result_size
    }
    pub fn get_contract(&self) -> Option<Contract> {
        self.raw.as_ref().and_then(|r| r.contract.clone().pop())
    }
}

impl TransactionExtention {
    pub fn get_contract(&self) -> Option<Contract> {
        self.transaction.as_ref().and_then(|t| t.get_contract())
    }
}
