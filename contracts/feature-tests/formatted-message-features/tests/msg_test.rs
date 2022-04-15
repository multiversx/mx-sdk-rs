use elrond_wasm_debug::DebugApi;
use formatted_message_features::*;

fn check_printed_and_clear(expected: &str) {
    let printed = DebugApi::new_from_static().printed_messages();
    assert_eq!(printed, vec![expected.to_string()]);
    DebugApi::new_from_static().printed_messages_clear();
}

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

    fmf.print_message_hex(10);

    let printed = DebugApi::new_from_static().printed_messages();
    assert_eq!(
        printed,
        vec!["Printing x: 5", "Printing x: 7", "Printing x: a"]
    );

    fmf.print_message_binary(12);

    let printed = DebugApi::new_from_static().printed_messages();
    assert_eq!(
        printed,
        vec![
            "Printing x: 5",
            "Printing x: 7",
            "Printing x: a",
            "Printing x: 1100"
        ]
    );

    fmf.print_message_binary(112);

    let printed = DebugApi::new_from_static().printed_messages();
    assert_eq!(
        printed,
        vec![
            "Printing x: 5",
            "Printing x: 7",
            "Printing x: a",
            "Printing x: 1100",
            "Printing x: 1110000"
        ]
    );

    fmf.print_message_binary(11112);

    let printed = DebugApi::new_from_static().printed_messages();
    assert_eq!(
        printed,
        vec![
            "Printing x: 5",
            "Printing x: 7",
            "Printing x: a",
            "Printing x: 1100",
            "Printing x: 1110000",
            "Printing x: 10101101101000"
        ]
    );
}

#[test]
fn test_print_codecs_hex() {
    let _ = DebugApi::dummy();

    let fmf = formatted_message_features::contract_obj::<DebugApi>();

    fmf.print_message_codec(0);
    check_printed_and_clear("Printing x: ");

    fmf.print_message_hex(0);
    check_printed_and_clear("Printing x: 0");

    fmf.print_message_codec(5);
    check_printed_and_clear("Printing x: 05");

    fmf.print_message_hex(5);
    check_printed_and_clear("Printing x: 5");

    fmf.print_message_codec(-5);
    check_printed_and_clear("Printing x: fb");

    fmf.print_message_hex(-5);
    check_printed_and_clear("Printing x: fffffffb");
}
