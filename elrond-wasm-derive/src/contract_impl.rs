use super::*;
use super::contract_gen::*;
use super::function_selector::*;

pub fn contract_implementation(
    contract: &Contract,
) -> proc_macro2::TokenStream {

    let contract_impl_ident = contract.contract_impl_name.clone();
    let trait_name_ident = contract.trait_name.clone();
    let method_impls = contract.extract_method_impls();

    if !contract.supertrait_paths.is_empty() {
      panic!("contract inheritance currently not supported");
    }

    let call_methods = contract.generate_call_methods();
    let auto_impl_defs = contract.generate_auto_impl_defs();
    let auto_impls = contract.generate_auto_impls();
    let endpoints = contract.generate_endpoints();
    let function_selector_body = generate_function_selector_body(&contract);
    let callback_body = contract.generate_callback_body();
    let api_where = snippets::api_where();

    let supertrait_impls = contract.generate_supertrait_impls();
    let contract_trait_api_impl = snippets::contract_trait_api_impl(&contract_impl_ident);

    // this definition is common to release and debug mode
    let main_definition = quote! {
      pub trait #trait_name_ident<T, BigInt, BigUint>: 
      ContractHookApi<BigInt, BigUint>
      // #( + #supertrait_paths <T, BigInt, BigUint>)* // currently not supported
      + Sized 
      #api_where
      {
        #(#method_impls)*

        #(#auto_impl_defs)*

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

      impl <T, BigInt, BigUint> #contract_impl_ident<T, BigInt, BigUint>
      #api_where
      {
        pub fn new(api: T) -> Self {
          #contract_impl_ident {
            api,
            _phantom1: core::marker::PhantomData,
            _phantom2: core::marker::PhantomData,
          }
        }
      }

      #contract_trait_api_impl

      #(#supertrait_impls)*

      impl <T, BigInt, BigUint> #trait_name_ident<T, BigInt, BigUint> for #contract_impl_ident<T, BigInt, BigUint> 
      #api_where
      {
        #(#auto_impls)*

        fn contract_proxy(&self, address: &Address) -> Box<OtherContractHandle<T, BigInt, BigUint>> {
          let contract_proxy = OtherContractHandle::new(self.api.clone(), address);
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

    let wasm_endpoints = quote! {
        #[cfg(feature = "wasm-output-mode")]
        #[allow(non_snake_case)]
        pub mod endpoints {
          use super::*;
          use elrond_wasm_node::*;

          fn new_arwen_instance() -> #contract_impl_ident<ArwenApiImpl, ArwenBigInt, ArwenBigUint> {
            let api = ArwenApiImpl{};
            #contract_impl_ident::new(api)
          }

          #(#endpoints)*
        }
    };

    let function_selector = quote! {
      use elrond_wasm::CallableContract;
      impl <T, BigInt, BigUint> CallableContract<T> for #contract_impl_ident<T, BigInt, BigUint> 
      #api_where
      {
        fn call(&self, fn_name: &[u8]) -> bool {
          #function_selector_body
        }

        fn clone_contract(&self) -> Box<dyn CallableContract<T>> {
          Box::new(#contract_impl_ident::new(self.api.clone()))
        }

        fn into_api(self: Box<Self>) -> T {
          self.api
        }
      }
    };

    quote! {
      #main_definition

      #wasm_endpoints

      #function_selector
    }
}
