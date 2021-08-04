use crate::codec_err::EncodeError;
use crate::nested_ser::NestedEncode;
use crate::nested_ser::NestedEncodeNoErr;
use crate::nested_ser_output::NestedEncodeOutput;
use crate::TypeInfo;

#[macro_export]
macro_rules! dep_encode_from_no_err {
    ($type:ty, $type_info:expr) => {
        impl NestedEncode for $type {
            const TYPE_INFO: TypeInfo = $type_info;

            #[inline]
            fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
                self.dep_encode_no_err(dest);
                Ok(())
            }

            #[inline]
            fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
                &self,
                dest: &mut O,
                _: ExitCtx,
                _: fn(ExitCtx, EncodeError) -> !,
            ) {
                self.dep_encode_no_err(dest);
            }
        }
    };
}

dep_encode_from_no_err! {(), TypeInfo::Unit}
dep_encode_from_no_err! {u8, TypeInfo::U8}
