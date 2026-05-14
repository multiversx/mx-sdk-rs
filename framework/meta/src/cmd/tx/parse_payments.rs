use anyhow::{Context, Result, anyhow};
use multiversx_sc_snippets::imports::{
    BigUint, Payment, PaymentVec, RustBigUint, StaticApi, TokenId,
};

use crate::cli::cli_args_tx::PaymentArgs;

/// Build a [`PaymentVec`] from [`PaymentArgs`]:
/// the `--token-transfers` pairs, then the `--payments` triples, then the
/// `--value` EGLD amount (appended last as a native `EGLD-000000` payment).
/// The interactor's `.payment()` normalises the vec into the correct transaction fields.
pub fn parse_all_payment_args(payment: &PaymentArgs) -> Result<PaymentVec<StaticApi>> {
    let mut payments = PaymentVec::new();
    payments.append_vec(parse_token_transfers_arg(&payment.token_transfers)?);
    payments.append_vec(parse_payments_arg(&payment.payments)?);
    if payment.value > 0 {
        let amount = BigUint::<StaticApi>::from(payment.value);
        payments.push(
            Payment::try_new(TokenId::native(), 0u64, amount)
                .map_err(|_| anyhow!("EGLD value must be non-zero"))?,
        );
    }
    Ok(payments)
}

/// Parse a flat `--token-transfers` list (`TOKEN-IDENT AMOUNT …` pairs) into payments.
/// The token identifier may include a hex nonce suffix for NFT/SFT: `TOKEN-abc-0a`.
fn parse_token_transfers_arg(transfers: &[String]) -> Result<PaymentVec<StaticApi>> {
    if transfers.len() % 2 != 0 {
        return Err(anyhow!(
            "--token-transfers requires an even number of values (TOKEN-IDENT AMOUNT …)"
        ));
    }
    let mut payments = PaymentVec::new();
    for chunk in transfers.chunks(2) {
        let extended_id = &chunk[0];
        let amount_str = &chunk[1];
        let (base_id, nonce) = split_extended_identifier(extended_id);
        let rust_amount: RustBigUint = amount_str
            .parse()
            .with_context(|| format!("invalid token amount: {amount_str}"))?;
        let amount = BigUint::<StaticApi>::from(rust_amount);
        payments.push(
            Payment::try_new(
                TokenId::<StaticApi>::from(base_id.as_bytes()),
                nonce,
                amount,
            )
            .map_err(|_| anyhow!("token amount must be non-zero: {extended_id}"))?,
        );
    }
    Ok(payments)
}

/// Parse a flat `--payments` list (`TOKEN-IDENT NONCE AMOUNT …` triples) into payments.
/// Nonce is an explicit decimal `u64`; use 0 for fungible tokens.
fn parse_payments_arg(explicit: &[String]) -> Result<PaymentVec<StaticApi>> {
    if explicit.len() % 3 != 0 {
        return Err(anyhow!(
            "--payments requires a multiple of 3 values (TOKEN-IDENT NONCE AMOUNT …)"
        ));
    }
    let mut payments = PaymentVec::new();
    for chunk in explicit.chunks(3) {
        let token_id_str = &chunk[0];
        let nonce_str = &chunk[1];
        let amount_str = &chunk[2];
        let nonce: u64 = nonce_str
            .parse()
            .with_context(|| format!("invalid nonce: {nonce_str}"))?;
        let rust_amount: RustBigUint = amount_str
            .parse()
            .with_context(|| format!("invalid token amount: {amount_str}"))?;
        let amount = BigUint::<StaticApi>::from(rust_amount);
        payments.push(
            Payment::try_new(
                TokenId::<StaticApi>::from(token_id_str.as_bytes()),
                nonce,
                amount,
            )
            .map_err(|_| anyhow!("token amount must be non-zero: {token_id_str}"))?,
        );
    }
    Ok(payments)
}

