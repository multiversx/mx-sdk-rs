#![no_std]
#![feature(never_type)]
#![feature(exhaustive_patterns)]
#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![feature(negative_impls)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(deprecated)]

pub use multiversx_sc_derive::{self as derive, contract, module, proxy};

// re-export basic heap types
extern crate alloc;

/// The current version of `multiversx_sc_codec`, re-exported.
pub use multiversx_sc_codec as codec;

/// Reexported for convenience.
pub use crate::codec::arrayvec;

pub mod abi;
pub mod api;
pub mod contract_base;
pub mod err_msg;
pub mod esdt;
pub mod external_view_contract;
pub mod formatter;
pub mod hex_call_data;
pub mod io;
pub mod log_util;
mod macros;
pub mod non_zero_util;
pub mod storage;
pub mod types;

pub use hex_call_data::*;
pub use hex_literal;
pub use storage::{storage_clear, storage_get, storage_get_len, storage_set};

/// Conveniently groups all framework imports required by a smart contract form the framework.
pub mod imports {
    pub use crate::{
        abi::TypeAbi,
        api::{ErrorApiImpl, ManagedTypeApi},
        arrayvec::ArrayVec,
        codec::{
            multi_types::*, DecodeError, IntoMultiValue, NestedDecode, NestedEncode, TopDecode,
            TopEncode,
        },
        contract_base::{ContractBase, ProxyObjBase},
        err_msg,
        esdt::*,
        io::*,
        non_zero_usize,
        non_zero_util::*,
        require, sc_format, sc_panic, sc_print,
        storage::mappers::*,
        types::*,
    };
    pub use core::ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    };
}

/// Conveniently groups all imports required for deriving framework-related traits for types.
pub mod derive_imports {
    pub use crate::{
        codec,
        codec::derive::{
            NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
            TopEncodeOrDefault,
        },
        derive::{ManagedVecItem, TypeAbi},
    };
}
