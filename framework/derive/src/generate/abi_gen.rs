use crate::model::ContractTrait;

pub fn generate_abi_provider(
    contract: &ContractTrait,
    is_contract_main: bool,
) -> proc_macro2::TokenStream {
    multiversx_sc_abi_derive_common::contract::abi_gen::generate_abi_provider(
        contract,
        is_contract_main,
        multiversx_sc_abi_derive_common::TypeAbiImportCrate::MultiversxSc,
        &quote! { multiversx_sc::contract_base::ContractAbiProvider },
        quote! { type Api = multiversx_sc::api::uncallable::UncallableApi; },
    )
}
