use core::marker::PhantomData;

use crate::{
    api::CallTypeApi,
    types::{ManagedAddress, ManagedBuffer},
};

use super::AnnotatedValue;

pub trait TxEnv: Sized {
    type Api: CallTypeApi;

    fn annotate_from<From>(&mut self, from: &From)
    where
        From: AnnotatedValue<Self, ManagedAddress<Self::Api>>;

    fn annotate_to<To>(&mut self, to: &To)
    where
        To: AnnotatedValue<Self, ManagedAddress<Self::Api>>;

    fn resolve_sender_address(&self) -> ManagedAddress<Self::Api>;

    fn default_gas(&self) -> u64;
}

// pub struct TxTestingEnvironemnt<Api>
// where
//     Api: CallTypeApi,
// {
//     pub(super) _phantom: PhantomData<Api>,
//     pub from_annotation: ManagedBuffer<Api>,
//     pub to_annotation: ManagedBuffer<Api>,
// }

// impl<Api> TxEnv<Api> for TxTestingEnvironemnt<Api>
// where
//     Api: CallTypeApi,
// {
//     fn annotate_from<From>(&mut self, from: &From)
//     where
//         From: AnnotatedValue<Api, ManagedAddress<Api>>,
//     {
//         self.from_annotation = from.annotation();
//     }

//     fn annotate_to<To>(&mut self, to: &To)
//     where
//         To: AnnotatedValue<Api, ManagedAddress<Api>>,
//     {
//         self.to_annotation = to.annotation();
//     }
// }
