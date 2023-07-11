mod vm_address;
mod vm_code_metadata;
mod vm_esdt_local_role;
mod vm_esdt_local_role_flags;
mod vm_h256;
mod vm_token_type;

use num_bigint::BigUint;
use num_traits::Zero;
pub use vm_address::VMAddress;
pub use vm_code_metadata::VMCodeMetadata;
pub use vm_esdt_local_role::EsdtLocalRole;
pub use vm_esdt_local_role_flags::EsdtLocalRoleFlags;
pub use vm_h256::H256;
pub use vm_token_type::VMTokenType;

pub type RawHandle = i32;

pub(crate) fn top_encode_u64(value: u64) -> Vec<u8> {
    top_encode_big_uint(&BigUint::from(value))
}

pub(crate) fn top_encode_big_uint(value: &BigUint) -> Vec<u8> {
    if value.is_zero() {
        Vec::new()
    } else {
        value.to_bytes_be()
    }
}

pub(crate) fn top_decode_u64(bytes: &[u8]) -> u64 {
    BigUint::from_bytes_be(bytes).try_into().unwrap()
}
