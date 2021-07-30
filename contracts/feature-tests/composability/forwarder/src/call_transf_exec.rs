elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ForwarderTransferExecuteModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_accept_funds(
        &self,
        to: Address,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] payment: Self::BigUint,
        #[payment_nonce] token_nonce: u64,
    ) {
        self.vault_proxy()
            .contract(to)
            .accept_funds(token, payment)
            .with_nft_nonce(token_nonce)
            .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_accept_funds_twice(
        &self,
        to: Address,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] payment: Self::BigUint,
        #[payment_nonce] token_nonce: u64,
    ) {
        let half_payment = payment / Self::BigUint::from(2u32);
        let half_gas = self.blockchain().get_gas_left() / 2;

        self.vault_proxy()
            .contract(to.clone())
            .accept_funds(token.clone(), half_payment.clone())
            .with_nft_nonce(token_nonce)
            .with_gas_limit(half_gas)
            .transfer_execute();

        self.vault_proxy()
            .contract(to)
            .accept_funds(token, half_payment)
            .with_nft_nonce(token_nonce)
            .with_gas_limit(half_gas)
            .transfer_execute();
    }

    /// Test that the default gas provided to the transfer_execute call
    /// leaves enough in the transaction for finish to happen.
    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_accept_funds_return_values(
        &self,
        to: Address,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] payment: Self::BigUint,
        #[payment_nonce] token_nonce: u64,
    ) -> MultiResult4<u64, u64, Self::BigUint, TokenIdentifier> {
        let gas_left_before = self.blockchain().get_gas_left();

        self.vault_proxy()
            .contract(to)
            .accept_funds(token.clone(), payment)
            .with_nft_nonce(token_nonce)
            .transfer_execute();

        let gas_left_after = self.blockchain().get_gas_left();

        (
            gas_left_before,
            gas_left_after,
            Self::BigUint::zero(),
            token,
        )
            .into()
    }

    #[endpoint]
    fn forward_transf_exec_accept_funds_multi_transfer(
        &self,
        to: Address,
        #[var_args] token_payments: VarArgs<MultiArg3<TokenIdentifier, u64, Self::BigUint>>,
    ) {
        let mut all_token_payments = Vec::new();

        for multi_arg in token_payments.into_vec() {
            let (token_name, token_nonce, amount) = multi_arg.into_tuple();
            let payment = EsdtTokenPayment {
                token_name,
                token_nonce,
                amount,
                token_type: EsdtTokenType::Invalid, // not used
            };

            all_token_payments.push(payment);
        }

        self.vault_proxy()
            .contract(to)
            .accept_funds(TokenIdentifier::egld(), Self::BigUint::zero())
            .esdt_multi_transfer(&all_token_payments);
    }
}
