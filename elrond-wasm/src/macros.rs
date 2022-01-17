// Note: Simple macros cannot be placed in elrond-wasm-derive,
// because Rust "cannot export macro_rules! macros from a `proc-macro` crate type currently".

/// Getting all imports needed for a smart contract.
#[macro_export]
macro_rules! imports {
    () => {
        use core::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        };
        use elrond_wasm::{
            api::{
                BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl,
                CryptoApi, CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi,
                LogApiImpl, ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
            },
            arrayvec::ArrayVec,
            contract_base::{ContractBase, ProxyObjBase},
            elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode},
            err_msg,
            esdt::*,
            io::*,
            non_zero_usize,
            non_zero_util::*,
            only_owner, require, sc_error, signal_error,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
            Box, Vec,
        };
    };
}

/// Imports required for deriving serialization and TypeAbi.
#[macro_export]
macro_rules! derive_imports {
    () => {
        use elrond_wasm::{
            derive::{ManagedVecItem, TypeAbi},
            elrond_codec,
            elrond_codec::elrond_codec_derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
        };
    };
}

/// Compact way of returning a static error message.
#[macro_export]
macro_rules! sc_error {
    ($s:expr) => {
        elrond_wasm::types::SCResult::Err(elrond_wasm::types::StaticSCError::from($s)).into()
    };
}

#[macro_export]
macro_rules! signal_error {
    ($msg:tt, $($arg:expr),+) => {{
        let mut ___buffer___ =
            elrond_wasm::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
        elrond_wasm::derive::format_receiver_args!(___buffer___, $msg, $($arg),+);
        Self::Api::error_api_impl().signal_error_from_buffer(___buffer___.into_managed_buffer().get_raw_handle());
    }};
    ($msg:tt) => {
        Self::Api::error_api_impl().signal_error($msg.as_bytes());
    };
}

/// Equivalent to the `?` operator for SCResult.
#[deprecated(
    since = "0.16.0",
    note = "The `?` operator can now be used on `SCResult`, please use it instead."
)]
#[macro_export]
macro_rules! sc_try {
    ($s:expr) => {
        match $s {
            elrond_wasm::types::SCResult::Ok(t) => t,
            elrond_wasm::types::SCResult::Err(e) => {
                return elrond_wasm::types::SCResult::Err(e);
            },
        }
    };
}

/// Allows us to write Solidity style `require!(<condition>, <error_msg>)` and avoid if statements.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use elrond_wasm::*;
/// # use elrond_wasm::api::BlockchainApi;
/// # use elrond_wasm::types::{*, SCResult::Ok};
/// # pub trait ExampleContract: elrond_wasm::contract_base::ContractBase
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     require!(self.blockchain().get_caller() == self.blockchain().get_owner_address(), "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! require {
    ($expression:expr, $error_msg:expr) => {
        if (!($expression)) {
            return sc_error!($error_msg);
        }
    };
}

/// Very compact way of not allowing anyone but the owner to call a function.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use elrond_wasm::*;
/// # use elrond_wasm::api::BlockchainApi;
/// # use elrond_wasm::types::{*, SCResult::Ok};
/// # pub trait ExampleContract: elrond_wasm::contract_base::ContractBase
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     only_owner!(self, "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! only_owner {
    ($trait_self: expr, $error_msg:expr) => {
        if ($trait_self.blockchain().get_caller() != $trait_self.blockchain().get_owner_address()) {
            return sc_error!($error_msg);
        }
    };
}

/// Converts usize to NonZeroUsize or returns SCError.
#[macro_export]
macro_rules! non_zero_usize {
    ($input: expr, $error_msg:expr) => {
        if let Some(nz) = NonZeroUsize::new($input) {
            nz
        } else {
            return sc_error!($error_msg);
        }
    };
}
