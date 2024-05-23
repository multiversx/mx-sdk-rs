multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DeployContractModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

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
        self.vault_proxy()
            .init(opt_arg)
            .deploy_contract(code, CodeMetadata::DEFAULT)
    }

    #[endpoint]
    fn deploy_vault_from_source(
        &self,
        source_address: ManagedAddress,
        opt_arg: OptionalValue<ManagedBuffer>,
    ) -> MultiValue2<ManagedAddress, OptionalValue<ManagedBuffer>> {
        self.vault_proxy()
            .init(opt_arg)
            .deploy_from_source(&source_address, CodeMetadata::DEFAULT)
            .into()
    }
}
