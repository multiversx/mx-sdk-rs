/// Error type returned by token identifier validation functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenIdValidationError {
    /// Token identifier length is outside the allowed range.
    InvalidLength,
    /// Ticker contains a character that is not uppercase alphanumeric.
    InvalidTickerChar,
    /// The dash separator is missing or not at the expected position.
    MissingOrMisplacedDash,
    /// The random suffix contains a character outside `[0-9a-f]`.
    InvalidRandomChar,
}

const TICKER_MIN_LENGTH: usize = 3;
const TICKER_MAX_LENGTH: usize = 10;
const ADDITIONAL_RANDOM_CHARS_LENGTH: usize = 6;

// +1 because of the '-' (dash) between ticker and the random chars
pub const IDENTIFIER_MIN_LENGTH: usize = TICKER_MIN_LENGTH + ADDITIONAL_RANDOM_CHARS_LENGTH + 1;
pub const IDENTIFIER_MAX_LENGTH: usize = TICKER_MAX_LENGTH + ADDITIONAL_RANDOM_CHARS_LENGTH + 1;

const DASH_CHARACTER: u8 = b'-';

/// Validates the ticker portion of a token identifier (uppercase alphanumeric).
///
/// Expects `ticker` to be the slice before the dash.
fn validate_ticker(ticker: &[u8]) -> Result<(), TokenIdValidationError> {
    let uppercase_letter_range = b'A'..=b'Z';
    let number_range = b'0'..=b'9';

    for ticker_char in ticker {
        let is_uppercase_letter = uppercase_letter_range.contains(ticker_char);
        let is_number = number_range.contains(ticker_char);

        if !is_uppercase_letter && !is_number {
            return Err(TokenIdValidationError::InvalidTickerChar);
        }
    }

    Ok(())
}

/// Validates the random suffix of a token identifier (lowercase hex digits `[0-9a-f]`).
///
/// The suffix is hex-encoded from 3 bytes, so only `[0-9a-f]{6}` is valid.
fn validate_random_chars(random_chars: &[u8]) -> Result<(), TokenIdValidationError> {
    let hex_letter_range = b'a'..=b'f';
    let number_range = b'0'..=b'9';

    for rand_char in random_chars {
        let is_hex_letter = hex_letter_range.contains(rand_char);
        let is_number = number_range.contains(rand_char);

        if !is_hex_letter && !is_number {
            return Err(TokenIdValidationError::InvalidRandomChar);
        }
    }

    Ok(())
}

/// There is a VM implementation of this very function.
/// This implementation is used for debugging mode and backwards compatibility.
/// Using the VM implementation instead of this one saves ~0.6 kB of contract code.
pub fn validate_token_identifier(token_id_slice: &[u8]) -> Result<(), TokenIdValidationError> {
    let length = token_id_slice.len();

    #[allow(clippy::manual_range_contains)]
    if length < IDENTIFIER_MIN_LENGTH || length > IDENTIFIER_MAX_LENGTH {
        return Err(TokenIdValidationError::InvalidLength);
    }

    // The dash must be at the fixed position separating the ticker from the 6-char suffix.
    let dash_pos = get_token_ticker_len(length);
    if token_id_slice[dash_pos] != DASH_CHARACTER {
        return Err(TokenIdValidationError::MissingOrMisplacedDash);
    }

    validate_ticker(&token_id_slice[..dash_pos])?;

    validate_random_chars(&token_id_slice[(dash_pos + 1)..])
}

pub fn get_token_ticker_len(token_id_len: usize) -> usize {
    token_id_len - ADDITIONAL_RANDOM_CHARS_LENGTH - 1
}

#[cfg(test)]
mod tests {
    use super::{TokenIdValidationError, validate_token_identifier};

    fn valid(token_id: &str) {
        assert_eq!(validate_token_identifier(token_id.as_bytes()), Ok(()));
    }

    fn invalid(token_id: &str, expected_err: TokenIdValidationError) {
        assert_eq!(
            validate_token_identifier(token_id.as_bytes()),
            Err(expected_err)
        );
    }

