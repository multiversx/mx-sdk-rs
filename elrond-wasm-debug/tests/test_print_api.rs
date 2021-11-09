use elrond_wasm::types::{BigUint, ManagedFrom};
use elrond_wasm_debug::{BigUintPrinter, DebugApi};

#[test]
fn test_print_api() {
    let api = DebugApi::dummy();

    let zero = BigUint::managed_from(api.clone(), 0u64);
    assert_eq!(
        format!("{:?}", BigUintPrinter { value: zero }),
        "BigUint { handle: 0, hex: \"00\", dec: \"0\" }"
    );

    let regular = BigUint::managed_from(api.clone(), 257u64);
    assert_eq!(
        format!("{:?}", BigUintPrinter { value: regular }),
        "BigUint { handle: 1, hex: \"0101\", dec: \"257\" }"
    );

    let huge_number = BigUint::from_bytes_be(
        api.clone(),
        &[
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ],
    );
    assert_eq!(
        format!("{:?}", BigUintPrinter { value: huge_number }),
        "BigUint { handle: 2, hex: \"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\", dec: \"7588550360256754183279148073529370729071901715047420004889892225542594864082845695\" }"
    );
}
