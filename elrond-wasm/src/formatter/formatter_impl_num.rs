use super::SCDisplay;

const MINUS_SYMBOL: &[u8] = b"-";

fn format_unsigned_rec<F: super::FormatByteReceiver>(num: u64, f: &mut F) {
    if num > 0 {
        format_unsigned_rec(num / 10, f);
        let last_digit = (num % 10) as u8;
        let ascii_last_digit = b'0' + last_digit;
        f.append_bytes(&[ascii_last_digit][..]);
    }
}

fn format_unsigned<F: super::FormatByteReceiver>(num: u64, f: &mut F) {
    if num == 0 {
        f.append_bytes(&b"0"[..]);
    } else {
        format_unsigned_rec(num, f);
    }
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
    if num < 0 {
        f.append_bytes(MINUS_SYMBOL);
        format_unsigned_rec((-num) as u64, f);
    } else {
        format_unsigned(num as u64, f);
    }
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
