use crate::{
    api::ManagedTypeApi,
    codec::{self},
    types::{ManagedBuffer, TokenIdentifier},
};

const PENDING_ENCODING: &[u8; 7] = b"pending";

#[derive(Default, Clone)]
pub enum TokenMapperState<'a, M: ManagedTypeApi<'a>> {
    #[default]
    NotSet,
    Pending,
    Token(TokenIdentifier<'a, M>),
}

impl<'a, M: ManagedTypeApi<'a>> TokenMapperState<'a, M> {
    pub fn is_set(&self) -> bool {
        matches!(self, TokenMapperState::Token(_))
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, TokenMapperState::Pending)
    }

    pub fn is_not_set(&self) -> bool {
        matches!(self, TokenMapperState::NotSet)
    }

    pub fn is_not_available(&self) -> bool {
        matches!(self, TokenMapperState::Pending | TokenMapperState::NotSet)
    }
}

impl<'a, M: ManagedTypeApi<'a>> codec::TopEncode for TokenMapperState<'a, M> {
    fn top_encode_or_handle_err<O, H>(
        &self,
        output: O,
        h: H,
    ) -> core::result::Result<(), H::HandledErr>
    where
        O: codec::TopEncodeOutput,
        H: codec::EncodeErrorHandler,
    {
        match self {
            TokenMapperState::NotSet => codec::TopEncode::top_encode_or_handle_err(&"", output, h),
            TokenMapperState::Pending => {
                codec::TopEncode::top_encode_or_handle_err(&"pending", output, h)
            },
            TokenMapperState::Token(token) => {
                codec::TopEncode::top_encode_or_handle_err(&token, output, h)
            },
        }
    }
}

impl<'a, M: ManagedTypeApi<'a>> codec::TopDecode for TokenMapperState<'a, M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> core::result::Result<Self, H::HandledErr>
    where
        I: codec::TopDecodeInput,
        H: codec::DecodeErrorHandler,
    {
        let decoded_input = ManagedBuffer::top_decode_or_handle_err(input, h)?;
        if decoded_input.is_empty() {
            Ok(TokenMapperState::NotSet)
        } else if decoded_input == PENDING_ENCODING {
            Ok(TokenMapperState::Pending)
        } else {
            let token_id = TokenIdentifier::from_esdt_bytes(decoded_input);
            Ok(TokenMapperState::Token(token_id))
        }
    }
}
