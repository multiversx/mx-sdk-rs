#![no_std]
#![feature(never_type)]

elrond_wasm::imports!();

pub mod big_int_methods;
pub mod big_int_operators;
pub mod block_info_features;
pub mod blockchain_api_features;
pub mod codec_err_test;
pub mod crypto_features;
pub mod echo;
pub mod echo_managed;
pub mod elliptic_curve_features;
pub mod event_features;
pub mod macros;
pub mod managed_buffer_features;
pub mod managed_vec_features;
pub mod storage_direct_load;
pub mod storage_direct_store;
pub mod storage_mapper_linked_list;
pub mod storage_mapper_map;
pub mod storage_mapper_map_storage;
pub mod storage_mapper_queue;
pub mod storage_mapper_set;
pub mod storage_mapper_single;
pub mod storage_mapper_token_attributes;
pub mod storage_mapper_vec;
pub mod type_features;
pub mod types;

#[elrond_wasm::contract]
pub trait BasicFeatures:
    big_int_methods::BigIntMethods
    + big_int_operators::BigIntOperators
    + elliptic_curve_features::EllipticCurveFeatures
    + block_info_features::BlockInfoFeatures
    + blockchain_api_features::BlockchainApiFeatures
    + codec_err_test::CodecErrorTest
    + crypto_features::CryptoFeatures
    + echo::EchoTypes
    + echo_managed::EchoManagedTypes
    + event_features::EventFeatures
    + macros::Macros
    + managed_buffer_features::ManagedBufferFeatures
    + managed_vec_features::ManagedVecFeatures
    + storage_direct_load::StorageLoadFeatures
    + storage_direct_store::StorageStoreFeatures
    + storage_mapper_queue::QueueMapperFeatures
    + storage_mapper_map::MapMapperFeatures
    + storage_mapper_map_storage::MapMapperNestedFeatures
    + storage_mapper_set::SetMapperFeatures
    + storage_mapper_single::SingleValueMapperFeatures
    + storage_mapper_vec::VecMapperFeatures
    + storage_mapper_token_attributes::TokenAttributesMapperFeatures
    + type_features::TypeFeatures
    + storage_mapper_linked_list::LinkedListMapperFeatures
{
    #[init]
    fn init(&self) {}

    #[endpoint(panicWithMessage)]
    fn panic_with_message(&self) {
        panic!("example panic message");
    }

    /// Operation that has caused issues in the past
    #[endpoint]
    fn count_ones(&self, arg: u64) -> u32 {
        arg.count_ones()
    }
}
