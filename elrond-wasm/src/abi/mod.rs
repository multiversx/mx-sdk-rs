mod build_info_abi;
mod contract_abi;
mod endpoint_abi;
mod type_abi;
mod type_abi_impl_basic;
mod type_abi_impl_codec_multi;
mod type_description;
mod type_description_container;

pub use build_info_abi::*;
pub use contract_abi::*;
pub use endpoint_abi::*;
pub use type_abi::*;
pub use type_description::*;
pub use type_description_container::*;

/// Used in generating the ABI.
// pub use git_version::git_version;

pub type TypeName = alloc::string::String;
