use super::SCDisplay;

const MINUS_SYMBOL: &[u8] = b"-";

/// u64::MAX is 18446744073709551615 in base 10, which is 20 digits. so 20 digits is enough
const MAX_BASE_10_LEN: usize = 20;

fn format_unsigned_to_buffer(mut num: u64, buffer: &mut [u8; MAX_BASE_10_LEN]) -> &[u8] {
    let mut buf_index = MAX_BASE_10_LEN;
    if num == 0 {
        buf_index -= 1;
        buffer[buf_index] = b'0';
    } else {
        while num > 0 {
            buf_index -= 1;
            let last_digit = (num % 10) as u8;
            let ascii_last_digit = b'0' + last_digit;
            buffer[buf_index] = ascii_last_digit;
            num /= 10;
        }
    }
    &buffer[buf_index..]
}

fn format_unsigned<F: super::FormatByteReceiver>(num: u64, f: &mut F) {
    let mut buffer = [0u8; MAX_BASE_10_LEN];
    let formatted = format_unsigned_to_buffer(num, &mut buffer);
    f.append_bytes(formatted);
}

macro_rules! formatter_unsigned {
    ($num_ty:ty) => {
        impl SCDisplay for $num_ty {
            #[inline]
            fn fmt<F: super::FormatByteReceiver>(&self, f: &mut F) {
                format_unsigned(*self as u64, f);
            }
        }
    };
}

formatter_unsigned! {u64}
formatter_unsigned! {u32}
formatter_unsigned! {usize}
formatter_unsigned! {u16}
formatter_unsigned! {u8}

fn format_signed<F: super::FormatByteReceiver>(num: i64, f: &mut F) {
    let abs = if num >= 0 {
        num as u64
    } else {
        f.append_bytes(MINUS_SYMBOL);
        if num == i64::MIN {
            // overflow egde case
            (i64::MAX as u64) + 1
        } else {
            (-num) as u64
        }
    };
    format_unsigned(abs, f);
}

macro_rules! formatter_signed {
    ($num_ty:ty) => {
        impl SCDisplay for $num_ty {
            #[inline]
            fn fmt<F: super::FormatByteReceiver>(&self, f: &mut F) {
                format_signed(*self as i64, f);
            }
        }
    };
}

formatter_signed! {i64}
formatter_signed! {i32}
formatter_signed! {isize}
formatter_signed! {i16}
formatter_signed! {i8}
