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

    if wasm32_mode() {
        // release mode adds endpoints for wasmer 
        proc_macro::TokenStream::from(quote! {
            #[macro_use]
            extern crate elrond_wasm;
    
            #contract_impl

            #[no_mangle]
            pub fn callBack () {
            let inst = new_arwen_instance();
            inst.callback();
            }
        })
      } else {
        // debug mode adds the contract interface, that we use for the mocks
        // this interface also relies on "call" methods with no parameter and a function selector
        proc_macro::TokenStream::from(quote! {
            #[macro_use]
            extern crate elrond_wasm;
    
            #contract_impl
        })
      }
}