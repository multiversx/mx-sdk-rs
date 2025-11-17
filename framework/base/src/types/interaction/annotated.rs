mod annotated_impl_big_uint;
mod annotated_impl_managed_address;
mod annotated_impl_managed_buffer;
mod annotated_impl_time;
mod annotated_impl_token_identifier;
mod annotated_impl_u64;

use crate::{
    api::ManagedTypeApi,
    formatter::FormatBuffer,
    types::{ManagedBuffer, ManagedBufferCachedBuilder},
};

use super::TxEnv;

/// Describes a value can also have a custom representation in a mandos scenario.
///
/// It is based on managed types in order to be embedded into parametric tests too.
pub trait AnnotatedValue<Env, T>: Sized
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api>;

    /// Produces the value from a reference of the annotated type. Might involve a `.clone()` in some cases.
    fn to_value(&self, env: &Env) -> T;

    /// Consumes annotated value to produce actual value.
    ///
    /// Override whenever it helps to avoid an unnecessary clone.
    fn into_value(self, env: &Env) -> T {
        self.to_value(env)
    }

    /// Can be used when working with references only.
    ///
    /// Override whenever it helps to avoid an unnecessary clone.
    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.to_value(env))
    }
}

/// Useful for u64 display in several places.
pub(super) fn display_u64<Api>(n: u64) -> ManagedBuffer<Api>
where
    Api: ManagedTypeApi,
{
    let mut result = ManagedBufferCachedBuilder::new_from_slice(&[]);
    result.append_display(&n);
    result.into_managed_buffer()
}
