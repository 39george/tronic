use std::{collections::HashMap, mem};

use super::IdHash;
use super::RecoverableSignature;

use super::contract::Contract;

#[derive(Debug, Clone, PartialEq)]
pub struct UnknownType;

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionResult {
    pub fee: i64,
    pub ret: i32,
    pub contract_ret: i32,
    pub asset_issue_id: String,
    pub withdraw_amount: i64,
    pub unfreeze_amount: i64,
    pub exchange_received_amount: i64,
    pub exchange_inject_another_amount: i64,
    pub exchange_withdraw_another_amount: i64,
    pub exchange_id: i64,
    pub shielded_transaction_fee: i64,
    pub order_id: Vec<u8>,
    pub order_details: Vec<UnknownType>,
    pub withdraw_expire_amount: i64,
    pub cancel_unfreeze_v2_amount: HashMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawTransaction {
    pub ref_block_bytes: Vec<u8>,
    pub ref_block_num: i64,
    pub ref_block_hash: Vec<u8>,
    pub expiration: i64,
    pub data: Vec<u8>,
    pub contract: Option<Contract>,
    pub scripts: Vec<u8>,
    pub timestamp: i64,
    pub fee_limit: i64,
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
    pub txid: IdHash,
    pub constant_result: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
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
                + mem::size_of_val(&raw.expiration)
                + mem::size_of_val(&raw.data)
                + mem::size_of_val(&raw.scripts)
                + mem::size_of_val(&raw.timestamp)
                + mem::size_of_val(&raw.fee_limit)
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
}
