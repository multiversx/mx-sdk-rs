use elrond_wasm::elrond_codec::*;

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    Inactive,
    Running,
    Ended,
}

impl Status {
    pub fn to_u8(&self) -> u8 {
        match self {
            Status::Inactive => 0,
            Status::Running => 1,
            Status::Ended => 2,
        }
    }

    pub fn from_u8(v: u8) -> Result<Self, DecodeError> {
        match v {
            0 => core::result::Result::Ok(Status::Inactive),
            1 => core::result::Result::Ok(Status::Running),
            2 => core::result::Result::Ok(Status::Ended),
            _ => core::result::Result::Err(DecodeError::InvalidValue),
        }
    }
}

impl Encode for Status {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        return self.to_u8().dep_encode_to(dest);
    }
}

impl Decode for Status {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        return Status::from_u8(u8::dep_decode(input)?);
    }
}
