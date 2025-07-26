use sha2::{Digest, Sha256};

use crate::domain::Hash32;

pub fn estimate_bandwidth(
    raw_data_len_bytes: i64,
    signature_list_size: i64,
) -> i64 {
    const DATA_PROTOBUF_EXTRA: i64 = 3;
    const MAX_RESULT_SIZE: i64 = 64;
    const SIGNATURE_SIZE: i64 = 67;

    let mut estimated_bandwidth =
        raw_data_len_bytes + DATA_PROTOBUF_EXTRA + MAX_RESULT_SIZE;
    for _ in 0..signature_list_size {
        estimated_bandwidth += SIGNATURE_SIZE;
    }
    estimated_bandwidth
}

pub trait TronOffsetDateTime: Sized {
    fn to_tron(&self) -> i64;
    fn from_tron(_: i64) -> Self;
}

impl TronOffsetDateTime for time::OffsetDateTime {
    fn to_tron(&self) -> i64 {
        (self.unix_timestamp_nanos() / 1_000_000) as i64
    }
    fn from_tron(tm: i64) -> Self {
        time::OffsetDateTime::from_unix_timestamp_nanos(tm as i128 * 1_000_000)
            .inspect_err(|e| {
                tracing::error!(
                    "failed to create OffsetDateTime from unix_timestamp: {e}"
                )
            })
            .unwrap_or(time::OffsetDateTime::UNIX_EPOCH)
    }
}

pub fn generate_txid(raw_data: &[u8]) -> Hash32 {
    let hash: [u8; 32] = Sha256::digest(raw_data).into();
    hash.into()
}
