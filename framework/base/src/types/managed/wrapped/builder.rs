mod managed_buffer_builder;
mod managed_buffer_builder_impl;
mod managed_buffer_builder_impl_basic;
mod managed_buffer_builder_impl_cached;

pub use managed_buffer_builder::ManagedBufferBuilder;
pub use managed_buffer_builder_impl::ManagedBufferBuilderImpl;
pub use managed_buffer_builder_impl_basic::ManagedBufferBuilderImplBasic;
pub use managed_buffer_builder_impl_cached::ManagedBufferBuilderImplCached;

#[deprecated(since = "0.48.0", note = "Renamed to ManagedBufferBuilder.")]
pub type ManagedBufferCachedBuilder<M> = ManagedBufferBuilder<M>;

#[cfg(feature = "managed-buffer-builder-cached")]
pub type ManagedBufferImplDefault<M> = ManagedBufferBuilderImplCached<M>;

#[cfg(not(feature = "managed-buffer-builder-cached"))]
pub type ManagedBufferImplDefault<M> = ManagedBufferBuilderImplBasic<M>;
