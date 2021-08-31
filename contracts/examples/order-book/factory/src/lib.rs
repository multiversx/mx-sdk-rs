#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct TokenIdPair<M: ManagedTypeApi> {
    first_token_id: TokenIdentifier<M>,
    second_token_id: TokenIdentifier<M>,
}

#[elrond_wasm::contract]
pub trait Factory {
    #[init]
    fn init(&self, pair_template_address: Address) {
        self.pair_template_address().set(&pair_template_address);
    }

    #[endpoint(createPair)]
    fn create_pair(&self, token_id_pair: TokenIdPair<Self::TypeManager>) -> SCResult<Address> {
        require!(self.get_pair(&token_id_pair).is_none(), "Already has pair");

        let mut arguments: ArgBuffer = ArgBuffer::new();
        arguments.push_argument_bytes(token_id_pair.first_token_id.to_esdt_identifier().as_slice());
        arguments.push_argument_bytes(
            token_id_pair
                .second_token_id
                .to_esdt_identifier()
                .as_slice(),
        );

        let pair_address = self
            .send()
            .deploy_from_source_contract(
                self.blockchain().get_gas_left(),
                &self.types().big_uint_zero(),
                &self.pair_template_address().get(),
                CodeMetadata::DEFAULT,
                &arguments,
            )
            .ok_or("Pair deployment failed")?;

        self.pairs().insert(token_id_pair, pair_address.clone());
        Ok(pair_address)
    }

    #[view(getPair)]
    fn get_pair(&self, token_id_pair: &TokenIdPair<Self::TypeManager>) -> Option<Address> {
        let address = self.pairs().get(token_id_pair);

        if address.is_none() {
            self.pairs().get(&TokenIdPair {
                first_token_id: token_id_pair.second_token_id.clone(),
                second_token_id: token_id_pair.first_token_id.clone(),
            })
        } else {
            address
        }
    }

    #[storage_mapper("pair_template_address")]
    fn pair_template_address(&self) -> SingleValueMapper<Self::Storage, Address>;

    #[storage_mapper("pairs")]
    fn pairs(&self) -> MapMapper<Self::Storage, TokenIdPair<Self::TypeManager>, Address>;
}
