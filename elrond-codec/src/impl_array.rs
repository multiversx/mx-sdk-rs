use crate::codec_err::{DecodeError, EncodeError};
use crate::nested_de::NestedDecode;
use crate::nested_de_input::NestedDecodeInput;
use crate::nested_ser::{
	dep_encode_slice_contents, dep_encode_slice_contents_or_exit, NestedEncode,
};
use crate::nested_ser_output::NestedEncodeOutput;
use crate::top_de::{top_decode_from_nested, top_decode_from_nested_or_exit, TopDecode};
use crate::top_de_input::TopDecodeInput;
use crate::top_ser::TopEncode;
use crate::top_ser_output::TopEncodeOutput;
use crate::TypeInfo;
use alloc::boxed::Box;
use arrayvec::ArrayVec;

macro_rules! array_impls {
    ($($n: tt,)+) => {
        $(
			impl<T: NestedEncode> NestedEncode for [T; $n] {
				#[inline]
				fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
					dep_encode_slice_contents(&self[..], dest)
				}

				#[inline]
				fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(&self, dest: &mut O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
					dep_encode_slice_contents_or_exit(&self[..], dest, c, exit);
				}
			}

			impl<T: NestedEncode> TopEncode for [T; $n] {
				#[inline]
				fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
					// the top encoded slice does not serialize its length, so just like the array
					(&self[..]).top_encode(output)
				}

				#[inline]
				fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
					(&self[..]).top_encode_or_exit(output, c, exit);
				}
            }

			impl<T: NestedDecode> NestedDecode for [T; $n] {
                #[allow(clippy::reversed_empty_ranges)]
            	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
					let mut r = ArrayVec::new();
					for _ in 0..$n {
						r.push(T::dep_decode(input)?);
					}
					let i = r.into_inner();

					match i {
						Ok(a) => Ok(a),
						Err(_) => Err(DecodeError::ARRAY_DECODE_ERROR),
					}
                }

                #[allow(clippy::reversed_empty_ranges)]
            	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(input: &mut I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
                    let mut r = ArrayVec::new();
					for _ in 0..$n {
						r.push(T::dep_decode_or_exit(input, c.clone(), exit));
					}
					let i = r.into_inner();

					match i {
						Ok(a) => a,
						Err(_) => exit(c, DecodeError::ARRAY_DECODE_ERROR),
					}
                }
            }

			impl<T: NestedDecode> TopDecode for [T; $n] {
                fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
                    top_decode_from_nested(input)
                }

                fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(input: I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
                    top_decode_from_nested_or_exit(input, c, exit)
                }

                fn top_decode_boxed<I: TopDecodeInput>(input: I) -> Result<Box<Self>, DecodeError> {
                    if let TypeInfo::U8 = T::TYPE_INFO {
                        // transmute directly
                        let bs = input.into_boxed_slice_u8();
                        if bs.len() != $n {
                            return Err(DecodeError::ARRAY_DECODE_ERROR);
                        }
                        let raw = Box::into_raw(bs);
                        let array_box = unsafe { Box::<[T; $n]>::from_raw(raw as *mut [T; $n]) };
                        Ok(array_box)
                    } else {
                        Ok(Box::new(Self::top_decode(input)?))
                    }
                }

                fn top_decode_boxed_or_exit<I: TopDecodeInput, ExitCtx: Clone>(input: I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Box<Self> {
                    if let TypeInfo::U8 = T::TYPE_INFO {
                        // transmute directly
                        let bs = input.into_boxed_slice_u8();
                        if bs.len() != $n {
                            exit(c, DecodeError::ARRAY_DECODE_ERROR);
                        }
                        let raw = Box::into_raw(bs);
                        let array_box = unsafe { Box::<[T; $n]>::from_raw(raw as *mut [T; $n]) };
                        array_box
                    } else {
                        Box::new(Self::top_decode_or_exit(input, c, exit))
                    }
                }
            }
        )+
    }
}

#[rustfmt::skip]
array_impls!(
	0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
	17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
	32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
	52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
	72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
	92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
	109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
	125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140,
	141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156,
	157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
	173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188,
	189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204,
	205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
	221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236,
	237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252,
	253, 254, 255, 256, 384, 512, 768, 1024, 2048, 4096, 8192, 16384, 32768,
);
