use time::OffsetDateTime;

use crate::domain::{
    IdHash, RecoverableSignature, address::TronAddress,
    transaction::Transaction,
};

use super::transaction::TransactionExtention;

#[derive(Clone, PartialEq)]
pub struct RawBlockHeader {
    pub timestamp: OffsetDateTime,
    pub tx_trie_root: Vec<u8>,
    pub parent_hash: Vec<u8>,
    /// bytes nonce = 5;
    /// bytes difficulty = 6;
    pub number: i64,
    pub witness_id: i64,
    pub witness_address: TronAddress,
    pub version: i32,
    pub account_state_root: Vec<u8>,
}

#[derive(Clone, PartialEq)]
pub struct BlockHeader {
    pub raw_data: Option<RawBlockHeader>,
    pub witness_signature: RecoverableSignature,
}

#[derive(Clone, PartialEq)]
pub struct BlockExtention {
    pub transactions: Vec<TransactionExtention>,
    pub block_header: Option<BlockHeader>,
    pub blockid: IdHash,
}

#[derive(Clone, PartialEq)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub block_header: Option<BlockHeader>,
}
