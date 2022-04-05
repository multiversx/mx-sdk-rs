use core::marker::PhantomData;

use elrond_codec::TopEncodeMulti;

use crate::{
    api::{BlockchainApiImpl, ErrorApiImpl, SendApi, SendApiImpl},
    contract_base::ExitCodecErrorHandler,
    err_msg,
    types::{BigUint, CodeMetadata, ManagedAddress, ManagedBuffer, ManagedVec},
};

use super::ManagedArgBuffer;

/// Using max u64 to represent maximum possible gas,
/// so that the value zero is not reserved and can be specified explicitly.
/// Leaving the gas limit unspecified will replace it with `api.get_gas_left()`.
const UNSPECIFIED_GAS_LIMIT: u64 = u64::MAX;

#[must_use]
pub struct ContractDeploy<SA>
where
    SA: SendApi + 'static,
{
    _phantom: PhantomData<SA>,
    to: Option<ManagedAddress<SA>>, // only used for Upgrade, ignored for Deploy
    egld_payment: BigUint<SA>,
    explicit_gas_limit: u64,
    arg_buffer: ManagedArgBuffer<SA>,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `ContractDeploy::<SA>::new`, here types can be inferred from the context.
pub fn new_contract_deploy<SA>(to: Option<ManagedAddress<SA>>) -> ContractDeploy<SA>
where
    SA: SendApi + 'static,
{
    let mut contract_deploy = ContractDeploy::<SA>::new();
    contract_deploy.to = to;
    contract_deploy
}

impl<SA> Default for ContractDeploy<SA>
where
    SA: SendApi + 'static,
{
    fn default() -> Self {
        let zero = BigUint::zero();
        let arg_buffer = ManagedArgBuffer::new_empty();
        ContractDeploy {
            _phantom: PhantomData,
            to: None,
            egld_payment: zero,
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            arg_buffer,
        }
    }
}

#[allow(clippy::return_self_not_must_use)]
impl<SA> ContractDeploy<SA>
where
    SA: SendApi + 'static,
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

impl<SA> ContractDeploy<SA>
where
    SA: SendApi + 'static,
{
    /// Executes immediately, synchronously, and returns Some(Address) of the deployed contract.  
    /// Will return None if the deploy fails.  
    pub fn deploy_contract(
        self,
        code: &ManagedBuffer<SA>,
        code_metadata: CodeMetadata,
    ) -> (ManagedAddress<SA>, ManagedVec<SA, ManagedBuffer<SA>>) {
        SA::send_api_impl().deploy_contract(
            self.resolve_gas_limit(),
            &self.egld_payment,
            code,
            code_metadata,
            &self.arg_buffer,
        )
    }

    pub fn deploy_from_source(
        self,
        source_address: &ManagedAddress<SA>,
        code_metadata: CodeMetadata,
    ) -> (ManagedAddress<SA>, ManagedVec<SA, ManagedBuffer<SA>>) {
        SA::send_api_impl().deploy_from_source_contract(
            self.resolve_gas_limit(),
            &self.egld_payment,
            source_address,
            code_metadata,
            &self.arg_buffer,
        )
    }

    pub fn upgrade_from_source(
        self,
        source_address: &ManagedAddress<SA>,
        code_metadata: CodeMetadata,
    ) {
        let sc_address = &self.to.as_ref().unwrap_or_else(|| {
            SA::error_api_impl().signal_error(err_msg::RECIPIENT_ADDRESS_NOT_SET)
        });
        SA::send_api_impl().upgrade_from_source_contract(
            sc_address,
            self.resolve_gas_limit(),
            &self.egld_payment,
            source_address,
            code_metadata,
            &self.arg_buffer,
        )
    }

    pub fn upgrade_contract(self, code: &ManagedBuffer<SA>, code_metadata: CodeMetadata) {
        let sc_address = self.to.as_ref().unwrap_or_else(|| {
            SA::error_api_impl().signal_error(err_msg::RECIPIENT_ADDRESS_NOT_SET)
        });
        SA::send_api_impl().upgrade_contract(
            sc_address,
            self.resolve_gas_limit(),
            &self.egld_payment,
            code,
            code_metadata,
            &self.arg_buffer,
        );
    }
}
