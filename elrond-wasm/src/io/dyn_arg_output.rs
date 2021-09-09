use crate::api::{ErrorApi, ManagedTypeApi};
use elrond_codec::{TopEncode, TopEncodeOutput};

pub trait DynArgOutput {
    fn push_single_arg<T: TopEncode>(&mut self, arg: T);
}
