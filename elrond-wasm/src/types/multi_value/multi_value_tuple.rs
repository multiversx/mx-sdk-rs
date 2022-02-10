use crate::elrond_codec::{
    DecodeErrorHandler, EncodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput, TopEncodeMulti,
    TopEncodeMultiOutput,
};

macro_rules! multi_value_impls {
    ($(($mv_struct:ident $($n:tt $name:ident)+) )+) => {
        $(
            #[derive(Clone)]
            pub struct $mv_struct<$($name,)+>(pub ($($name,)+));

            impl<$($name),+> From<($($name,)+)> for $mv_struct<$($name,)+> {
                #[inline]
                fn from(tuple: ($($name,)+)) -> Self {
                    $mv_struct(tuple)
                }
            }

            impl<$($name,)+> $mv_struct<$($name,)+> {
                #[inline]
                pub fn into_tuple(self) -> ($($name,)+) {
                    self.0
                }
            }

            impl<$($name),+ > TopEncodeMulti for $mv_struct<$($name,)+>
            where
                $($name: TopEncodeMulti,)+
            {
                type DecodeAs = Self; // TODO: reassemble from component DecodeAs

                fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
                where
                    O: TopEncodeMultiOutput,
                    H: EncodeErrorHandler,
                {
                    $(
                        (self.0).$n.multi_encode_or_handle_err(output, h)?;
                    )+
                    Ok(())
                }
            }

            impl<$($name),+ > TopDecodeMulti for $mv_struct<$($name,)+>
            where
                $($name: TopDecodeMulti,)+
            {
                fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
                where
                    I: TopDecodeMultiInput,
                    H: DecodeErrorHandler,
                {
                    Ok($mv_struct((
                        $(
                            $name::multi_decode_or_handle_err(input, h)?
                        ),+
                    )))
                }
            }

            // impl<$($name),+ > DynArg for $mv_struct<$($name,)+>
            // where
            //     $($name: DynArg,)+
            // {
            //     fn dyn_load<I>(loader: &mut I, arg_id: ArgId) -> Self
            //     where
            //         I: DynArgInput,
            //     {
            //         $mv_struct((
            //             $(
            //                 $name::dyn_load(loader, arg_id)
            //             ),+
            //         ))
            //     }
            // }

            // impl<$($name),+> EndpointResult for $mv_struct<$($name,)+>
            // where
            //     $($name: EndpointResult,)+
            // {
            //     type DecodeAs = Self; // TODO: reassemble from component DecodeAs

            //     #[inline]
			// 	fn finish<FA>(&self)
            //     where
            //         FA: ManagedTypeApi + EndpointFinishApi ,
            //     {
            //         $(
            //             (self.0).$n.finish::<FA>();
            //         )+
            //     }
            // }

            // impl<$($name),+> ContractCallArg for &$mv_struct<$($name,)+>
            // where
            //     $($name: ContractCallArg,)+
            // {
            //     #[inline]
            //     fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
            //         $(
            //             (self.0).$n.push_dyn_arg(output);
            //         )+
            //     }
            // }

            // impl<$($name),+> ContractCallArg for $mv_struct<$($name,)+>
            // where
            //     $($name: ContractCallArg,)+
            // {
            //     fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
            //         ContractCallArg::push_dyn_arg(&self, output)
            //     }
            // }

            // impl<$($name),+ > TypeAbi for $mv_struct<$($name,)+>
            // where
            //     $($name: TypeAbi,)+
            // {
            //     fn type_name() -> String {
            //         let mut repr = String::from("multi");
            //         repr.push('<');
            //         $(
            //             if $n > 0 {
            //                 repr.push(',');
            //             }
            //             repr.push_str($name::type_name().as_str());
            //         )+
            //         repr.push('>');
            //         repr
            //     }

            //     fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
			// 		$(
			// 			$name::provide_type_descriptions(accumulator);
            //         )+
            //     }

            //     fn is_multi_arg_or_result() -> bool {
            //         true
            //     }

            //     fn output_abis(output_names: &[&'static str]) -> Vec<OutputAbi> {
            //         let mut result = Vec::new();
            //         $(
            //             if output_names.len() > $n {
            //                 result.append(&mut $name::output_abis(&[output_names[$n]]));

            //             } else {
            //                 result.append(&mut $name::output_abis(&[]));
            //             }

            //         )+
            //         result
            //     }
            // }


        )+
    }
}

multi_value_impls! {
    (MultiValue2  0 T0 1 T1)
    (MultiValue3  0 T0 1 T1 2 T2)
    (MultiValue4  0 T0 1 T1 2 T2 3 T3)
    (MultiValue5  0 T0 1 T1 2 T2 3 T3 4 T4)
    (MultiValue6  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (MultiValue7  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (MultiValue8  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (MultiValue9  0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (MultiValue10 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (MultiValue11 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (MultiValue12 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (MultiValue13 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (MultiValue14 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (MultiValue15 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (MultiValue16 0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
