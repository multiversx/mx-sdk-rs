use multiversx_sc::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{TopDecode, TopEncode},
    },
    contract_base::ManagedSerializer,
    derive::type_abi,
    types::{ManagedBuffer, ManagedBufferReadToEnd},
};
use multiversx_sc_scenario::api::StaticApi;

#[type_abi]
#[derive(TopDecode, TopEncode, Clone, PartialEq, Debug)]
pub struct CallData<M: ManagedTypeApi> {
    pub endpoint: ManagedBuffer<M>,
    pub gas_limit: u64,
    pub data: ManagedBufferReadToEnd<M>,
}

#[test]
fn read_to_end_codec_test() {
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

    let decoded: CallData<StaticApi> =
        ManagedSerializer::<StaticApi>::new().top_decode_from_managed_buffer(&encoded);
    assert_eq!(decoded, cd);

    assert_eq!(
        decoded.data.as_managed_buffer(),
        &ManagedBuffer::from("ddd")
    );
    assert_eq!(
        decoded.data.into_managed_buffer(),
        ManagedBuffer::from("ddd")
    );
}
