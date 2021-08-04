use crate::codec_err::EncodeError;
use crate::dep_encode_from_no_err;
use crate::nested_ser::NestedEncode;
use crate::nested_ser::NestedEncodeNoErr;
use crate::nested_ser_output::NestedEncodeOutput;
use crate::top_encode_from_no_err;
use crate::top_ser::TopEncode;
use crate::top_ser::TopEncodeNoErr;
use crate::top_ser_output::TopEncodeOutput;
use crate::TypeInfo;
// The main unsigned types need to be reversed before serializing.
macro_rules! encode_num_unsigned {
    ($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
        impl NestedEncodeNoErr for $num_type {
            #[inline(never)]
            fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
                dest.write(&self.to_be_bytes()[..]);
            }
        }

        dep_encode_from_no_err! {$num_type, $type_info}
    };
}

encode_num_unsigned! {u64, 64, TypeInfo::U64}
encode_num_unsigned! {u32, 32, TypeInfo::U32}
encode_num_unsigned! {u16, 16, TypeInfo::U16}

macro_rules! encode_num_unsigned {
    ($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
        impl TopEncodeNoErr for $num_type {
            #[inline]
            fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
                output.set_u64(*self as u64);
            }
        }
        top_encode_from_no_err! {$num_type, $type_info}
    };
}

encode_num_unsigned! {u64, 64, TypeInfo::U64}
encode_num_unsigned! {u32, 32, TypeInfo::U32}
encode_num_unsigned! {usize, 32, TypeInfo::USIZE}
encode_num_unsigned! {u16, 16, TypeInfo::U16}
encode_num_unsigned! {u8, 8, TypeInfo::U8}
