#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct TokenIdPair<M: ManagedTypeApi> {
    first_token_id: TokenIdentifier<M>,
    second_token_id: TokenIdentifier<M>,
}

#[multiversx_sc::contract]
pub trait Factory {
    #[init]
    fn init(&self, pair_template_address: ManagedAddress) {
        self.pair_template_address().set(&pair_template_address);
    }

    #[endpoint(createPair)]
    fn create_pair(&self, token_id_pair: TokenIdPair<Self::Api>) -> ManagedAddress {
        require!(self.get_pair(&token_id_pair).is_none(), "Already has pair");

        let mut arguments = ManagedArgBuffer::new();
        arguments.push_arg(&token_id_pair.first_token_id);
        arguments.push_arg(&token_id_pair.second_token_id);

        let (pair_address, _) = self.send_raw().deploy_from_source_contract(
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &self.pair_template_address().get(),
            CodeMetadata::DEFAULT,
            &arguments,
        );
        self.pairs().insert(token_id_pair, pair_address.clone());

        pair_address
    }

    #[view(getPair)]
    fn get_pair(&self, token_id_pair: &TokenIdPair<Self::Api>) -> Option<ManagedAddress> {
        let opt_address = self.pairs().get(token_id_pair);

        if opt_address.is_none() {
            self.pairs().get(&TokenIdPair {
                first_token_id: token_id_pair.second_token_id.clone(),
                second_token_id: token_id_pair.first_token_id.clone(),
            })
        } else {
            opt_address
        }
    }

    #[storage_mapper("pair_template_address")]
    fn pair_template_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("pairs")]
    fn pairs(&self) -> MapMapper<TokenIdPair<Self::Api>, ManagedAddress>;
}
