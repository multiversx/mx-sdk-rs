use multiversx_sc::types::ManagedAddress;
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::address::Address;
use multiversx_sdk::data::types::native::NativeConvertible;

#[test]
fn test_managed_address_to_native() {
    let _ = DebugApi::dummy();
    let expected = Address::from_bech32_string("erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh").unwrap();
    let managed_address: ManagedAddress<DebugApi> = ManagedAddress::from(expected.to_bytes());
    let native = managed_address.to_native();

    assert_eq!(
        native.to_bytes(),
        expected.to_bytes()
    )
}