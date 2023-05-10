use crate::{
    api::ManagedTypeApi,
    codec::{self, derive::TopDecode},
    types::TokenIdentifier,
};

#[derive(TopDecode, Default)]
pub enum TokenMapperState<M: ManagedTypeApi> {
    #[default]
    NotSet,
    Pending,
    Token(TokenIdentifier<M>),
}

impl<M: ManagedTypeApi> TokenMapperState<M> {
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

impl<M: ManagedTypeApi> codec::TopEncode for TokenMapperState<M> {
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
