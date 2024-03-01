// Note: Simple macros cannot be placed in multiversx-sc-derive,
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
        use multiversx_sc::{
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
            require, require_old, sc_error, sc_format, sc_panic, sc_print,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
        };
    };
}

/// Imports required for deriving serialization and TypeAbi.
#[macro_export]
macro_rules! derive_imports {
    () => {
        use multiversx_sc::{
            codec,
            codec::derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
            derive::{ManagedVecItem, TypeAbi},
        };
    };
}

/// Compact way of returning a static error message.
#[macro_export]
macro_rules! sc_error {
    ($s:expr) => {
        multiversx_sc::types::SCResult::Err(multiversx_sc::types::StaticSCError::from($s)).into()
    };
}

/// Allows us to write Solidity style `require!(<condition>, <error_msg>)` and avoid if statements.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// Example:
///
/// ```rust
/// # use multiversx_sc::require_old;
/// # use multiversx_sc::types::{*, SCResult::Ok};
/// # pub trait ExampleContract: multiversx_sc::contract_base::ContractBase
/// # {
/// fn only_accept_positive_old(&self, x: i32) -> SCResult<()> {
///     require_old!(x > 0, "only positive values accepted");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! require_old {
    ($expression:expr, $error_msg:expr) => {
        if (!($expression)) {
            return multiversx_sc::sc_error!($error_msg);
        }
    };
}

#[macro_export]
macro_rules! sc_panic {
    ($msg:tt, $($arg:expr),+ $(,)?) => {{
        let mut ___buffer___ =
            multiversx_sc::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
        multiversx_sc::derive::format_receiver_args!(___buffer___, $msg, $($arg),+);
        multiversx_sc::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(___buffer___.into_managed_buffer());
    }};
    ($msg:expr $(,)?) => {
        multiversx_sc::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message($msg);
    };
}

/// Allows us to write Solidity style `require!(<condition>, <error_msg>)` and avoid if statements.
///
/// The most common way to use it is to provide a string message with optional format arguments.
///
/// It is also possible to give the error as a variable of types such as `&str`, `&[u8]` or `ManagedBuffer`.
///
/// Examples:
///
/// ```rust
/// # use multiversx_sc::{types::ManagedBuffer, require};
/// # pub trait ExampleContract: multiversx_sc::contract_base::ContractBase
/// # {
/// fn only_accept_positive(&self, x: i32) {
///     require!(x > 0, "only positive values accepted");
/// }
///
/// fn only_accept_negative(&self, x: i32) {
///     require!(x < 0, "only negative values accepted, {} is not negative", x);
/// }
///
/// fn only_accept_zero(&self, x: i32, message: &ManagedBuffer<Self::Api>) {
///     require!(x == 0, message,);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! require {
    ($expression:expr, $($msg_tokens:tt),+  $(,)?) => {
        if (!($expression)) {
            multiversx_sc::sc_panic!($($msg_tokens),+);
        }
    };
}

#[macro_export]
macro_rules! sc_print {
    ($msg:tt, $($arg:expr),* $(,)?) => {{
        let mut ___buffer___ =
            <<Self::Api as multiversx_sc::api::PrintApi>::PrintApiImpl as multiversx_sc::api::PrintApiImpl>::Buffer::default();
        multiversx_sc::derive::format_receiver_args!(___buffer___, $msg, $($arg),*);
        <<Self::Api as multiversx_sc::api::PrintApi>::PrintApiImpl as multiversx_sc::api::PrintApiImpl>::print_buffer(
            &<Self::Api as multiversx_sc::api::PrintApi>::print_api_impl(),
            ___buffer___,
        );
    }};
}

#[macro_export]
macro_rules! sc_format {
    ($msg:tt, $($arg:expr),+ $(,)?) => {{
        let mut ___buffer___ =
            multiversx_sc::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
        multiversx_sc::derive::format_receiver_args!(___buffer___, $msg, $($arg),+);
        ___buffer___.into_managed_buffer()
    }};
    ($msg:expr $(,)?) => {{
        multiversx_sc::types::ManagedBuffer::new_from_bytes($msg.as_bytes())
    }};
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
            multiversx_sc::types::SCResult::Ok(t) => t,
            multiversx_sc::types::SCResult::Err(e) => {
                return multiversx_sc::types::SCResult::Err(e);
            },
        }
    };
}

/// Very compact way of not allowing anyone but the owner to call a function.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use multiversx_sc::*;
/// # use multiversx_sc::api::BlockchainApi;
/// # use multiversx_sc::types::{*, SCResult::Ok};
/// # pub trait ExampleContract: multiversx_sc::contract_base::ContractBase
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     only_owner!(self, "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[deprecated(
    since = "0.26.0",
    note = "Replace with the `#[only_owner]` attribute that can be placed on an endpoint. That one is more compact and shows up in the ABI."
)]
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
        NonZeroUsize::new($input).unwrap_or_else(|| sc_panic!($error_msg))
    };
}
