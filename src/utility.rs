pub(crate) fn estimate_bandwidth(
    raw_data_len_bytes: i64,
    // 1 by default
    signature_list_size: i64,
) -> i64 {
    const DATA_PROTOBUF_EXTRA: i64 = 3;
    const MAX_RESULT_SIZE: i64 = 64;
    const SIGNATURE_SIZE: i64 = 67; // Bytes per signature

    let mut estimated_bandwidth =
        raw_data_len_bytes + DATA_PROTOBUF_EXTRA + MAX_RESULT_SIZE;
    for _ in 0..signature_list_size {
        // Iterate based on number of signatures
        estimated_bandwidth += SIGNATURE_SIZE;
    }
    estimated_bandwidth // Return total estimated bandwidth in bytes
}
