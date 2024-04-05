mod managed_buffer_builder;
mod managed_buffer_builder_impl;
mod managed_buffer_builder_impl_cached;

pub use managed_buffer_builder::ManagedBufferBuilder;
pub use managed_buffer_builder_impl::ManagedBufferBuilderImpl;
pub use managed_buffer_builder_impl_cached::ManagedBufferBuilderImplCached;

pub type ManagedBufferCachedBuilder<M> = ManagedBufferBuilder<M>;

pub type ManagedBufferImplDefault<M> = ManagedBufferBuilderImplCached<M>;
