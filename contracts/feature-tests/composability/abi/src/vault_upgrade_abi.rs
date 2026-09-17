use multiversx_sc_abi::imports::*;

#[multiversx_sc_abi::contract_abi(call = VaultUpgradeCall)]
pub trait VaultUpgradeAbi {
    #[upgrade]
    fn upgrade(
        &self,
        opt_arg_to_echo: OptionalValue<BytesAbi>,
    ) -> MultiValue2<StringAbi, OptionalValue<BytesAbi>>;
}
