use core::marker::PhantomData;

use multiversx_chain_core::types::EsdtLocalRole;

use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedEncode, TopDecode, TopEncode},
        DecodeErrorHandler, TopDecode, TopDecodeInput, TopEncodeMulti,
    },
    types::{
        interaction::decode_result, ManagedAddress, MultiValueEncoded, RHListItem, RHListItemExec,
        SyncCallRawResult, TxEnv,
    },
};

// #[derive(TopEncode, NestedEncode)]
#[derive(Debug, Clone)]
pub struct SpecialRolesForAddress<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub roles: MultiValueEncoded<M, EsdtLocalRole>,
}

impl<M: ManagedTypeApi> TopDecode for SpecialRolesForAddress<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        // address:role,role,role,role
        const ADDRESS_LEN: usize = 62;

        let buffer = input.into_boxed_slice_u8();
        let address_bytes = &buffer[..ADDRESS_LEN];
        let roles_bytes = &buffer[64..]; // address len + separator (1)

        let mut roles_vec = MultiValueEncoded::new();
        let mut start = 0;

        for (i, &byte) in roles_bytes.iter().enumerate() {
            if byte == b',' {
                // first part between start and i
                if let Ok(role) = EsdtLocalRole::top_decode_or_handle_err(&roles_bytes[start..i], h)
                {
                    roles_vec.push(role);
                }
                start = i + 1;
            }
        }

        // after last ','
        if start < roles_bytes.len() {
            if let Ok(role) = EsdtLocalRole::top_decode_or_handle_err(&roles_bytes[start..], h) {
                roles_vec.push(role);
            }
        }

        Ok(Self {
            address: ManagedAddress::top_decode_or_handle_err(address_bytes, h)?,
            roles: roles_vec,
        })
    }
}

// each element is made from address : role,role,role,role
// let mut roles = MultiValueEncoded::new();

// find : and separate buffer
// let address_buffer = buffer.copy_slice(0usize, 62usize).unwrap(); // address length
// let roles_buffer = buffer.copy_slice(64usize, buffer.len()).unwrap();

// left decode as address
// let address =
//     ManagedAddress::top_decode(address_buffer).expect("couldn't decode as address");

// right separate by ,

// decode each as esdtLocalRole
// for role_buffer in roles_buffer {
//     let role = EsdtLocalRole::top_decode(role_buffer)
//         .expect("couldn't decode as esdtLocalRole");
//     roles.push(role);
// }
// collect results in vec of struct

pub type SpecialRolesResult<M> = MultiValueEncoded<M, SpecialRolesForAddress<M>>;

// pub struct ReturnsSpecialRolesResult<M: ManagedTypeApi>(PhantomData<M>);

// impl<Env, Original, M> RHListItem<Env, Original> for ReturnsSpecialRolesResult<M>
// where
//     Env: TxEnv,
//     M: ManagedTypeApi,
// {
//     type Returns = SpecialRolesResult<M>;
// }

// impl<Env, Original, M> RHListItemExec<SyncCallRawResult<Env::Api>, Env, Original>
//     for ReturnsSpecialRolesResult<M>
// where
//     Env: TxEnv,
//     M: ManagedTypeApi,
// {
//     fn item_process_result(self, raw_result: &SyncCallRawResult<Env::Api>) -> Self::Returns {
//         let mut vec_of_buffers = raw_result.0.clone();
//         // take out global roles (ESDTBurnForAll as of now)
//         let _global_roles = vec_of_buffers.take(0);

//         let mut roles_and_addresses = MultiValueEncoded::new();

//         for buffer in vec_of_buffers {
//             let role_and_address = SpecialRolesForAddress::top_decode(buffer)
//                 .expect("decode error on special roles and address map");
//             roles_and_addresses.push(role_and_address);
//         }

//         roles_and_addresses
//     }

//     fn item_tx_expect(&self, prev: <Env as TxEnv>::RHExpect) -> <Env as TxEnv>::RHExpect {
//         prev
//     }
// }
