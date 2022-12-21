use core::marker::PhantomData;

use crate::codec::{CodecFrom, TopEncodeMulti};

use crate::{
    api::{BlockchainApiImpl, CallTypeApi},
    contract_base::{ExitCodecErrorHandler, SendRawWrapper},
    err_msg,
    io::{ArgErrorHandler, ArgId, ManagedResultArgLoader},
    types::{BigUint, CodeMetadata, ManagedAddress, ManagedBuffer, ManagedOption, ManagedVec},
};

use super::ManagedArgBuffer;

/// Using max u64 to represent maximum possible gas,
/// so that the value zero is not reserved and can be specified explicitly.
/// Leaving the gas limit unspecified will replace it with `api.get_gas_left()`.
const UNSPECIFIED_GAS_LIMIT: u64 = u64::MAX;

#[must_use]
pub struct ContractDeploy<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    _phantom: PhantomData<SA>,
    pub to: ManagedOption<SA, ManagedAddress<SA>>, // only used for Upgrade, ignored for Deploy
    pub egld_payment: BigUint<SA>,
    pub explicit_gas_limit: u64,
    pub arg_buffer: ManagedArgBuffer<SA>,
    _return_type: PhantomData<OriginalResult>,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `ContractDeploy::<SA>::new`, here types can be inferred from the context.
pub fn new_contract_deploy<SA, OriginalResult>(
    to: ManagedOption<SA, ManagedAddress<SA>>,
) -> ContractDeploy<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    let mut contract_deploy = ContractDeploy::new();
    contract_deploy.to = to;
    contract_deploy
}

impl<SA, OriginalResult> Default for ContractDeploy<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    fn default() -> Self {
        let zero = BigUint::zero();
        let arg_buffer = ManagedArgBuffer::new();
        ContractDeploy {
            _phantom: PhantomData,
            to: ManagedOption::none(),
            egld_payment: zero,
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            arg_buffer,
            _return_type: PhantomData,
        }
    }
}

#[allow(clippy::return_self_not_must_use)]
impl<SA, OriginalResult> ContractDeploy<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_egld_transfer(mut self, payment_amount: BigUint<SA>) -> Self {
        self.egld_payment = payment_amount;
        self
    }

    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.explicit_gas_limit = gas_limit;
        self
    }

    pub fn push_endpoint_arg<T: TopEncodeMulti>(&mut self, endpoint_arg: &T) {
        let h = ExitCodecErrorHandler::<SA>::from(err_msg::CONTRACT_CALL_ENCODE_ERROR);
        let Ok(()) = endpoint_arg.multi_encode_or_handle_err(&mut self.arg_buffer, h);
    }

    fn resolve_gas_limit(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            SA::blockchain_api_impl().get_gas_left()
        } else {
            self.explicit_gas_limit
        }
    }
}

impl<SA, OriginalResult> ContractDeploy<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn decode_result<RequestedResult>(
        raw_result: ManagedVec<SA, ManagedBuffer<SA>>,
    ) -> RequestedResult
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let mut loader = ManagedResultArgLoader::new(raw_result);
        let arg_id = ArgId::from(&b"init result"[..]);
        let h = ArgErrorHandler::<SA>::from(arg_id);
        let Ok(result) = RequestedResult::multi_decode_or_handle_err(&mut loader, h);
        result
    }

    /// Executes immediately, synchronously, and returns Some(Address) of the deployed contract.  
    /// Will return None if the deploy fails.  
    pub fn deploy_contract<RequestedResult>(
        self,
        code: &ManagedBuffer<SA>,
        code_metadata: CodeMetadata,
    ) -> (ManagedAddress<SA>, RequestedResult)
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let (address, raw_result) = SendRawWrapper::<SA>::new().deploy_contract(
            self.resolve_gas_limit(),
            &self.egld_payment,
            code,
            code_metadata,
            &self.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        (address, Self::decode_result(raw_result))
    }

    pub fn deploy_from_source<RequestedResult>(
        self,
        source_address: &ManagedAddress<SA>,
        code_metadata: CodeMetadata,
    ) -> (ManagedAddress<SA>, RequestedResult)
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let (address, raw_result) = SendRawWrapper::<SA>::new().deploy_from_source_contract(
            self.resolve_gas_limit(),
            &self.egld_payment,
            source_address,
            code_metadata,
            &self.arg_buffer,
        );

        SendRawWrapper::<SA>::new().clean_return_data();

        (address, Self::decode_result(raw_result))
    }

    pub fn upgrade_from_source(
        self,
        source_address: &ManagedAddress<SA>,
        code_metadata: CodeMetadata,
    ) {
        let gas = self.resolve_gas_limit();
        let sc_address = &self
            .to
            .unwrap_or_sc_panic(err_msg::RECIPIENT_ADDRESS_NOT_SET);
        SendRawWrapper::<SA>::new().upgrade_from_source_contract(
            sc_address,
            gas,
            &self.egld_payment,
            source_address,
            code_metadata,
            &self.arg_buffer,
        );
    }

    pub fn upgrade_contract(self, code: &ManagedBuffer<SA>, code_metadata: CodeMetadata) {
        let gas = self.resolve_gas_limit();
        let sc_address = &self
            .to
            .unwrap_or_sc_panic(err_msg::RECIPIENT_ADDRESS_NOT_SET);
        SendRawWrapper::<SA>::new().upgrade_contract(
            sc_address,
            gas,
            &self.egld_payment,
            code,
            code_metadata,
            &self.arg_buffer,
        );
    }
}
