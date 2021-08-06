use crate::codec_err::EncodeError;
use crate::dep_encode_from_no_err;
use crate::nested_ser::NestedEncode;
use crate::nested_ser::NestedEncodeNoErr;
use crate::nested_ser_output::NestedEncodeOutput;
use crate::TypeInfo;

// Derive the implementation of the other types by casting.
#[macro_export]
macro_rules! encode_num_mimic {
    ($num_type:ty, $mimic_type:ident, $type_info:expr) => {
        impl NestedEncodeNoErr for $num_type {
            #[inline]
            fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
                (*self as $mimic_type).dep_encode_no_err(dest)
            }
        }

        dep_encode_from_no_err! {$num_type, $type_info}
    };
}

encode_num_mimic! {usize, u32, TypeInfo::USIZE}
encode_num_mimic! {i64, u64, TypeInfo::I64}
encode_num_mimic! {i32, u32, TypeInfo::I32}
encode_num_mimic! {isize, u32, TypeInfo::ISIZE}
encode_num_mimic! {i16, u16, TypeInfo::I16}
encode_num_mimic! {i8, u8, TypeInfo::I8}
