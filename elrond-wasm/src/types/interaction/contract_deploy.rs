use crate::{
    api::SendApi,
    types::{BigUint, CodeMetadata, ManagedAddress, ManagedBuffer, ManagedVec},
    ContractCallArg,
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
    api: SA,
    to: ManagedAddress<SA>, // only used for Upgrade, ignored for Deploy
    egld_payment: BigUint<SA>,
    explicit_gas_limit: u64,
    arg_buffer: ManagedArgBuffer<SA>,
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `ContractDeploy::<SA>::new`, here types can be inferred from the context.
pub fn new_contract_deploy<SA>(api: SA, to: ManagedAddress<SA>) -> ContractDeploy<SA>
where
    SA: SendApi + 'static,
{
    let mut contract_deploy = ContractDeploy::<SA>::new(api);
    contract_deploy.to = to;
    contract_deploy
}

impl<SA> ContractDeploy<SA>
where
    SA: SendApi + 'static,
{
    pub fn new(api: SA) -> Self {
        let zero = BigUint::zero();
        let zero_address = ManagedAddress::zero();
        let arg_buffer = ManagedArgBuffer::new_empty();
        ContractDeploy {
            api,
            to: zero_address,
            egld_payment: zero,
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            arg_buffer,
        }
    }

    pub fn with_egld_transfer(mut self, payment_amount: BigUint<SA>) -> Self {
        self.egld_payment = payment_amount;
        self
    }

    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.explicit_gas_limit = gas_limit;
        self
    }

    pub fn push_endpoint_arg<D: ContractCallArg>(&mut self, endpoint_arg: D) {
        endpoint_arg.push_dyn_arg(&mut self.arg_buffer);
    }

    // pub fn get_mut_arg_buffer(&mut self) -> &mut ArgBuffer {
    //     &mut self.arg_buffer
    // }

    // /// Provided for cases where we build the contract deploy by hand.
    // pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
    //     self.arg_buffer.push_argument_bytes(bytes);
    // }

    fn resolve_gas_limit(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            self.api.get_gas_left()
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
        self.api.deploy_contract(
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
        self.api.deploy_from_source_contract(
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
        self.api.upgrade_from_source_contract(
            &self.to,
            self.resolve_gas_limit(),
            &self.egld_payment,
            source_address,
            code_metadata,
            &self.arg_buffer,
        )
    }

    pub fn upgrade_contract(self, code: &ManagedBuffer<SA>, code_metadata: CodeMetadata) {
        self.api.upgrade_contract(
            &self.to,
            self.resolve_gas_limit(),
            &self.egld_payment,
            code,
            code_metadata,
            &self.arg_buffer,
        );
    }
}
