mod returns_bt;
mod returns_new_address;
mod returns_new_managed_address;
mod returns_raw_result;
mod returns_result;
mod returns_result_as;
mod returns_result_unmanaged;
mod with_new_address;
mod with_raw_result;
mod with_result;
mod with_result_conv;

pub use returns_bt::*;
pub use returns_new_address::*;
pub use returns_new_managed_address::*;
pub use returns_raw_result::*;
pub use returns_result::*;
pub use returns_result_as::*;
pub use returns_result_unmanaged::ReturnsResultUnmanaged;
pub use with_new_address::*;
pub use with_raw_result::WithRawResult;
pub use with_result::WithResult;
pub use with_result_conv::*;

use super::TxEnv;

pub trait TxResultHandler<Env>
where
    Env: TxEnv,
{
    type OriginalResult;
}

impl<Env> TxResultHandler<Env> for ()
where
    Env: TxEnv,
{
    type OriginalResult = ();
}

/// Indicates that given result handler is empty, i.e. doesn't cause any side effects and returns nothing.
///
/// Implemented for `()` and `OriginalResultMarker`.
pub trait TxEmptyResultHandler<Env>: TxResultHandler<Env>
where
    Env: TxEnv,
{
}

impl<Env> TxEmptyResultHandler<Env> for () where Env: TxEnv {}
