#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// contract to help with snippet generation tests
// contains endpoints with various types combinations
// TODO: Split into modules when testing the auto-imports

// To generate snippets, change dir to the `meta` folder and run:
// cargo run snippets
// Add --overwrite if you want to overwrite existing snippets

// Additionally, we also have to update the interact-rs snippets manually to add relative paths:
// [dependencies.multiversx-sc-snippets]
// version = "0.43.5"
// path = "../../../../framework/snippets"

#[derive(
    TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Debug, Clone,
)]
pub struct MyCoolStruct<M: ManagedTypeApi> {
    pub awesome: BaseBigUint<M>,
}

#[multiversx_sc::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn no_arg_no_result_endpoint(&self) {}

    #[endpoint]
    fn no_arg_one_result_endpoint(&self) -> u64 {
        0
    }

    #[endpoint]
    fn one_arg_no_result_endpoint(&self, _arg: u64) {}

    #[endpoint]
    fn one_arg_one_result_endpoint(&self, _arg: u64) -> BaseBigUint {
        BaseBigUint::zero()
    }

    #[endpoint]
    fn multi_result(&self, _arg: TokenIdentifier) -> MultiValueEncoded<BaseBigUint> {
        MultiValueEncoded::new()
    }

    #[endpoint]
    fn nested_result(&self, _arg: TokenIdentifier) -> ManagedVec<ManagedVec<BaseBigUint>> {
        ManagedVec::new()
    }

    #[endpoint]
    fn custom_struct(&self, _arg: MyCoolStruct<CurrentApi>) -> MyCoolStruct<CurrentApi> {
        MyCoolStruct {
            awesome: BaseBigUint::zero(),
        }
    }

    #[endpoint]
    fn optional_type(&self, _arg: OptionalValue<BaseBigUint>) -> OptionalValue<TokenIdentifier> {
        OptionalValue::None
    }

    #[endpoint]
    fn option_type(&self, _arg: Option<ManagedVec<TokenIdentifier>>) -> Option<u64> {
        None
    }

    #[endpoint]
    fn esdt_token_payment(&self, _arg: OptionalValue<EsdtTokenPayment>) -> EsdtTokenPayment {
        EsdtTokenPayment::new(
            TokenIdentifier::from_esdt_bytes(b"COOL-123456"),
            0,
            BaseBigUint::from(1_000u64),
        )
    }

    #[endpoint]
    fn egld_or_esdt_payment(&self, arg: EgldOrEsdtTokenPayment) -> EgldOrEsdtTokenIdentifier {
        arg.token_identifier
    }

    #[payable("*")]
    #[endpoint]
    fn payable_endpoint(&self) {}

    #[endpoint]
    fn managed_buffer(
        &self,
        _arg: Option<ManagedBuffer>,
    ) -> MultiValueEncoded<ManagedVec<MyCoolStruct<CurrentApi>>> {
        MultiValueEncoded::new()
    }

    #[endpoint]
    fn multi_value_2(&self, arg: MultiValue2<u64, BaseBigUint>) -> MultiValue2<u64, BaseBigUint> {
        arg
    }

    #[endpoint]
    fn multi_value_4(
        &self,
        arg: MultiValue4<u64, BaseBigUint, MyCoolStruct<CurrentApi>, TokenIdentifier>,
    ) -> MultiValue4<u64, BaseBigUint, MyCoolStruct<CurrentApi>, TokenIdentifier> {
        arg
    }

    #[endpoint]
    fn complex_multi_values(
        &self,
        arg: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BaseBigUint>>,
    ) -> MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BaseBigUint>> {
        arg
    }
}
