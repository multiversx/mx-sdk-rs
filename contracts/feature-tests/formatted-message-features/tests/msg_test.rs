use elrond_wasm_debug::DebugApi;
use formatted_message_features::*;

#[test]
fn test_print() {
    let _ = DebugApi::dummy();

    let fmf = formatted_message_features::contract_obj::<DebugApi>();

    fmf.print_message(5);

    let printed = DebugApi::new_from_static().printed_messages();
    assert_eq!(printed, vec!["Printing x: 5"]);

    fmf.print_message(7);

    let printed = DebugApi::new_from_static().printed_messages();
    assert_eq!(printed, vec!["Printing x: 5", "Printing x: 7"]);
}
