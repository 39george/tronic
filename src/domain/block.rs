use time::OffsetDateTime;

use crate::domain::{
    address::TronAddress, transaction::Transaction, Hash32, RecoverableSignature, RefBlockBytes,
    RefBlockHash,
};

use super::transaction::TransactionExtension;

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
    pub raw_data: RawBlockHeader,
    /// Legacy ECDSA signature.
    pub witness_signature: Option<RecoverableSignature>,
    /// Post-quantum signatures.
    pub pq_auth_sig: Option<super::PqAuthSig>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockExtension {
    pub transactions: Vec<TransactionExtension>,
    pub block_header: BlockHeader,
    pub blockid: Hash32,
}

impl BlockExtension {
    pub(crate) fn calculate_ref_block_bytes(&self) -> RefBlockBytes {
        let last_2_bytes = (self.block_header.raw_data.number & 0xFFFF) as u16;
        last_2_bytes.to_be_bytes().into()
    }
    /// Get bytes 8..24 of the blockid
    pub(crate) fn calculate_ref_block_hash(&self) -> RefBlockHash {
        self.blockid.0[8..16].try_into().unwrap()
    }
    pub(crate) fn fill_header_info_in_transaction(
        &self,
        transaction: &mut super::transaction::Transaction,
    ) {
        transaction.raw.ref_block_bytes = self.calculate_ref_block_bytes();
        transaction.raw.ref_block_hash = self.calculate_ref_block_hash();
        transaction.raw.ref_block_num = self.block_header.raw_data.number;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub block_header: Option<BlockHeader>,
}
