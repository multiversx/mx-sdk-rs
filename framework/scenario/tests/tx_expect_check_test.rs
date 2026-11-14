use multiversx_sc::chain_core::types::ReturnCode;
use multiversx_sc_scenario::scenario_model::{TxExpect, TxResponse, TxResponseStatus};

fn response(status: ReturnCode, message: &str, out: Vec<Vec<u8>>) -> TxResponse {
    TxResponse {
        out,
        tx_error: TxResponseStatus::new(status, message),
        ..Default::default()
    }
}

#[test]
fn tx_expect_check_ok() {
    let success = response(ReturnCode::Success, "", vec![b"abc".to_vec()]);
    assert!(TxExpect::ok().check_response(&success).is_ok());
    assert!(
        TxExpect::ok()
            .result("str:abc")
            .check_response(&success)
            .is_ok()
    );

    let failed = response(ReturnCode::UserError, "boom", vec![]);
    assert!(
        TxExpect::user_error("str:boom")
            .check_response(&failed)
            .is_ok()
    );
}

#[test]
fn tx_expect_check_status_mismatch() {
    let failed = response(ReturnCode::UserError, "boom", vec![]);
    let mismatch = TxExpect::ok().check_response(&failed).unwrap_err();
    assert_eq!(mismatch.field, "status");
    assert!(mismatch.expected.contains('0'));
    assert!(mismatch.actual.contains("boom"));

    // expecting an error, but the tx succeeded
    let success = response(ReturnCode::Success, "", vec![]);
    let mismatch = TxExpect::user_error("str:boom")
        .check_response(&success)
        .unwrap_err();
    assert_eq!(mismatch.field, "status");
}

#[test]
fn tx_expect_check_message_mismatch() {
    let failed = response(ReturnCode::UserError, "boom", vec![]);
    let mismatch = TxExpect::user_error("str:other")
        .check_response(&failed)
        .unwrap_err();
    assert_eq!(mismatch.field, "message");
    assert_eq!(mismatch.actual, "boom");
}

#[test]
fn tx_expect_check_out_mismatch() {
    let success = response(ReturnCode::Success, "", vec![b"abc".to_vec()]);
    let mismatch = TxExpect::ok()
        .result("str:xyz")
        .check_response(&success)
        .unwrap_err();
    assert_eq!(mismatch.field, "out");
}
