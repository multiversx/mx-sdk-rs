use elrond_wasm::elrond_codec::*;

/// Copied from elrond-wasm serialization tests. 
pub enum SerExample2 {
    Unit,
    Newtype(u32),
    Tuple(u32, u32),
    Struct { a: u32 },
}

impl Encode for SerExample2 {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        match self {
            SerExample2::Unit => {
                using_encoded_number(0u64, 32, false, false, |buf| dest.write(buf));
            },
            SerExample2::Newtype(arg1) => {
                using_encoded_number(1u64, 32, false, false, |buf| dest.write(buf));
                using_encoded_number(*arg1 as u64, 32, false, false, |buf| dest.write(buf));
            },
            SerExample2::Tuple(arg1, arg2) => {
                using_encoded_number(2u64, 32, false, false, |buf| dest.write(buf));
                using_encoded_number(*arg1 as u64, 32, false, false, |buf| dest.write(buf));
                using_encoded_number(*arg2 as u64, 32, false, false, |buf| dest.write(buf));
            },
            SerExample2::Struct { a } => {
                using_encoded_number(3u64, 32, false, false, |buf| dest.write(buf));
                using_encoded_number(*a as u64, 32, false, false, |buf| dest.write(buf));
            },
        }
        Ok(())
    }
}

impl Decode for SerExample2 {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        match u32::dep_decode(input)? {
            0 => Ok(SerExample2::Unit),
            1 => Ok(SerExample2::Newtype(u32::dep_decode(input)?)),
            2 => Ok(SerExample2::Tuple(u32::dep_decode(input)?, u32::dep_decode(input)?)),
            3 => Ok(SerExample2::Struct{ a: u32::dep_decode(input)? }),
            _ => Err(DecodeError::InvalidValue),
        }
    }
}
