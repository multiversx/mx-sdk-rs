elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait DeployContractModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint]
    fn deploy_contract(
        &self,
        code: ManagedBuffer,
        #[var_args] opt_arg: OptionalArg<ManagedBuffer>,
    ) -> MultiResult2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        self.deploy_vault(&code, opt_arg)
    }

    #[endpoint]
    fn deploy_two_contracts(
        &self,
        code: ManagedBuffer,
    ) -> MultiResult2<ManagedAddress, ManagedAddress> {
        let (first_deployed_contract_address, _) =
            self.deploy_vault(&code, OptionalArg::None).into_tuple();
        let (second_deployed_contract_address, _) =
            self.deploy_vault(&code, OptionalArg::None).into_tuple();

        (
            first_deployed_contract_address,
            second_deployed_contract_address,
        )
            .into()
    }

    fn deploy_vault(
        &self,
        code: &ManagedBuffer,
        #[var_args] opt_arg: OptionalArg<ManagedBuffer>,
    ) -> MultiResult2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        self.vault_proxy()
            .init(opt_arg)
            .deploy_contract(code, CodeMetadata::DEFAULT)
            .into()
    }

    #[endpoint]
    fn deploy_vault_from_source(
        &self,
        source_address: ManagedAddress,
        #[var_args] opt_arg: OptionalArg<ManagedBuffer>,
    ) -> MultiResult2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        self.vault_proxy()
            .init(opt_arg)
            .deploy_from_source(&source_address, CodeMetadata::DEFAULT)
            .into()
    }
}
