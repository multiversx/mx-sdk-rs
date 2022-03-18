use elrond_wasm_debug::DebugApi;
use formatted_message_features::*;

#[test]
fn test_print() {
    let _ = DebugApi::dummy();

    let fmf = formatted_message_features::contract_obj::<DebugApi>();

    fmf.print_message(5);

    // TODO: also test print output
}
