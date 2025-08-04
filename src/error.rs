use alloy_primitives::U256;
use time::OffsetDateTime;

use crate::domain::address::TronAddress;
use crate::domain::estimate::ResourceState;
use crate::domain::transaction::TxCode;
use crate::domain::trx::Trx;
use crate::domain::{Hash32, Message};
use crate::protocol::transaction::result::ContractResult;

#[derive(thiserror::Error)]
pub enum Error {
    #[error("unexpected error: {0}")]
    Unexpected(
        #[from]
        // #[backtrace]
        anyhow::Error,
    ),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("tron address is not on blockchain (0 incoming transactions)")]
    NoAccount(TronAddress),
    #[error("tron protocol: {0}")]
    TronProtocol(#[from] tonic::Status),
    #[error("transaction failed with: {0}")]
    FailedTransaction(String, Option<ContractResult>),
    #[error("transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("bad header: {0}")]
    BadHeader(#[from] http::header::InvalidHeaderName),
    #[error("bad header value: {0}")]
    BadHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("signature failure: {0}")]
    Signature(#[from] k256::ecdsa::signature::Error),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("precondigion failed: {0}")]
    PreconditionFailed(String),
    #[error("expired at: {0}")]
    Expired(OffsetDateTime),
    #[error("insufficient resources: {0:#?}")]
    InsufficientResources(ResourceState),
    #[error(
        "insufficient token balance: {balance}, but need: {need} for {token}"
    )]
    InsufficientTokenBalance {
        balance: U256,
        need: U256,
        token: &'static str,
    },
    #[error("insufficient balance: {balance}, but need: {need}")]
    InsufficientBalance { balance: Trx, need: Trx },
    #[error(
        "insufficient balance to unfreeze: {frozen}, but trying to unfreeze: {trying_to_unfreeze}"
    )]
    InsufficientFrozen {
        frozen: Trx,
        trying_to_unfreeze: Trx,
    },
    #[error("transaction error: {txid:?}, {result:?}, {msg}")]
    Transaction {
        txid: Hash32,
        result: TxCode,
        msg: Message,
    },
    #[error("transaction was not confirmed within the expected time")]
    TransactionTimeout,
}

crate::impl_debug!(Error);
