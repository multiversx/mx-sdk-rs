use crate::{
    api::ManagedTypeApi,
    codec::{
        self,
        derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    },
    types::TokenIdentifier,
};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Default)]
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
        matches!(self, TokenMapperState::Pending | TokenMapperState::NotSet
    }
}