/// Split an mxpy-style extended token identifier into `(base_identifier, nonce)`.
///
/// Format: `TOKEN-xxxxxx` (fungible, nonce = 0) or `TOKEN-xxxxxx-<hex>` (NFT/SFT).
fn split_extended_identifier(extended_id: &str) -> (String, u64) {
    let parts: Vec<&str> = extended_id.split('-').collect();
    if parts.len() >= 3 {
        let last = parts[parts.len() - 1];
        if !last.is_empty() && last.bytes().all(|b| b.is_ascii_hexdigit()) {
            if let Ok(nonce) = u64::from_str_radix(last, 16) {
                let base = parts[..parts.len() - 1].join("-");
                return (base, nonce);
            }
        }
    }
    (extended_id.to_string(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── split_extended_identifier ──────────────────────────────────────────────

    #[test]
    fn split_fungible_two_parts() {
        let (base, nonce) = split_extended_identifier("USDC-350c4e");
        assert_eq!(base, "USDC-350c4e");
        assert_eq!(nonce, 0);
    }

    #[test]
    fn split_nft_hex_nonce() {
        // nonce 10 = 0x0a
        let (base, nonce) = split_extended_identifier("NFT-abc123-0a");
        assert_eq!(base, "NFT-abc123");
        assert_eq!(nonce, 10);
    }

    #[test]
    fn split_nft_large_nonce() {
        // nonce 256 = 0x100
        let (base, nonce) = split_extended_identifier("SFT-abc123-0100");
        assert_eq!(base, "SFT-abc123");
        assert_eq!(nonce, 256);
    }

    #[test]
    fn split_non_hex_suffix_left_intact() {
        // "xyz" has 'x' which is not a hex digit — treated as fungible
        let (base, nonce) = split_extended_identifier("TOKEN-abc123-xyz");
        assert_eq!(base, "TOKEN-abc123-xyz");
        assert_eq!(nonce, 0);
    }

    #[test]
    fn split_egld_special_identifier() {
        let (base, nonce) = split_extended_identifier("EGLD-000000");
        assert_eq!(base, "EGLD-000000");
        assert_eq!(nonce, 0);
    }

    #[test]
    fn split_single_part_no_dash() {
        let (base, nonce) = split_extended_identifier("EGLD");
        assert_eq!(base, "EGLD");
        assert_eq!(nonce, 0);
    }

    // ── parse_token_transfers ──────────────────────────────────────────────────

    #[test]
    fn token_transfers_odd_count_is_error() {
        let result = parse_token_transfers_arg(&["USDC-350c4e".to_string()]);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("even number of values")
        );
    }

    #[test]
    fn token_transfers_invalid_amount_is_error() {
        let result =
            parse_token_transfers_arg(&["USDC-350c4e".to_string(), "not_a_number".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn token_transfers_zero_amount_is_error() {
        let result = parse_token_transfers_arg(&["USDC-350c4e".to_string(), "0".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn token_transfers_single_fungible() {
        let result = parse_token_transfers_arg(&["USDC-350c4e".to_string(), "10000".to_string()]);
        let payments = result.expect("should succeed");
        assert_eq!(payments.len(), 1);
        assert_eq!(payments.get(0).token_nonce, 0);
        assert_eq!(
            payments.get(0).token_identifier,
            TokenId::<StaticApi>::from(b"USDC-350c4e" as &[u8])
        );
    }

    #[test]
    fn token_transfers_nft_nonce_decoded_from_hex_suffix() {
        // nonce 10 = 0x0a
        let result = parse_token_transfers_arg(&["NFT-abc123-0a".to_string(), "1".to_string()]);
        let payments = result.expect("should succeed");
        assert_eq!(payments.len(), 1);
        assert_eq!(payments.get(0).token_nonce, 10);
        assert_eq!(
            payments.get(0).token_identifier,
            TokenId::<StaticApi>::from(b"NFT-abc123" as &[u8])
        );
    }

    #[test]
    fn token_transfers_multiple_pairs() {
        let result = parse_token_transfers_arg(&[
            "USDC-350c4e".to_string(),
            "10000".to_string(),
            "EGLD-000000".to_string(),
            "1000000000000000000".to_string(),
        ]);
        assert_eq!(result.expect("should succeed").len(), 2);
    }

    // ── parse_payments ────────────────────────────────────────────────────────

    #[test]
    fn payments_two_args_is_error() {
        let result = parse_payments_arg(&["TOKEN-abc".to_string(), "0".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("multiple of 3"));
    }

    #[test]
    fn payments_invalid_nonce_is_error() {
        let result = parse_payments_arg(&[
            "TOKEN-abc".to_string(),
            "not_a_nonce".to_string(),
            "100".to_string(),
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn payments_invalid_amount_is_error() {
        let result = parse_payments_arg(&[
            "TOKEN-abc".to_string(),
            "0".to_string(),
            "not_an_amount".to_string(),
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn payments_zero_amount_is_error() {
        let result =
            parse_payments_arg(&["USDC-350c4e".to_string(), "0".to_string(), "0".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn payments_single_fungible() {
        let result = parse_payments_arg(&[
            "USDC-350c4e".to_string(),
            "0".to_string(),
            "10000".to_string(),
        ]);
        let payments = result.expect("should succeed");
        assert_eq!(payments.len(), 1);
        assert_eq!(payments.get(0).token_nonce, 0);
        assert_eq!(
            payments.get(0).token_identifier,
            TokenId::<StaticApi>::from(b"USDC-350c4e" as &[u8])
        );
    }

    #[test]
    fn payments_nft_with_explicit_nonce() {
        let result =
            parse_payments_arg(&["NFT-abc123".to_string(), "42".to_string(), "3".to_string()]);
        let payments = result.expect("should succeed");
        assert_eq!(payments.len(), 1);
        assert_eq!(payments.get(0).token_nonce, 42);
        assert_eq!(
            payments.get(0).token_identifier,
            TokenId::<StaticApi>::from(b"NFT-abc123" as &[u8])
        );
    }

    #[test]
    fn payments_multiple_triples() {
        let result = parse_payments_arg(&[
            "USDC-350c4e".to_string(),
            "0".to_string(),
            "10000".to_string(),
            "EGLD-000000".to_string(),
            "0".to_string(),
            "1000000000000000000".to_string(),
        ]);
        assert_eq!(result.expect("should succeed").len(), 2);
    }

    // ── build_payments ────────────────────────────────────────────────────────

    #[test]
    fn build_egld_value_only() {
        let args = PaymentArgs {
            value: 1_000_000_000_000_000_000,
            token_transfers: vec![],
            payments: vec![],
        };
        let payments = parse_all_payment_args(&args).expect("should succeed");
        assert_eq!(payments.len(), 1);
        assert_eq!(payments.get(0).token_nonce, 0);
        assert_eq!(
            payments.get(0).token_identifier,
            TokenId::<StaticApi>::native()
        );
    }

    #[test]
    fn build_zero_value_no_transfers_is_empty() {
        let args = PaymentArgs {
            value: 0,
            token_transfers: vec![],
            payments: vec![],
        };
        let payments = parse_all_payment_args(&args).expect("should succeed");
        assert_eq!(payments.len(), 0);
    }

    #[test]
    fn build_token_transfers_plus_egld_appended_last() {
        let args = PaymentArgs {
            value: 1_000_000_000_000_000_000,
            token_transfers: vec!["USDC-350c4e".to_string(), "10000".to_string()],
            payments: vec![],
        };
        let payments = parse_all_payment_args(&args).expect("should succeed");
        assert_eq!(payments.len(), 2);
        // EGLD appended last
        assert_eq!(
            payments.get(1).token_identifier,
            TokenId::<StaticApi>::native()
        );
    }

    #[test]
    fn build_token_transfers_and_payments_combined() {
        let args = PaymentArgs {
            value: 0,
            token_transfers: vec!["USDC-350c4e".to_string(), "10000".to_string()],
            payments: vec!["NFT-abc123".to_string(), "5".to_string(), "1".to_string()],
        };
        let payments = parse_all_payment_args(&args).expect("should succeed");
        assert_eq!(payments.len(), 2);
    }

    #[test]
    fn build_propagates_inner_error() {
        // Odd token-transfers count should bubble up as an error
        let args = PaymentArgs {
            value: 0,
            token_transfers: vec!["USDC-350c4e".to_string()],
            payments: vec![],
        };
        assert!(parse_all_payment_args(&args).is_err());
    }
}
