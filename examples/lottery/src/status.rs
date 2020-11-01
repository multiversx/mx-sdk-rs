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
            _ => core::result::Result::Err(DecodeError::INVALID_VALUE),
        }
    }
}

impl TopEncode for Status {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.to_u8().top_encode(output)
    }
}

impl TopDecode for Status {
    fn top_decode<I: TopDecodeInput, R, F: FnOnce(Result<Self, DecodeError>) -> R>(input: I, f: F) -> R {
        u8::top_decode(input, |res| match res {
            core::result::Result::Ok(num) => f(Status::from_u8(num)),
            core::result::Result::Err(e) => f(core::result::Result::Err(e)),
        })
    }
}
