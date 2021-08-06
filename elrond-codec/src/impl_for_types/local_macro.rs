// Derive the implementation of the other types by casting.
#[macro_export]
macro_rules! dep_encode_num_mimic {
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

#[macro_export]
macro_rules! top_encode_from_no_err {
    ($type:ty, $type_info:expr) => {
        impl TopEncode for $type {
            const TYPE_INFO: TypeInfo = $type_info;

            #[inline]
            fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
                self.top_encode_no_err(output);
                Ok(())
            }

            #[inline]
            fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
                &self,
                output: O,
                _: ExitCtx,
                _: fn(ExitCtx, EncodeError) -> !,
            ) {
                self.top_encode_no_err(output);
            }
        }
    };
}
