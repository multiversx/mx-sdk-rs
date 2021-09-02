use crate::api::{BlockchainApi, SendApi};
use crate::types::{Address, ArgBuffer, BigUint, BoxedBytes, CodeMetadata};

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
    address: Address, // only used for Upgrade, ignored for Deploy
    payment_amount: BigUint<SA::ProxyTypeManager>,
    explicit_gas_limit: u64,
    pub arg_buffer: ArgBuffer, // TODO: make private and find a better way to serialize
}

/// Syntactical sugar to help macros to generate code easier.
/// Unlike calling `ContractDeploy::<SA>::new`, here types can be inferred from the context.
pub fn new_contract_deploy<SA>(
    api: SA,
    address: Address,
    payment_amount: BigUint<SA::ProxyTypeManager>,
) -> ContractDeploy<SA>
where
    SA: SendApi + 'static,
{
    let mut contract_deploy = ContractDeploy::<SA>::new(api);
    contract_deploy.address = address;
    contract_deploy.payment_amount = payment_amount;

    contract_deploy
}

impl<SA> ContractDeploy<SA>
where
    SA: SendApi + 'static,
{
    pub fn new(api: SA) -> Self {
        let zero = BigUint::zero(api.type_manager());
        ContractDeploy {
            api,
            address: Address::zero(),
            payment_amount: zero,
            explicit_gas_limit: UNSPECIFIED_GAS_LIMIT,
            arg_buffer: ArgBuffer::new(),
        }
    }

    pub fn with_egld_transfer(mut self, payment_amount: BigUint<SA::ProxyTypeManager>) -> Self {
        self.payment_amount = payment_amount;
        self
    }

    pub fn with_gas_limit(mut self, gas_limit: u64) -> Self {
        self.explicit_gas_limit = gas_limit;
        self
    }

    pub fn get_mut_arg_buffer(&mut self) -> &mut ArgBuffer {
        &mut self.arg_buffer
    }

    /// Provided for cases where we build the contract deploy by hand.
    pub fn push_argument_raw_bytes(&mut self, bytes: &[u8]) {
        self.arg_buffer.push_argument_bytes(bytes);
    }

    fn resolve_gas_limit(&self) -> u64 {
        if self.explicit_gas_limit == UNSPECIFIED_GAS_LIMIT {
            self.api.blockchain().get_gas_left()
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
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
    ) -> Option<Address> {
        self.api.deploy_contract(
            self.resolve_gas_limit(),
            &self.payment_amount,
            code,
            code_metadata,
            &self.arg_buffer,
        )
    }

    pub fn upgrade_contract(self, code: &BoxedBytes, code_metadata: CodeMetadata) {
        self.api.upgrade_contract(
            &self.address,
            self.resolve_gas_limit(),
            &self.payment_amount,
            code,
            code_metadata,
            &self.arg_buffer,
        );
    }

    // TODO: deploy contract with code from another contract
}
