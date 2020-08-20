use super::*;
use super::contract_gen::*;
use super::contract_impl::*;

pub fn process_contract(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

    let args_input = parse_macro_input!(args as syn::AttributeArgs);
    let proc_input = &parse_macro_input!(input as syn::ItemTrait);

    let contract = Contract::new(args_input, proc_input);

    let contract_impl = contract_implementation(&contract);
    let contract_impl_ident = contract.contract_impl_name.clone();

    let wasm_callback_endpoint = quote!{
        pub mod callback_endpoint {
            use super::*;
            use elrond_wasm_node::*;

            fn new_arwen_instance() -> #contract_impl_ident<ArwenApiImpl, ArwenBigInt, ArwenBigUint> {
                let api = ArwenApiImpl{};
                #contract_impl_ident::new(api)
            }

            #[no_mangle]
            #[allow(non_snake_case)]
            pub fn callBack () {
                let inst = new_arwen_instance();
                inst.callback();
            }
        }
    };

    proc_macro::TokenStream::from(quote! {
        #[macro_use]
        extern crate elrond_wasm;

        #contract_impl

        #wasm_callback_endpoint
    })
}