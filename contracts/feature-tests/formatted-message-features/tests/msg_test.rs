use formatted_message_features::*;
use multiversx_sc_scenario::DebugApi;

fn check_printed_and_clear(expected: &str) {
    let printed = DebugApi::new_from_static().printed_messages();
    assert_eq!(printed, vec![expected.to_string()]);
    DebugApi::new_from_static().printed_messages_clear();
}

#[test]
fn test_print_ascii() {
    let _ = DebugApi::dummy();
    let fmf = formatted_message_features::contract_obj::<DebugApi>();

    fmf.print_message(5);
    check_printed_and_clear("Printing x: 5");

    fmf.print_message(7);
    check_printed_and_clear("Printing x: 7");

    fmf.print_message(i32::MAX);
    check_printed_and_clear("Printing x: 2147483647");

    fmf.print_message(i32::MIN);
    check_printed_and_clear("Printing x: -2147483648");
}

#[test]
fn test_print_binary() {
    let _ = DebugApi::dummy();
    let fmf = formatted_message_features::contract_obj::<DebugApi>();

    fmf.print_message_binary(12);
    check_printed_and_clear("Printing x: 1100");

    fmf.print_message_binary(112);
    check_printed_and_clear("Printing x: 1110000");

    fmf.print_message_binary(11112);
    check_printed_and_clear("Printing x: 10101101101000");

    fmf.print_message_binary(u32::MAX);
    check_printed_and_clear("Printing x: 11111111111111111111111111111111");
}

#[test]
fn test_print_hex() {
    let _ = DebugApi::dummy();
    let fmf = formatted_message_features::contract_obj::<DebugApi>();

    fmf.print_message_hex(0);
    check_printed_and_clear("Printing x: 0");

    fmf.print_message_hex(5);
    check_printed_and_clear("Printing x: 5");

    fmf.print_message_hex(-5);
    check_printed_and_clear("Printing x: fffffffb");

    fmf.print_message_hex(i32::MAX);
    check_printed_and_clear("Printing x: 7fffffff");

    fmf.print_message_hex(i32::MIN);
    check_printed_and_clear("Printing x: 80000000");
}

#[test]
fn test_print_codecs() {
    let _ = DebugApi::dummy();
    let fmf = formatted_message_features::contract_obj::<DebugApi>();

    fmf.print_message_codec(0);
    check_printed_and_clear("Printing x: ");

    fmf.print_message_codec(5);
    check_printed_and_clear("Printing x: 05");

    fmf.print_message_codec(-5);
    check_printed_and_clear("Printing x: fb");

    fmf.print_message_codec(i32::MAX);
    check_printed_and_clear("Printing x: 7fffffff");

    fmf.print_message_codec(i32::MIN);
    check_printed_and_clear("Printing x: 80000000");
}
