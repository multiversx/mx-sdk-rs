pub fn generate_tx_hash(
    tx_id: &str,
    explicit_tx_hash: &Option<multiversx_sc::types::H256>,
) -> multiversx_chain_vm::types::H256 {
    if let Some(explicit_tx_hash) = explicit_tx_hash {
        explicit_tx_hash.as_array().into()
    } else {
        let bytes = tx_id.as_bytes();
        let mut result = [b'.'; 32];
        if bytes.len() > 32 {
            result[..].copy_from_slice(&bytes[..32]);
        } else {
            result[..bytes.len()].copy_from_slice(bytes);
        }
        result.into()
    }
}
