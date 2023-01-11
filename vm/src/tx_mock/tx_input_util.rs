use multiversx_sc::types::heap::H256;

pub fn generate_tx_hash_dummy(tx_id: &str) -> H256 {
    let bytes = tx_id.as_bytes();
    let mut result = [b'.'; 32];
    if bytes.len() > 32 {
        result[..].copy_from_slice(&bytes[..32]);
    } else {
        result[..bytes.len()].copy_from_slice(bytes);
    }
    result.into()
}
