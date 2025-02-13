use multiversx_chain_core::types::EsdtLocalRole;
use multiversx_sc_codec::multi_types::IgnoreValue;
extern crate bech32_no_std;

use crate::{
    api::ManagedTypeApi,
    codec::{DecodeErrorHandler, TopDecode, TopDecodeInput},
    types::{ManagedAddress, MultiValueEncoded},
};

use bech32_no_std::FromBase32;

pub type SpecialRolesResult<M> = MultiValueEncoded<M, LocalRoleEntry<M>>;

#[derive(Debug, Clone, Default)]
pub struct LocalRoleEntry<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub roles: MultiValueEncoded<M, EsdtLocalRole>,
}

impl<M: ManagedTypeApi> LocalRoleEntry<M> {
    pub fn new() -> Self {
        Self {
            address: ManagedAddress::zero(),
            roles: MultiValueEncoded::new(),
        }
    }
}

pub type GlobalRole = IgnoreValue;

impl<M: ManagedTypeApi> TopDecode for LocalRoleEntry<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        const ADDRESS_LEN: usize = 62;

        if input.byte_len() < ADDRESS_LEN {
            return Ok(LocalRoleEntry::new());
        }

        let mut buffer = [0u8; 279]; // max possible size
        input.into_max_size_buffer_align_right(&mut buffer, h);

        let address_buffer = &buffer[..ADDRESS_LEN];
        let roles_buffer = &buffer[ADDRESS_LEN + 2..];

        let decoded_address = FromBase32::from_base32(address_buffer.to_vec())
            .expect("Failed to decode base32 address");
        let address = ManagedAddress::new_from_bytes(decoded_address);

        let mut roles = MultiValueEncoded::<M, EsdtLocalRole>::new();

        for role_slice in roles_buffer.split(|&b| b == b',') {
            roles.push(EsdtLocalRole::from(role_slice));
        }

        Ok(LocalRoleEntry { address, roles })
    }
}
