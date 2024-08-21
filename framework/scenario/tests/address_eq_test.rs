use multiversx_sc::types::TestAddress;
use multiversx_sc_scenario::api::StaticApi;

const ALICE: TestAddress = TestAddress::new("alice");
const SC_ADDR: TestAddress = TestAddress::new("sc");

#[test]
fn test_address_eq() {
    let alice2 = TestAddress::new("alice");

    assert_eq!(ALICE, alice2);
    assert_eq!(ALICE, alice2.to_address());
    assert_eq!(ALICE.to_address(), alice2);
    assert_eq!(ALICE.to_address(), alice2.to_address());

    assert_eq!(ALICE, alice2.to_managed_address::<StaticApi>());
    assert_eq!(ALICE.to_managed_address::<StaticApi>(), alice2);
    assert_eq!(
        ALICE.to_managed_address::<StaticApi>(),
        alice2.to_managed_address::<StaticApi>()
    );
}

#[test]
fn test_sc_address_eq() {
    let sc2 = TestAddress::new("sc");

    assert_eq!(SC_ADDR, sc2);
    assert_eq!(SC_ADDR, sc2.to_address());
    assert_eq!(SC_ADDR.to_address(), sc2);
    assert_eq!(SC_ADDR.to_address(), sc2.to_address());

    assert_eq!(SC_ADDR, sc2.to_managed_address::<StaticApi>());
    assert_eq!(SC_ADDR.to_managed_address::<StaticApi>(), sc2);
    assert_eq!(
        SC_ADDR.to_managed_address::<StaticApi>(),
        sc2.to_managed_address::<StaticApi>()
    );
}
