elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait DeployContractModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;

    #[endpoint]
    fn deploy_contract(&self, code: ManagedBuffer) -> SCResult<ManagedAddress> {
        let deployed_contract_address = self.deploy_vault(&code).ok_or("Deploy failed")?;

        Ok(deployed_contract_address)
    }

    #[endpoint(deployFromSource)]
    fn deploy_from_source(
        &self,
        source_contract_address: ManagedAddress,
        #[var_args] arguments: VarArgs<ManagedBuffer>,
    ) -> SCResult<ManagedAddress> {
        // TODO: use proxies to perform deploy here
        // raw deploy belongs to forwarder-raw
        self.raw_vm_api()
            .deploy_from_source_contract(
                self.blockchain().get_gas_left(),
                &self.types().big_uint_zero(),
                &source_contract_address,
                CodeMetadata::DEFAULT,
                &arguments.into_vec().managed_into(self.type_manager()),
            )
            .ok_or("Deploy from source contract failed")
            .into()
    }

    #[endpoint]
    fn deploy_two_contracts(
        &self,
        code: ManagedBuffer,
    ) -> SCResult<MultiResult2<ManagedAddress, ManagedAddress>> {
        let first_deployed_contract_address =
            self.deploy_vault(&code).ok_or("First deploy failed")?;

        let second_deployed_contract_address =
            self.deploy_vault(&code).ok_or("Second deploy failed")?;

        Ok((
            first_deployed_contract_address,
            second_deployed_contract_address,
        )
            .into())
    }

    #[endpoint]
    fn deploy_vault(&self, code: &ManagedBuffer) -> Option<ManagedAddress> {
        self.vault_proxy()
            .init()
            .deploy_contract(code, CodeMetadata::DEFAULT)
    }
}
