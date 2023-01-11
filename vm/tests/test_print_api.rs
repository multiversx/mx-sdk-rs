use multiversx_chain_vm::{BigUintPrinter, DebugApi};
use multiversx_sc::types::BigUint;

#[test]
fn test_print_api() {
    let _ = DebugApi::dummy();

    let zero = BigUint::<DebugApi>::from(0u64);
    assert_eq!(
        format!("{:?}", BigUintPrinter { value: zero }),
        "BigUint { handle: -100, hex: \"00\", dec: \"0\" }"
    );

    let regular = BigUint::<DebugApi>::from(257u64);
    assert_eq!(
        format!("{:?}", BigUintPrinter { value: regular }),
        "BigUint { handle: -101, hex: \"0101\", dec: \"257\" }"
    );

    let huge_number = BigUint::<DebugApi>::from_bytes_be(&[
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]);
    assert_eq!(
        format!("{:?}", BigUintPrinter { value: huge_number }),
        "BigUint { handle: -102, hex: \"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\", dec: \"7588550360256754183279148073529370729071901715047420004889892225542594864082845695\" }"
    );
}
