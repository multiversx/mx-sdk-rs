use multiversx_sc::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    derive::ManagedVecItem,
    types::{
        BigUint, ManagedBuffer, ManagedType, ManagedVecItemPayload, ManagedVecItemPayloadBuffer,
        Ref,
    },
};
use multiversx_sc_scenario::api::StaticApi;

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_enum_2_managed > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug,
)]
enum EnumWithFieldsManaged<M: ManagedTypeApi> {
    Variant1,
    Variant2(ManagedBuffer<M>),
    Variant3(BigUint<M>),
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn enum_static() {
    assert_eq!(
        <EnumWithFieldsManaged<StaticApi> as multiversx_sc::types::ManagedVecItem>::payload_size(),
        5
    );
    assert!(
        !<EnumWithFieldsManaged<StaticApi> as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn enum_to_bytes_writer_variant_2() {
    let mut payload = ManagedVecItemPayloadBuffer::new_buffer();
    let mb = ManagedBuffer::from("abc");
    let handle_bytes = mb.get_raw_handle().to_be_bytes();
    <EnumWithFieldsManaged<StaticApi> as multiversx_sc::types::ManagedVecItem>::save_to_payload(
        EnumWithFieldsManaged::Variant2(mb),
        &mut payload,
    );
    assert_eq!(
        payload.into_array(),
        [
            1,
            handle_bytes[0],
            handle_bytes[1],
            handle_bytes[2],
            handle_bytes[3]
        ]
    );
}

#[test]
fn enum_from_bytes_reader_variant_2() {
    unsafe {
        let mb = ManagedBuffer::from("abc");
        let handle_bytes = mb.get_raw_handle().to_be_bytes();
        let payload = [
            1,
            handle_bytes[0],
            handle_bytes[1],
            handle_bytes[2],
            handle_bytes[3],
        ];
        let enum_from_bytes =
            <EnumWithFieldsManaged<StaticApi> as multiversx_sc::types::ManagedVecItem>::borrow_from_payload(
                &payload.into(),
            )
        ;
        assert_eq!(
            enum_from_bytes,
            Ref::new(EnumWithFieldsManaged::<StaticApi>::Variant2(mb))
        );
    }
}
