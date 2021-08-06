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
