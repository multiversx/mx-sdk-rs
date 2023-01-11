#![no_std]
#![feature(never_type)]

multiversx_sc::imports!();

pub mod big_num_methods;
pub mod big_num_operators;
pub mod block_info_features;
pub mod blockchain_api_features;
pub mod codec_err_test;
pub mod crypto_features;
pub mod echo;
pub mod echo_managed;
pub mod elliptic_curve_features;
pub mod event_features;
pub mod macro_features;
pub mod managed_address_features;
pub mod managed_buffer_features;
pub mod managed_vec_features;
pub mod non_zero_features;
pub mod storage_direct_load;
pub mod storage_direct_store;
pub mod storage_mapper_fungible_token;
pub mod storage_mapper_linked_list;
pub mod storage_mapper_map;
pub mod storage_mapper_map_storage;
pub mod storage_mapper_non_fungible_token;
pub mod storage_mapper_queue;
pub mod storage_mapper_set;
pub mod storage_mapper_single;
pub mod storage_mapper_token_attributes;
pub mod storage_mapper_unique_id_mapper;
pub mod storage_mapper_vec;
pub mod storage_mapper_whitelist;
pub mod storage_raw_api_features;
pub mod struct_eq;
pub mod token_identifier_features;
pub mod types;

#[multiversx_sc::contract]
pub trait BasicFeatures:
    big_num_methods::BigIntMethods
    + big_num_operators::BigIntOperators
    + block_info_features::BlockInfoFeatures
    + blockchain_api_features::BlockchainApiFeatures
    + codec_err_test::CodecErrorTest
    + crypto_features::CryptoFeatures
    + echo::EchoTypes
    + echo_managed::EchoManagedTypes
    + elliptic_curve_features::EllipticCurveFeatures
    + event_features::EventFeatures
    + macro_features::Macros
    + managed_address_features::ManagedAddressFeatures
    + managed_buffer_features::ManagedBufferFeatures
    + managed_vec_features::ManagedVecFeatures
    + storage_raw_api_features::StorageRawApiFeatures
    + storage_direct_load::StorageLoadFeatures
    + storage_direct_store::StorageStoreFeatures
    + storage_mapper_linked_list::LinkedListMapperFeatures
    + storage_mapper_queue::QueueMapperFeatures
    + storage_mapper_map::MapMapperFeatures
    + storage_mapper_map_storage::MapStorageMapperFeatures
    + storage_mapper_set::SetMapperFeatures
    + storage_mapper_single::SingleValueMapperFeatures
    + storage_mapper_vec::VecMapperFeatures
    + storage_mapper_token_attributes::TokenAttributesMapperFeatures
    + storage_mapper_whitelist::StorageMapperWhitelistFeatures
    + storage_mapper_fungible_token::FungibleTokenMapperFeatures
    + storage_mapper_non_fungible_token::NonFungibleTokenMapperFeatures
    + storage_mapper_unique_id_mapper::UniqueIdMapperFeatures
    + struct_eq::StructEquals
    + token_identifier_features::TokenIdentifierFeatures
    + non_zero_features::TypeFeatures
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
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

    #[endpoint]
    fn endpoint_with_mutable_arg(&self, mut arg1: BigUint, arg2: u64, arg3: u32) -> BigUint {
        arg1 += arg2;
        arg1 += arg3;

        arg1
    }
}
