use num_bigint::BigUint;

use crate::{
    crypto_functions::keccak256,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
    types::{top_decode_u64, VMTokenType},
};

/// Issues a new fungible token.
#[allow(unused_variables)]
pub fn issue(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 4 {
        let tx_result = TxResult::from_vm_error("not enough arguments");
        return (tx_result, BlockchainUpdate::empty());
    }
    let name = tx_input.args[0].clone();
    let ticker = tx_input.args[1].clone();
    let total_supply = BigUint::from_bytes_be(tx_input.args[2].clone().as_ref());
    let decimals = top_decode_u64(tx_input.args[3].clone().as_ref()) as u32;

    register_and_set_roles(tx_input, tx_cache, ticker, VMTokenType::Fungible)
}

/// Issues a new semi-fungible token.
#[allow(unused_variables)]
pub fn issue_semi_fungible(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 2 {
        let tx_result = TxResult::from_vm_error("not enough arguments");
        return (tx_result, BlockchainUpdate::empty());
    }
    let name = tx_input.args[0].clone();
    let ticker = tx_input.args[1].clone();

    register_and_set_roles(tx_input, tx_cache, ticker, VMTokenType::SemiFungible)
}

/// Issues a new non-fungible token.
#[allow(unused_variables)]
pub fn issue_non_fungible(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 2 {
        let tx_result = TxResult::from_vm_error("not enough arguments");
        return (tx_result, BlockchainUpdate::empty());
    }
    let name = tx_input.args[0].clone();
    let ticker = tx_input.args[1].clone();

    register_and_set_roles(tx_input, tx_cache, ticker, VMTokenType::NonFungible)
}

// Issues a new token and sets all roles for its type.
#[allow(unused_variables)]
pub fn register_and_set_all_roles(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 4 {
        let tx_result = TxResult::from_vm_error("not enough arguments");
        return (tx_result, BlockchainUpdate::empty());
    }

    let name = tx_input.args[0].clone();
    let ticker = tx_input.args[1].clone();
    let token_type = VMTokenType::from_system_sc_arg(&tx_input.args[2]);
    let decimals = top_decode_u64(tx_input.args[3].clone().as_ref()) as u32;

    register_and_set_roles(tx_input, tx_cache, ticker, token_type)
}

fn register_and_set_roles(
    tx_input: TxInput,
    tx_cache: TxCache,
    ticker: Vec<u8>,
    token_type: VMTokenType,
) -> (TxResult, BlockchainUpdate) {
    let mut new_token_identifiers = tx_cache.get_new_token_identifiers();

    let token_identifier = if let Some((i, ti)) =
        first_token_identifier_with_ticker(&new_token_identifiers, &ticker)
    {
        new_token_identifiers.remove(i);
        ti.into_bytes()
    } else {
        generate_token_identifier_from_ticker(&tx_input, &tx_cache, &ticker)
    };

    tx_cache.with_account_mut(&tx_input.from, |account| {
        account
            .esdt
            .register_and_set_roles(&token_identifier, token_type);
    });
    tx_cache.set_new_token_identifiers(new_token_identifiers);

    let tx_result = TxResult {
        result_values: vec![token_identifier],
        ..Default::default()
    };

    (tx_result, tx_cache.into_blockchain_updates())
}

fn first_token_identifier_with_ticker(
    token_identifiers: &[String],
    ticker: &[u8],
) -> Option<(usize, String)> {
    let extract_ticker =
        |ti: &String| -> String { ti.split('-').map(|x| x.to_string()).next().unwrap() };

    token_identifiers
        .iter()
        .position(|x| extract_ticker(x).as_bytes() == ticker)
        .map(|i| (i, token_identifiers[i].clone()))
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
