use multiversx_sc::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    derive::ManagedVecItem,
    types::{
        BigInt, BigUint, ConstDecimals, ManagedDecimal, ManagedDecimalSigned,
        ManagedVecItemPayload, NumDecimals,
    },
};
use multiversx_sc_scenario::api::StaticApi;

// to test, run the following command in the crate folder:
// cargo expand --test derive_managed_vec_item_biguint_test > expanded.rs

#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug,
)]
pub struct ManagedStructWithDecimal<M: ManagedTypeApi> {
    pub var_dec: ManagedDecimal<M, NumDecimals>,
    pub const_dec: ManagedDecimal<M, ConstDecimals<4>>,
    pub var_dec_signed: ManagedDecimalSigned<M, NumDecimals>,
    pub const_dec_signed: ManagedDecimalSigned<M, ConstDecimals<4>>,
}

#[test]
#[allow(clippy::assertions_on_constants)]
fn struct_with_decimal_static() {
    assert_eq!(
        <ManagedStructWithDecimal<StaticApi> as multiversx_sc::types::ManagedVecItem>::payload_size(
        ),
        24
    );
    assert!(
        !<ManagedStructWithDecimal<StaticApi> as multiversx_sc::types::ManagedVecItem>::SKIPS_RESERIALIZATION
    );
}

#[test]
fn struct_with_decimal_read_write() {
    let num_dec_1 = 3;
    let num_dec_2 = 5;
    let s = ManagedStructWithDecimal::<StaticApi> {
        var_dec: ManagedDecimal::from_raw_units(BigUint::from(123_000u32), num_dec_1),
        const_dec: ManagedDecimal::from_raw_units(BigUint::from(124_000u32), ConstDecimals),
        var_dec_signed: ManagedDecimalSigned::from_raw_units(BigInt::from(125_000), num_dec_2),
        const_dec_signed: ManagedDecimalSigned::from_raw_units(
            BigInt::from(-126_000),
            ConstDecimals,
        ),
    };

    let mut payload = <ManagedStructWithDecimal<StaticApi> as multiversx_sc::types::ManagedVecItem>::PAYLOAD::new_buffer();
    <ManagedStructWithDecimal<StaticApi> as multiversx_sc::types::ManagedVecItem>::save_to_payload(
        s.clone(),
        &mut payload,
    );
    let struct_from_bytes =
        <ManagedStructWithDecimal<StaticApi> as multiversx_sc::types::ManagedVecItem>::read_from_payload(
            &payload
        );
    assert_eq!(struct_from_bytes, s);

    // check payload
    let slice = payload.payload_slice();
    assert_eq!(slice[7], num_dec_1 as u8);
    assert_eq!(slice[19], num_dec_2 as u8);
}
