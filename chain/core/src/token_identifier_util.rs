const TICKER_MIN_LENGTH: usize = 3;
const TICKER_MAX_LENGTH: usize = 10;
const ADDITIONAL_RANDOM_CHARS_LENGTH: usize = 6;

// +1 because of the '-' (dash) between ticker and the random chars
pub const IDENTIFIER_MIN_LENGTH: usize = TICKER_MIN_LENGTH + ADDITIONAL_RANDOM_CHARS_LENGTH + 1;
pub const IDENTIFIER_MAX_LENGTH: usize = TICKER_MAX_LENGTH + ADDITIONAL_RANDOM_CHARS_LENGTH + 1;

const DASH_CHARACTER: u8 = b'-';

/// There is a VM implementation of this very function.
/// This implementation is used for debugging mode and backwards compatibility.
/// Using the VM implementation instead of this one saves ~0.6 kB of contract code.
pub fn validate_token_identifier(token_id_slice: &[u8]) -> bool {
    let length = token_id_slice.len();

    #[allow(clippy::manual_range_contains)]
    if length < IDENTIFIER_MIN_LENGTH || length > IDENTIFIER_MAX_LENGTH {
        return false;
    }

    let lowercase_letter_range = b'a'..=b'z';
    let uppercase_letter_range = b'A'..=b'Z';
    let number_range = b'0'..=b'9';

    // ticker must be all uppercase alphanumeric
    let ticker_len = get_token_ticker_len(length);
    let ticker = &token_id_slice[..ticker_len];
    for ticker_char in ticker {
        let is_uppercase_letter = uppercase_letter_range.contains(ticker_char);
        let is_number = number_range.contains(ticker_char);

        if !is_uppercase_letter && !is_number {
            return false;
        }
    }

    let dash_position = ticker_len;
    if token_id_slice[dash_position] != DASH_CHARACTER {
        return false;
    }

    // random chars are alphanumeric lowercase
    let random_chars = &token_id_slice[(length - ADDITIONAL_RANDOM_CHARS_LENGTH)..];
    for rand_char in random_chars {
        let is_lowercase_letter = lowercase_letter_range.contains(rand_char);
        let is_number = number_range.contains(rand_char);

        if !is_lowercase_letter && !is_number {
            return false;
        }
    }

    true
}

pub fn get_token_ticker_len(token_id_len: usize) -> usize {
    token_id_len - ADDITIONAL_RANDOM_CHARS_LENGTH - 1
}
