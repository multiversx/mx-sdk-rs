use core::marker::PhantomData;

use crate::{
    api::CallTypeApi,
    types::{ManagedAddress, ManagedBuffer},
};

use super::AnnotatedValue;

pub trait TxEnvironemnt<Api>
where
    Api: CallTypeApi,
{
    fn annotate_from<From>(&mut self, from: &From)
    where
        From: AnnotatedValue<Api, ManagedAddress<Api>>;

    fn annotate_to<To>(&mut self, to: &To)
    where
        To: AnnotatedValue<Api, ManagedAddress<Api>>;
}

impl<Api> TxEnvironemnt<Api> for ()
where
    Api: CallTypeApi,
{
    fn annotate_from<From>(&mut self, _from: &From)
    where
        From: AnnotatedValue<Api, ManagedAddress<Api>>,
    {
    }

    fn annotate_to<To>(&mut self, _to: &To)
    where
        To: AnnotatedValue<Api, ManagedAddress<Api>>,
    {
    }
}

pub struct TxTestingEnvironemnt<Api>
where
    Api: CallTypeApi,
{
    pub(super) _phantom: PhantomData<Api>,
    pub from_annotation: ManagedBuffer<Api>,
    pub to_annotation: ManagedBuffer<Api>,
}

impl<Api> TxEnvironemnt<Api> for TxTestingEnvironemnt<Api>
where
    Api: CallTypeApi,
{
    fn annotate_from<From>(&mut self, from: &From)
    where
        From: AnnotatedValue<Api, ManagedAddress<Api>>,
    {
        self.from_annotation = from.annotation();
    }

    fn annotate_to<To>(&mut self, to: &To)
    where
        To: AnnotatedValue<Api, ManagedAddress<Api>>,
    {
        self.to_annotation = to.annotation();
    }
}
