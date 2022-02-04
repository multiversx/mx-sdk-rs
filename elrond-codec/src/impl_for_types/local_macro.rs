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
            fn dep_encode_or_handle_err<O, H>(
                &self,
                dest: &mut O,
                _h: H,
            ) -> Result<(), H::HandledErr>
            where
                O: NestedEncodeOutput,
                H: EncodeErrorHandler,
            {
                self.dep_encode_no_err(dest);
                Ok(())
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
            fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
            where
                O: TopEncodeOutput,
                H: EncodeErrorHandler,
            {
                self.top_encode_no_err(output);
                Ok(())
            }
        }
    };
}
