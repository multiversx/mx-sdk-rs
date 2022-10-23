// Derive the implementation of the other types by casting.
#[macro_export]
macro_rules! dep_encode_num_mimic {
    ($num_type:ty, $mimic_type:ident) => {
        impl NestedEncode for $num_type {
            #[inline]
            fn dep_encode_or_handle_err<O, H>(
                &self,
                dest: &mut O,
                h: H,
            ) -> Result<(), H::HandledErr>
            where
                O: NestedEncodeOutput,
                H: EncodeErrorHandler,
            {
                (*self as $mimic_type).dep_encode_or_handle_err(dest, h)
            }
        }
    };
}
