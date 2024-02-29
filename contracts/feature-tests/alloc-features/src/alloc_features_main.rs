#![no_std]
#![feature(never_type)]

multiversx_sc::imports!();

pub mod echo_alloc;
pub mod echo_managed_alloc;
pub mod elliptic_curve_features_legacy;
pub mod macro_features_legacy;
pub mod managed_buffer_features_alloc;
pub mod storage_direct_load_alloc;
pub mod storage_direct_store_alloc;
pub mod type_features_alloc;
pub mod types;

/// Features of the framework/VM that use the heap allocator.
///
/// They mostly revolve around types that explicitly allocate on the heap: `Vec`, `BoxedBytes`, `String`, etc.
///
/// Also some legacy/deprecated features still preserved here:
/// - some will be removed,
/// - some will be kept to provide test coverage for otherwise unused VM endpoints.
#[multiversx_sc::contract]
pub trait AllocFeatures:
    echo_alloc::EchoAllocTypes
    + echo_managed_alloc::EchoManagedTypesWithAlloc
    + elliptic_curve_features_legacy::EllipticCurveFeatures
    + macro_features_legacy::MacroFeaturesLegacy
    + managed_buffer_features_alloc::ManagedBufferFeatures
    + storage_direct_load_alloc::StorageLoadFeatures
    + storage_direct_store_alloc::StorageStoreFeatures
    + type_features_alloc::AllocTypeFeatures
{
    #[init]
    fn init(&self) {}
}
