/// Conveniently groups all framework imports required by a smart contract form the framework.
pub mod imports {
    pub use crate::{
        abi::TypeAbi,
        api::{ErrorApiImpl, ManagedTypeApi, VMApi},
        arrayvec::ArrayVec,
        codec::{
            multi_types::*, CodecFrom, CodecFromSelf, CodecInto, DecodeError, IntoMultiValue,
            NestedDecode, NestedEncode, TopDecode, TopEncode,
        },
        contract_base::{ContractBase, ProxyObjBase, ProxyObjNew},
        err_msg,
        io::*,
        non_zero_usize,
        non_zero_util::*,
        require, sc_format, sc_panic, sc_print,
        storage::mappers::*,
        types::{system_proxy::*, *},
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
        derive::{type_abi, ManagedVecItem, TypeAbi},
    };
}

/// Conveniently groups all imports required for generated proxies.
pub mod proxy_imports {
    pub use super::{derive_imports::*, imports::*};
}
