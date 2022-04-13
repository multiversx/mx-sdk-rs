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
            elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
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

/// Allows us to write Solidity style `require!(<condition>, <error_msg>)` and avoid if statements.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// Example:
///
/// ```rust
/// # use elrond_wasm::require_old;
/// # use elrond_wasm::types::{*, SCResult::Ok};
/// # pub trait ExampleContract: elrond_wasm::contract_base::ContractBase
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
            return elrond_wasm::sc_error!($error_msg);
        }
    };
}

#[macro_export]
macro_rules! sc_panic {
    ($msg:tt, $($arg:expr),+ $(,)?) => {{
        let mut ___buffer___ =
            elrond_wasm::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
        elrond_wasm::derive::format_receiver_args!(___buffer___, $msg, $($arg),+);
        elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(___buffer___.into_managed_buffer());
    }};
    ($msg:expr $(,)?) => {
        elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message($msg);
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
/// # use elrond_wasm::{types::ManagedBuffer, require};
/// # pub trait ExampleContract: elrond_wasm::contract_base::ContractBase
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
            elrond_wasm::sc_panic!($($msg_tokens),+);
        }
    };
}

#[macro_export]
macro_rules! sc_print {
    ($msg:tt, $($arg:expr),* $(,)?) => {{
        let mut ___buffer___ =
            <<Self::Api as elrond_wasm::api::PrintApi>::PrintApiImpl as elrond_wasm::api::PrintApiImpl>::Buffer::default();
        elrond_wasm::derive::format_receiver_args!(___buffer___, $msg, $($arg),*);
        <<Self::Api as elrond_wasm::api::PrintApi>::PrintApiImpl as elrond_wasm::api::PrintApiImpl>::print_buffer(
            &<Self::Api as elrond_wasm::api::PrintApi>::print_api_impl(),
            ___buffer___,
        );
    }};
}

#[macro_export]
macro_rules! sc_format {
    ($msg:tt, $($arg:expr),+ $(,)?) => {{
        let mut ___buffer___ =
            elrond_wasm::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
        elrond_wasm::derive::format_receiver_args!(___buffer___, $msg, $($arg),+);
        ___buffer___.into_managed_buffer()
    }};
    ($msg:expr $(,)?) => {{
        elrond_wasm::types::ManagedBuffer::new_from_bytes($msg.as_bytes())
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
            elrond_wasm::types::SCResult::Ok(t) => t,
            elrond_wasm::types::SCResult::Err(e) => {
                return elrond_wasm::types::SCResult::Err(e);
            },
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
