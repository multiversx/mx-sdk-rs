use crate::codec_err::EncodeError;
use crate::top_encode_from_no_err;
use crate::top_ser::TopEncode;
use crate::top_ser::TopEncodeNoErr;
use crate::top_ser_output::TopEncodeOutput;
use crate::TypeInfo;

macro_rules! encode_num_signed {
    ($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
        impl TopEncodeNoErr for $num_type {
            #[inline]
            fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
                output.set_i64(*self as i64);
            }
        }

        top_encode_from_no_err! {$num_type, $type_info}
    };
}

encode_num_signed! {i64, 64, TypeInfo::I64}
encode_num_signed! {i32, 32, TypeInfo::I32}
encode_num_signed! {isize, 32, TypeInfo::ISIZE}
encode_num_signed! {i16, 16, TypeInfo::I16}
encode_num_signed! {i8, 8, TypeInfo::I8}
