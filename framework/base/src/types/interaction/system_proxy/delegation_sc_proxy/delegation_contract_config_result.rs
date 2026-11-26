use alloc::string::String;

use multiversx_sc_codec::{DecodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput};

use crate::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress},
};

/// The decoded result of the `getContractConfig` view of a delegation contract.
#[derive(Debug, Clone)]
pub struct DelegationContractConfigResult<M: ManagedTypeApi> {
    pub owner: ManagedAddress<M>,
    /// Service fee, in basis points (10000 = 100%).
    pub service_fee: u64,
    /// Maximum delegation cap, in EGLD atto-units. 0 means uncapped.
    pub max_delegation_cap: BigUint<M>,
    pub initial_owner_funds: BigUint<M>,
    pub automatic_activation: bool,
    pub with_delegation_cap: bool,
    pub changeable_service_fee: bool,
    pub check_cap_on_redelegate: bool,
    /// Block nonce at which the contract was created.
    pub created_nonce: u64,
    pub unbond_period_in_epochs: u64,
}

impl<M> TopDecodeMulti for DelegationContractConfigResult<M>
where
    M: ManagedTypeApi,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let is_true = |value: String| value == "true";

        let owner = ManagedAddress::multi_decode_or_handle_err(input, h)?;
        let service_fee = u64::multi_decode_or_handle_err(input, h)?;
        let max_delegation_cap = BigUint::multi_decode_or_handle_err(input, h)?;
        let initial_owner_funds = BigUint::multi_decode_or_handle_err(input, h)?;
        let automatic_activation = String::multi_decode_or_handle_err(input, h)?;
        let with_delegation_cap = String::multi_decode_or_handle_err(input, h)?;
        let changeable_service_fee = String::multi_decode_or_handle_err(input, h)?;
        let check_cap_on_redelegate = String::multi_decode_or_handle_err(input, h)?;
        let created_nonce = u64::multi_decode_or_handle_err(input, h)?;
        let unbond_period_in_epochs = u64::multi_decode_or_handle_err(input, h)?;

        Ok(DelegationContractConfigResult {
            owner,
            service_fee,
            max_delegation_cap,
            initial_owner_funds,
            automatic_activation: is_true(automatic_activation),
            with_delegation_cap: is_true(with_delegation_cap),
            changeable_service_fee: is_true(changeable_service_fee),
            check_cap_on_redelegate: is_true(check_cap_on_redelegate),
            created_nonce,
            unbond_period_in_epochs,
        })
    }
}
