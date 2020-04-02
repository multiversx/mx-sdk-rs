use super::*;
use super::contract_gen::*;

pub fn process_contract(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

    let args_input = parse_macro_input!(args as syn::AttributeArgs);
    let proc_input = &parse_macro_input!(input as syn::ItemTrait);

    let contract = Contract::new(args_input, proc_input);

    let contract_impl_ident = contract.contract_impl_name.clone();
    let trait_name_ident = contract.trait_name.clone();
    let method_impls = contract.extract_method_impls();

    let call_methods = contract.generate_call_methods();
    let event_defs = contract.generate_event_defs();
    let event_impls = contract.generate_event_impls();
    let endpoints = contract.generate_endpoints();
    let function_selector_body = contract.generate_function_selector_body();
    let callback_body = contract.generate_callback_body();

    let contract_imports = snippets::contract_imports();
    let api_where = snippets::api_where();

    let contract_trait_api_impl = snippets::contract_trait_api_impl(&contract_impl_ident);

    // this definition is common to release and debug mode
    let main_definition = quote! {
      #[macro_use]
      extern crate elrond_wasm;

      #contract_imports

      pub trait #trait_name_ident<T, BigInt, BigUint>: ContractHookApi<BigInt, BigUint> + Sized 
      #api_where
      {
        #(#method_impls)*

        #(#event_defs)*

        fn contract_proxy(&self, address: &Address) -> Box<OtherContractHandle<T, BigInt, BigUint>>;

        fn callback(&self);
      }

      pub struct #contract_impl_ident<T, BigInt, BigUint>
      #api_where
      {
          api: T,
          _phantom1: core::marker::PhantomData<BigInt>,
          _phantom2: core::marker::PhantomData<BigUint>,
      }

      pub struct OtherContractHandle<T, BigInt, BigUint>
      #api_where
      {
          api: T,
          address: Address,
          _phantom1: core::marker::PhantomData<BigInt>,
          _phantom2: core::marker::PhantomData<BigUint>,
      }

      impl <T, BigInt, BigUint> #contract_impl_ident<T, BigInt, BigUint>
      #api_where
      {
        pub fn new(api: T) -> Self {
          #contract_impl_ident {
            api: api,
            _phantom1: core::marker::PhantomData,
            _phantom2: core::marker::PhantomData,
          }
        }
      }

      #contract_trait_api_impl

      impl <T, BigInt, BigUint> #trait_name_ident<T, BigInt, BigUint> for #contract_impl_ident<T, BigInt, BigUint> 
      #api_where
      {
        #(#event_impls)*

        fn contract_proxy(&self, address: &Address) -> Box<OtherContractHandle<T, BigInt, BigUint>> {
          let contract_proxy = OtherContractHandle {
            api: self.api.clone(),
            address: address.clone(),
            _phantom1: core::marker::PhantomData,
            _phantom2: core::marker::PhantomData,
          };
          Box::new(contract_proxy)
        }

        fn callback(&self) {
          #callback_body
        }
      }

      impl <T, BigInt, BigUint> #contract_impl_ident<T, BigInt, BigUint>
      #api_where
      {
        #(#call_methods)*
      }

    };

    if wasm32_mode() {
      // release mode adds endpoints for wasmer 
      proc_macro::TokenStream::from(quote! {
        #main_definition

        use elrond_wasm_node::{ArwenBigInt, ArwenBigUint};
        use elrond_wasm_node::*;

        fn new_arwen_instance() -> #contract_impl_ident<ArwenApiImpl, ArwenBigInt, ArwenBigUint> {
          let api = ArwenApiImpl{};
          #contract_impl_ident::new(api)
        }

        #(#endpoints)*

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
        #main_definition
  
        use elrond_wasm::CallableContract;
        impl <T, BigInt, BigUint> CallableContract for #contract_impl_ident<T, BigInt, BigUint> 
        #api_where
        {
          fn call(&self, fn_name: &'static str) {
            #function_selector_body
          }
  
          fn clone_contract(&self) -> Box<dyn CallableContract> {
            Box::new(#contract_impl_ident::new(self.api.clone()))
          }
        }
      })
    }
}
