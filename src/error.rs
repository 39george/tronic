use crate::domain::address::TronAddress;
use crate::protocol::transaction::result::ContractResult;

#[derive(thiserror::Error)]
pub enum Error {
    #[error("unexpected error: {0}")]
    Unexpected(
        #[from]
        #[backtrace]
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
}

crate::impl_debug!(Error);
