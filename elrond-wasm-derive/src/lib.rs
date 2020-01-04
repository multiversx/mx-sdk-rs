

#![allow(dead_code)]
#![allow(stable_features)]

// ensure we don't run out of macro stack
#![recursion_limit="1024"]

extern crate proc_macro;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

mod utils;
use utils::*;

fn wasm32_mode() -> bool {
  // this checks if we set --release or not in the command line
  // we should always set --release when building sc wasm and never when running the debugger, so this works
  let debug_mode = cfg!(debug_assertions);
  !debug_mode

  // this is supposed to check whether or not the target starts with "wasm32-...
  // for some reason this no longer works, TODO: investigate
  //cfg!(target_arch = "wasm32")

  // when debugging the macro output, the above methods don't seem to work
  // so just temporarily hardcode while bugfixing
  //true
}

#[proc_macro_attribute]
pub fn contract(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

    let proc_input = &parse_macro_input!(input as syn::ItemTrait);
    let contract = Contract::new(proc_input);

    let contract_struct = contract.struct_name.clone();
    let trait_name = contract.trait_name.clone();
    let method_sigs = contract.extract_method_sigs();
    let method_impls = contract.extract_method_impls();

    let call_methods = contract.generate_call_methods();
    let endpoints = contract.generate_endpoints();
    let function_selector_body = contract.generate_function_selector_body();

    // this definition is common to release and debug mode
    let main_definition = quote! {
      use elrond_wasm;
      use elrond_wasm::Address;
      use elrond_wasm::StorageKey;
      use elrond_wasm::ContractHookApi;
      use elrond_wasm::ContractIOApi;
      use elrond_wasm::BigIntApi;
      use elrond_wasm_node::ArwenBigInt;
      use elrond_wasm_node::*;
      use core::ops::{AddAssign, SubAssign};

      pub trait #trait_name<BI>: ContractHookApi<BI> where BI: BigIntApi {
        #(#method_sigs)*
      }

      pub struct #contract_struct<T, BI>
      where 
          BI: BigIntApi + 'static,
          for<'b> BI: AddAssign<&'b BI>,
          for<'b> BI: SubAssign<&'b BI>,
          T: ContractHookApi<BI> + ContractIOApi<BI> + Clone + 'static
      {
          api: T,
          _phantom: BI
      }

      impl <T, BI> #contract_struct<T, BI>
      where 
          BI: BigIntApi + 'static,
          for<'b> BI: AddAssign<&'b BI>,
          for<'b> BI: SubAssign<&'b BI>,
          T: ContractHookApi<BI> + ContractIOApi<BI> + Clone + 'static
      {
        pub fn new(api: T) -> Self {
          #contract_struct {
            api: api,
            _phantom: BI::from(0), // TODO: figure out a way to make this an *ACTUAL* phantom in no_std
          }
        }
      }

      impl <T, BI> ContractHookApi<BI> for #contract_struct<T, BI>
      where 
          BI: BigIntApi + 'static,
          for<'b> BI: AddAssign<&'b BI>,
          for<'b> BI: SubAssign<&'b BI>,
          T: ContractHookApi<BI> + ContractIOApi<BI> + Clone + 'static
      {
        #[inline]
        fn get_owner(&self) -> Address {
          self.api.get_owner()
        }

        #[inline]
        fn get_caller(&self) -> Address {
          self.api.get_caller()
        }

        #[inline]
        fn signal_error(&self) {
          self.api.signal_error();
        }
    
        #[inline]
        fn write_log(&self, topics: &[[u8;32]], data: &[u8]) {
          self.api.write_log(topics, data);
        }
    
        #[inline]
        fn storage_store_big_int(&self, key: &StorageKey, value: &BI) {
          self.api.storage_store_big_int(key, value);
        }
        
        #[inline]
        fn storage_load_big_int(&self, key: &StorageKey) -> BI {
          self.api.storage_load_big_int(key)
        }
        
        #[inline]
        fn get_call_value_big_int(&self) -> BI {
          self.api.get_call_value_big_int()
        }  
      }

      impl <T, BI> #trait_name<BI> for #contract_struct<T, BI> 
      where 
          BI: BigIntApi + 'static,
          for<'b> BI: AddAssign<&'b BI>,
          for<'b> BI: SubAssign<&'b BI>,
          T: ContractHookApi<BI> + ContractIOApi<BI> + Clone + 'static
      {
        #(#method_impls)*
      }

      impl <T, BI> #contract_struct<T, BI>
        where 
            BI: BigIntApi + 'static,
            for<'b> BI: AddAssign<&'b BI>,
            for<'b> BI: SubAssign<&'b BI>,
            T: ContractHookApi<BI> + ContractIOApi<BI> + Clone + 'static
      {
        #(#call_methods)*
      }

    };

    if wasm32_mode() {
      // release mode adds endpoints for wasmer 
      proc_macro::TokenStream::from(quote! {
        #main_definition

        fn new_arwen_instance() -> #contract_struct<ArwenApiImpl, ArwenBigInt> {
          #contract_struct {
            api: ArwenApiImpl{},
            _phantom: ArwenBigInt::from(0), // TODO: figure out a way to make this an *ACTUAL* phantom in no_std
          }
        }

        #(#endpoints)*       
      })
    } else {
      // debug mode adds the contract interface, that we use for the mocks
      // this interface also relies on "call" methods with no parameter and a function selector
      proc_macro::TokenStream::from(quote! {
        #main_definition
  
        use elrond_wasm::CallableContract;
        impl <T, BI> CallableContract for #contract_struct<T, BI> 
        where 
            BI: BigIntApi + 'static,
            for<'b> BI: AddAssign<&'b BI>,
            for<'b> BI: SubAssign<&'b BI>,
            T: ContractHookApi<BI> + ContractIOApi<BI> + Clone + 'static
        {
          fn call(&self, fn_name: &'static str) {
            #function_selector_body
          }
  
          fn clone_contract(&self) -> Box<dyn CallableContract> {
            Box::new(#contract_struct {
              api: self.api.clone(),
              _phantom: BI::from(0), // TODO: figure out a way to make this an *ACTUAL* phantom in no_std
            })
          }
        }
      })
    }
}
