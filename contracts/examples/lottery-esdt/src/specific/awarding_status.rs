use multiversx_sc::derive_imports::*;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq)]
pub enum AwardingStatus {
    Ongoing,
    Finished,
}
