use crate::{
    generate::{contract_gen::*, proxy_gen, supertrait_gen},
    model::ContractTrait,
    parse::parse_contract_trait,
    preprocessing::trait_preprocessing,
    validate::validate_contract,
};

pub fn process_proxy(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let new_input = trait_preprocessing(input);
    let args_input = parse_macro_input!(args as syn::AttributeArgs);
    let proc_input = parse_macro_input!(new_input as syn::ItemTrait);

    let contract = parse_contract_trait(args_input, &proc_input);
    validate_contract(&contract);

    let proxy_impl = proxy_implementation(&contract, true);

    proc_macro::TokenStream::from(quote! {
      #proxy_impl
    })
}

pub fn proxy_implementation(
    contract: &ContractTrait,
    is_contract_main: bool,
) -> proc_macro2::TokenStream {
    let trait_name_ident = contract.trait_name.clone();
    let method_impls = extract_method_impls(contract);

    // this definition is common to release and debug mode
    let supertraits_main = supertrait_gen::main_supertrait_decl(contract.supertraits.as_slice());
    let main_definition = quote! {
        pub trait #trait_name_ident:
        multiversx_sc::contract_base::ContractBase
        + Sized
        #(#supertraits_main)*
        where
        {
            #(#method_impls)*
        }
    };

    let proxy_trait = proxy_gen::proxy_trait(contract);
    let proxy_obj_code = if is_contract_main {
        proxy_gen::proxy_obj_code(contract)
    } else {
        quote! {}
    };

    quote! {
        #main_definition

        #proxy_trait

        #proxy_obj_code
    }
}
