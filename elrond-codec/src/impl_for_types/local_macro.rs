// Derive the implementation of the other types by casting.
#[macro_export]
macro_rules! dep_encode_num_mimic {
    ($num_type:ty, $mimic_type:ident, $type_info:expr) => {
        impl NestedEncode for $num_type {
            const TYPE_INFO: TypeInfo = $type_info;

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
