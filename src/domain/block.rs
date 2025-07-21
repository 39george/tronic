use time::OffsetDateTime;

use crate::domain::{
    Hash32, RecoverableSignature, address::TronAddress,
    transaction::Transaction,
};

use super::transaction::TransactionExtention;

#[derive(Debug, Clone, PartialEq)]
pub struct RawBlockHeader {
    pub timestamp: OffsetDateTime,
    pub tx_trie_root: Hash32,
    pub parent_hash: Hash32,
    /// bytes nonce = 5;
    /// bytes difficulty = 6;
    pub number: i64,
    pub witness_id: i64,
    pub witness_address: TronAddress,
    pub version: i32,
    pub account_state_root: Hash32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockHeader {
    pub raw_data: Option<RawBlockHeader>,
    pub witness_signature: RecoverableSignature,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockExtention {
    pub transactions: Vec<TransactionExtention>,
    pub block_header: Option<BlockHeader>,
    pub blockid: Hash32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub block_header: Option<BlockHeader>,
}
