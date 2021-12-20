use elrond_codec::TryStaticCast;

use crate::api::ErrorApi;

use super::{BigFloatApi, BigIntApi, EllipticCurveApi, ManagedBufferApi, StaticBufferApi};

pub type Handle = i32;

pub trait ManagedTypeApi:
    TryStaticCast
    + BigFloatApi
    + BigIntApi
    + EllipticCurveApi
    + ManagedBufferApi
    + StaticBufferApi
    + ErrorApi
    + Clone
    + 'static
{
    const TICKER_MIN_LENGTH: usize = 3;
    const TICKER_MAX_LENGTH: usize = 10;
    const ADDITIONAL_RANDOM_CHARS_LENGTH: usize = 6;
    // +1 because of the '-' (dash) between ticker and the random chars
    const IDENTIFIER_MIN_LENGTH: usize =
        Self::TICKER_MIN_LENGTH + Self::ADDITIONAL_RANDOM_CHARS_LENGTH + 1;
    const IDENTIFIER_MAX_LENGTH: usize =
        Self::TICKER_MAX_LENGTH + Self::ADDITIONAL_RANDOM_CHARS_LENGTH + 1;

    const DASH_CHARACTER: u8 = b'-';

    fn instance() -> Self;

    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle;

    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle;

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle;

    fn mb_to_big_float(&self, buffer_handle: Handle) -> Handle;

    fn mb_from_big_float(&self, big_float_handle: Handle) -> Handle;

    fn mb_overwrite_static_buffer(&self, buffer_handle: Handle) -> bool {
        self.with_lockable_static_buffer(|lsb| {
            let len = self.mb_len(buffer_handle);
            lsb.try_lock_with_copy_bytes(len, |dest| {
                let _ = self.mb_load_slice(buffer_handle, 0, dest);
            })
        })
    }

    fn append_mb_to_static_buffer(&self, buffer_handle: Handle) -> bool {
        self.with_lockable_static_buffer(|lsb| {
            let len = self.mb_len(buffer_handle);
            lsb.try_extend_from_copy_bytes(len, |dest| {
                let _ = self.mb_load_slice(buffer_handle, 0, dest);
            })
        })
    }

    fn append_static_buffer_to_mb(&self, buffer_handle: Handle) {
        self.with_lockable_static_buffer(|lsb| {
            self.mb_append_bytes(buffer_handle, lsb.as_slice());
        });
    }

    fn validate_token_identifier(&self, token_id_handle: Handle) -> bool {
        let token_id_bytes = self.mb_to_boxed_bytes(token_id_handle);
        let id_len = token_id_bytes.len();

        #[allow(clippy::manual_range_contains)]
        if id_len < Self::IDENTIFIER_MIN_LENGTH || id_len > Self::IDENTIFIER_MAX_LENGTH {
            return false;
        }

        let token_id_slice = token_id_bytes.as_slice();

        let lowercase_letter_range = &b'a'..=&b'z';
        let uppercase_letter_range = &b'A'..=&b'Z';
        let number_range = &b'0'..=&b'9';

        // ticker must be all uppercase alphanumeric
        let ticker_len = id_len - Self::ADDITIONAL_RANDOM_CHARS_LENGTH - 1;
        let ticker = &token_id_slice[..ticker_len];
        for ticker_char in ticker {
            let is_uppercase_letter = uppercase_letter_range.contains(&ticker_char);
            let is_number = number_range.contains(&ticker_char);

            if !is_uppercase_letter && !is_number {
                return false;
            }
        }

        let dash_position = ticker_len;
        if token_id_slice[dash_position] != Self::DASH_CHARACTER {
            return false;
        }

        // random chars are alphanumeric lowercase
        let random_chars = &token_id_slice[(id_len - Self::ADDITIONAL_RANDOM_CHARS_LENGTH)..];
        for rand_char in random_chars {
            let is_lowercase_letter = lowercase_letter_range.contains(&rand_char);
            let is_number = number_range.contains(&rand_char);

            if !is_lowercase_letter && !is_number {
                return false;
            }
        }

        true
    }
}
