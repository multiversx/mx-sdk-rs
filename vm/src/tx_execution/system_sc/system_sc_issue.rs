use num_bigint::BigUint;

use crate::{
    crypto_functions::keccak256,
    tx_mock::{TxCache, TxContext, TxInput, TxResult},
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

    let token_identifier = if let Some((i, ti)) =
        first_token_identifier_with_ticker(&new_token_identifiers, &ticker)
    {
        new_token_identifiers.remove(i);
        ti.into_bytes()
    } else {
        generate_token_identifier_from_ticker(tx_input, tx_cache, &ticker)
    };

    println!(
        "\n\ngenerated new token_identifier: {}\n\n",
        std::str::from_utf8(&token_identifier).unwrap()
    );

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

/// Issues a new semi-fungible token.
pub fn issue_semi_fungible(tx_context: TxContext) -> (TxContext, TxResult) {
    issue_non_fungible(tx_context)
}

/// Issues a new non-fungible token.
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

    let token_identifier = if let Some((i, ti)) =
        first_token_identifier_with_ticker(&new_token_identifiers, &ticker)
    {
        new_token_identifiers.remove(i);
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

fn first_token_identifier_with_ticker(
    token_identifiers: &Vec<String>,
    ticker: &[u8],
) -> Option<(usize, String)> {
    let extract_ticker =
        |ti: &String| -> String { ti.split("-").map(|x| x.to_string()).next().unwrap() };

    let position = token_identifiers
        .iter()
        .position(|x| extract_ticker(x).as_bytes() == ticker);

    if let Some(i) = position {
        Some((i, token_identifiers[i].clone()))
    } else {
        None
    }
}

fn generate_token_identifier_from_ticker(
    tx_input: &TxInput,
    tx_cache: &TxCache,
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

    token_identifier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_token_identifier_with_ticker_ok() {
        let ticker = String::from("BBBB").into_bytes();
        let new_token_indetifiers = vec![
            "AAAA-0123".to_string(),
            "BBBB-4567".to_string(),
            "BBBB-0123".to_string(),
            "CCCC-4567".to_string(),
        ];

        let ti = first_token_identifier_with_ticker(&new_token_indetifiers, &ticker);
        let expected = b"BBBB-4567".as_slice();
        assert_eq!(expected, ti.unwrap().1.into_bytes());
    }

    #[test]
    fn test_first_token_identifier_with_ticker_is_none() {
        let ticker = String::from("BBBB").into_bytes();
        let new_token_indetifiers = vec!["AAAA-0123".to_string()];

        let i = first_token_identifier_with_ticker(&new_token_indetifiers, &ticker);
        let expected = None;
        assert_eq!(expected, i);
    }
}
