use crate::vault_proxy;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DeployContractModule {
    #[endpoint]
    fn deploy_contract(
        &self,
        code: ManagedBuffer,
        opt_arg: OptionalValue<ManagedBuffer>,
    ) -> MultiValue2<ManagedAddress, OptionalValue<ManagedBuffer>> {
        self.perform_deploy_vault(&code, opt_arg).into()
    }

    #[endpoint]
    fn deploy_two_contracts(
        &self,
        code: ManagedBuffer,
    ) -> MultiValue2<ManagedAddress, ManagedAddress> {
        let (first_deployed_contract_address, _) =
            self.perform_deploy_vault(&code, OptionalValue::None);
        let (second_deployed_contract_address, _) =
            self.perform_deploy_vault(&code, OptionalValue::None);

        (
            first_deployed_contract_address,
            second_deployed_contract_address,
        )
            .into()
    }

    fn perform_deploy_vault(
        &self,
        code: &ManagedBuffer,
        opt_arg: OptionalValue<ManagedBuffer>,
    ) -> (ManagedAddress, OptionalValue<ManagedBuffer>) {
        let (new_address, response_vec) = self
            .tx()
            .typed(vault_proxy::VaultProxy)
            .init(opt_arg)
            .code(code.clone())
            .returns(ReturnsNewAddress)
            .returns(ReturnsRawResult)
            .sync_call();

        let response = if response_vec.is_empty() {
            OptionalValue::None
        } else {
            OptionalValue::Some(response_vec.get(0).clone_value())
        };

        (new_address, response)
    }

    #[endpoint]
    fn deploy_vault_from_source(
        &self,
        source_address: ManagedAddress,
        opt_arg: OptionalValue<ManagedBuffer>,
    ) -> MultiValue2<ManagedAddress, OptionalValue<ManagedBuffer>> {
        let (new_address, response_vec) = self
            .tx()
            .typed(vault_proxy::VaultProxy)
            .init(opt_arg)
            .code_metadata(CodeMetadata::DEFAULT)
            .from_source(source_address)
            .returns(ReturnsNewAddress)
            .returns(ReturnsRawResult)
            .sync_call();

        let response = if response_vec.is_empty() {
            OptionalValue::None
        } else {
            OptionalValue::Some(response_vec.get(0).clone_value())
        };

        MultiValue2::from((new_address, response))
    }
}
