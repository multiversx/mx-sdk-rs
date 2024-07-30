use multiversx_sc::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    contract_base::ManagedSerializer,
    derive::ManagedVecItem,
    types::{
        BigUint, EsdtTokenPayment, ManagedBuffer, ManagedBufferReadToEnd, ManagedByteArray,
        ManagedType, TokenIdentifier,
    },
};
use multiversx_sc_scenario::api::StaticApi;

#[derive(TopDecode, TopEncode, Clone)]
pub struct CallData<M: ManagedTypeApi> {
    pub endpoint: ManagedBuffer<M>,
    pub gas_limit: u64,
    pub data: ManagedBufferReadToEnd<M>,
}

#[test]
fn read_To_end_codec_test() {
    let cd: CallData<_> = CallData::<StaticApi> {
        endpoint: ManagedBuffer::from("abc"),
        gas_limit: 0x100_0000,
        data: ManagedBuffer::from("ddd").into(),
    };

    #[rustfmt::skip]
    let expected = &[
        /* endpoint length */ 0, 0, 0, 3, 
        /* endpoint contents */ b'a', b'b', b'c',
        /* gas limit */ 0, 0, 0, 0, 1, 0, 0, 0,
        /* data */ b'd', b'd', b'd',
    ];

    let encoded = ManagedSerializer::<StaticApi>::new().top_encode_to_managed_buffer(&cd);
    assert_eq!(encoded.to_boxed_bytes().as_slice(), expected.as_slice());
}
