use num_bigint::BigUint;

use crate::{
    crypto_functions::keccak256,
    tx_mock::{TxContext, TxResult},
    types::top_decode_u64,
};

/// Issues a new token.
pub fn issue(tx_context: TxContext) -> (TxContext, TxResult) {
    let tx_input = tx_context.input_ref();
    let tx_cache = tx_context.blockchain_cache();
    let tx_result: TxResult;

    if tx_input.args.len() < 4 {
        tx_result = TxResult::from_vm_error("not enough arguments");
        return (tx_context, tx_result);
    }
    let _name = tx_input.args[0].clone();
    let ticker = tx_input.args[1].clone();
    let _total_supply = BigUint::from_bytes_be(tx_input.args[2].clone().as_ref());
    let _decimals = top_decode_u64(tx_input.args[3].clone().as_ref()) as u32;

    let mut new_token_identifiers = tx_cache.get_new_token_identifiers();
    let token_identifier = if let Some(ti) = new_token_identifiers.pop_front() {
        println!("\n\ntoken_identifier: {}\n\n", ti);
        ti.into_bytes()
    } else {
        generate_token_identifier_from_ticker(tx_input, tx_cache, &ticker)
    };

    tx_cache.with_account_mut(&tx_input.from, |account| {
        account.esdt.issue_token(&token_identifier);
    });
    tx_cache.set_new_token_identifiers(new_token_identifiers);

    tx_result = TxResult {
        result_values: vec![token_identifier],
        ..Default::default()
    };

    (tx_context, tx_result)
}

/// Issues a semi-fungible token.
pub fn issue_semi_fungible(tx_context: TxContext) -> (TxContext, TxResult) {
    issue_non_fungible(tx_context)
}

/// Issues a non-fungible token.
pub fn issue_non_fungible(tx_context: TxContext) -> (TxContext, TxResult) {
    let tx_input = tx_context.input_ref();
    let tx_cache = tx_context.blockchain_cache();
    let tx_result: TxResult;

    if tx_input.args.len() < 2 {
        tx_result = TxResult::from_vm_error("not enough arguments");
        return (tx_context, tx_result);
    }
    let _name = tx_input.args[0].clone();
    let ticker = tx_input.args[1].clone();

    let mut new_token_identifiers = tx_cache.get_new_token_identifiers();
    let token_identifier = if let Some(ti) = new_token_identifiers.pop_front() {
        println!("\n\ntoken_identifier: {}\n\n", ti);
        ti.into_bytes()
    } else {
        generate_token_identifier_from_ticker(tx_input, tx_cache, &ticker)
    };

    tx_cache.with_account_mut(&tx_input.from, |account| {
        account.esdt.issue_token(&token_identifier);
    });
    tx_cache.set_new_token_identifiers(new_token_identifiers);

    tx_result = TxResult {
        result_values: vec![token_identifier],
        ..Default::default()
    };

    (tx_context, tx_result)
}

fn generate_token_identifier_from_ticker(
    tx_input: &crate::tx_mock::TxInput,
    tx_cache: &crate::tx_mock::TxCache,
    ticker: &[u8],
) -> Vec<u8> {
    let new_random_base = [
        tx_input.from.as_bytes(),
        tx_cache
            .blockchain_ref()
            .current_block_info
            .block_random_seed
            .as_slice(),
    ]
    .concat();
    let new_random = keccak256(&new_random_base);
    let new_random_for_ticker = &new_random[..3];

    let token_identifier = [
        ticker,
        "-".as_bytes(),
        hex::encode(new_random_for_ticker).as_bytes(),
    ]
    .concat();
    println!(
        "\n\ngenerated new token_identifier: {}\n\n",
        std::str::from_utf8(&token_identifier).unwrap()
    );

    token_identifier
}
