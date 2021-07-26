#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, Clone)]
pub struct TokenIdPair {
    first_token_id: TokenIdentifier,
    second_token_id: TokenIdentifier,
}

#[elrond_wasm_derive::contract]
pub trait Factory {
    #[init]
    fn init(&self, pair_template_address: Address) {
        self.pair_template_address().set(&pair_template_address);
    }

    #[endpoint(createPair)]
    fn create_pair(&self, token_id_pair: TokenIdPair) -> SCResult<Address> {
        require!(self.get_pair(&token_id_pair).is_none(), "Already has pair");

        let mut arguments: ArgBuffer = ArgBuffer::new();
        arguments.push_argument_bytes(token_id_pair.first_token_id.as_esdt_identifier());
        arguments.push_argument_bytes(token_id_pair.second_token_id.as_esdt_identifier());

        let pair_address = self
            .send()
            .deploy_from_source_contract(
                self.blockchain().get_gas_left(),
                &Self::BigUint::zero(),
                &self.pair_template_address().get(),
                CodeMetadata::DEFAULT,
                &arguments,
            )
            .ok_or("Pair deployment failed")?;

        self.pairs().insert(token_id_pair, pair_address.clone());
        Ok(pair_address)
    }

    #[view(getPair)]
    fn get_pair(&self, token_id_pair: &TokenIdPair) -> Option<Address> {
        let address = self.pairs().get(&token_id_pair);

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
    fn pairs(&self) -> MapMapper<Self::Storage, TokenIdPair, Address>;
}
