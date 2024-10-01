mod build_info_abi;
mod contract_abi;
mod endpoint_abi;
mod esdt_attribute_abi;
mod event_abi;
mod type_abi;
mod type_abi_from;
mod type_abi_impl_basic;
mod type_abi_impl_codec_multi;
mod type_abi_impl_vm_core;
mod type_description;
mod type_description_container;

#[cfg(feature = "num-bigint")]
mod type_abi_impl_big_int;

pub use build_info_abi::*;
pub use contract_abi::*;
pub use endpoint_abi::*;
pub use esdt_attribute_abi::EsdtAttributeAbi;
pub use event_abi::*;
pub use type_abi::*;
pub use type_abi_from::*;
pub use type_description::*;
pub use type_description_container::*;

pub type TypeName = alloc::string::String;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct TypeNames {
    pub abi: alloc::string::String,
    pub rust: alloc::string::String,
}

impl TypeNames {
    pub const fn new() -> Self {
        TypeNames {
            abi: alloc::string::String::new(),
            rust: alloc::string::String::new(),
        }
    }
}