    #[test]
    fn test_valid_identifiers() {
        valid("ALC-6258d2");
        valid("ALC123-6258d2");
        valid("12345-6258d2");
        // max-length ticker (10 chars)
        valid("EGLDRIDEFL-08d8ef");
        // number at end of max-length ticker
        valid("EGLDRIDEF2-08d8ef");
    }

    #[test]
    fn test_invalid_length() {
        // total too short: ticker 0 chars
        invalid("-6258d2", TokenIdValidationError::InvalidLength);
        // total too short: ticker 2 chars
        invalid("AL-6258d2", TokenIdValidationError::InvalidLength);
        // total too short: no dash, 9 chars
        invalid("ALC6258d2", TokenIdValidationError::InvalidLength);
        // total too long: ticker 11 chars
        invalid("ALCCCCCCCCC-6258d2", TokenIdValidationError::InvalidLength);
        // total too long: suffix 7 chars
        invalid("EGLDRIDEFL-08d8eff", TokenIdValidationError::InvalidLength);
    }

    #[test]
    fn test_missing_or_misplaced_dash() {
        // no dash at expected position (valid length, but no dash at pos 3)
        invalid("ABCX6258d2", TokenIdValidationError::MissingOrMisplacedDash);
        // dash too early (valid length 10, dash at pos 2, expected at pos 3)
        invalid("AL-C6258d2", TokenIdValidationError::MissingOrMisplacedDash);
        // dash too early (valid length 12, dash at pos 3, expected at pos 5)
        invalid(
            "ALC-6258d2ff",
            TokenIdValidationError::MissingOrMisplacedDash,
        );
        // dash too late (valid length 16, dash at pos 10, expected at pos 9)
        invalid(
            "EGLDRIDEFL-08d8e",
            TokenIdValidationError::MissingOrMisplacedDash,
        );
    }

    #[test]
    fn test_invalid_ticker_char() {
        // all lowercase ticker
        invalid("alc-6258d2", TokenIdValidationError::InvalidTickerChar);
        // lowercase char in ticker
        invalid(
            "EGLDRIDEFl-08d8ef",
            TokenIdValidationError::InvalidTickerChar,
        );
        // special char in ticker
        invalid(
            "EGLDRIDEF*-08d8ef",
            TokenIdValidationError::InvalidTickerChar,
        );
        // extra dash inside the ticker (dash at expected pos, but ticker contains a dash)
        // "AL-C-6258d2" (11 chars): dash_pos=4, ticker="AL-C" → InvalidTickerChar
        invalid("AL-C-6258d2", TokenIdValidationError::InvalidTickerChar);
        // "ALC--6258d2" (11 chars): dash_pos=4, ticker="ALC-" → InvalidTickerChar
        invalid("ALC--6258d2", TokenIdValidationError::InvalidTickerChar);
        // identifier starting with a dash: dash_pos=3, ticker="-LC" → InvalidTickerChar
        invalid("-LC-6258d2", TokenIdValidationError::InvalidTickerChar);
    }

    #[test]
    fn test_invalid_random_char() {
        // uppercase char in random part
        invalid("ALC-6258D2", TokenIdValidationError::InvalidRandomChar);
        invalid(
            "EGLDRIDEFL-08d8eF",
            TokenIdValidationError::InvalidRandomChar,
        );
        // special char in random part
        invalid(
            "EGLDRIDEFL-08d*ef",
            TokenIdValidationError::InvalidRandomChar,
        );
        // non-hex lowercase letters (g-z) in random part
        invalid("ABC-ghijkl", TokenIdValidationError::InvalidRandomChar);
        invalid("ALC-6258g2", TokenIdValidationError::InvalidRandomChar);
        invalid("ALC-zzzzzz", TokenIdValidationError::InvalidRandomChar);
        // dash in the random part (dash at expected pos, suffix contains a dash)
        // "ALC-625-d2" (10 chars): dash_pos=3, ticker="ALC" ✓, suffix="625-d2" → InvalidRandomChar
        invalid("ALC-625-d2", TokenIdValidationError::InvalidRandomChar);
    }
}
