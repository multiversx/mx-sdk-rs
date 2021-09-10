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

    #[endpoint(deployFromSource)]
    fn deploy_from_source(
        &self,
        source_contract_address: ManagedAddress,
        #[var_args] arguments: VarArgs<ManagedBuffer>,
    ) -> ManagedAddress {
        // TODO: use proxies to perform deploy here
        // raw deploy belongs to forwarder-raw
        let (address, _) = self.raw_vm_api().deploy_from_source_contract(
            self.blockchain().get_gas_left(),
            &self.types().big_uint_zero(),
            &source_contract_address,
            CodeMetadata::DEFAULT,
            &arguments.into_vec().managed_into(),
        );

        address
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

    #[endpoint]
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
