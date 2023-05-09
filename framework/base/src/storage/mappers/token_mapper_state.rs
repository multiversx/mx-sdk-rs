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
        match self {
            TokenMapperState::Pending => true,
            _ => false,
        }
    }
    pub fn is_not_set(&self) -> bool {
        match self {
            TokenMapperState::NotSet => true,
            _ => false,
        }
    }
    pub fn is_not_available(&self) -> bool {
        match self {
            TokenMapperState::Pending => true,
            TokenMapperState::NotSet => true,
            _ => false,
        }
    }
}
