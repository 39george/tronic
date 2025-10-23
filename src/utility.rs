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
    fn try_from_tron(_: i64) -> Result<Self, time::error::ComponentRange>;
}

impl TronOffsetDateTime for time::OffsetDateTime {
    fn to_tron(&self) -> i64 {
        (self.unix_timestamp_nanos() / 1_000_000) as i64
    }
    fn try_from_tron(tm: i64) -> Result<Self, time::error::ComponentRange> {
        let abs = (tm as i128).abs() as i128;
        let ns = if abs <= 253_402_300_799 {
            // secs
            (tm as i128) * 1_000_000_000
        } else if abs <= 253_402_300_799_000 {
            // millis
            (tm as i128) * 1_000_000
        } else if abs <= 253_402_300_799_000_000 {
            // micros
            (tm as i128) * 1_000
        } else {
            // nanos
            tm as i128
        };

        time::OffsetDateTime::from_unix_timestamp_nanos(ns)
    }
}

pub fn generate_txid(raw_data: &[u8]) -> Hash32 {
    let digest = Sha256::digest(raw_data);
    let hash: [u8; 32] = digest.into();
    hash.into()
}
