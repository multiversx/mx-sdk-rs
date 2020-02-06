use super::*;
use super::gen::*;

pub fn process_contract(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

    let proc_input = &parse_macro_input!(input as syn::ItemTrait);
    let contract = Contract::new(proc_input);

    let contract_struct = contract.struct_name.clone();
    let trait_name = contract.trait_name.clone();
    let method_impls = contract.extract_method_impls();

    let call_methods = contract.generate_call_methods();
    let event_defs = contract.generate_event_defs();
    let event_impls = contract.generate_event_impls();
    let endpoints = contract.generate_endpoints();
    let function_selector_body = contract.generate_function_selector_body();

    let bi_where = quote! {
      where 
          BigInt: BigIntApi + 'static,
          BigUint: BigUintApi<BigInt> + 'static,
          for<'b> BigInt: AddAssign<&'b BigInt>,
          for<'b> BigInt: SubAssign<&'b BigInt>,
          for<'b> BigInt: MulAssign<&'b BigInt>,
    };

    let api_where = quote! {
      #bi_where
        T: ContractHookApi<BigInt> + ContractIOApi<BigInt, BigUint> + Clone + 'static
    };

    // this definition is common to release and debug mode
    let main_definition = quote! {
      use elrond_wasm;
      use elrond_wasm::Address;
      use elrond_wasm::StorageKey;
      use elrond_wasm::ErrorMessage;
      use elrond_wasm::ContractHookApi;
      use elrond_wasm::ContractIOApi;
      use elrond_wasm::BigIntApi;
      use elrond_wasm::BigUintApi;
      use core::ops::{AddAssign, SubAssign, MulAssign};

      pub trait #trait_name<BigInt, BigUint>: ContractHookApi<BigInt> + Sized 
      #bi_where
      {
        #(#method_impls)*

        #(#event_defs)*
      }

      pub struct #contract_struct<T, BigInt, BigUint>
      #api_where
      {
          api: T,
          _phantom1: BigInt,
          _phantom2: BigUint,
      }

      impl <T, BigInt, BigUint> #contract_struct<T, BigInt, BigUint>
      where 
          BigInt: BigIntApi + 'static,
          BigUint: BigUintApi<BigInt> + 'static,
          for<'b> BigInt: AddAssign<&'b BigInt>,
          for<'b> BigInt: SubAssign<&'b BigInt>,
          for<'b> BigInt: MulAssign<&'b BigInt>,
          T: ContractHookApi<BigInt> + ContractIOApi<BigInt, BigUint> + Clone + 'static
      {
        pub fn new(api: T) -> Self {
          #contract_struct {
            api: api,
            _phantom1: BigInt::phantom(), // TODO: figure out a way to make this an *ACTUAL* phantom in no_std
            _phantom2: BigUint::phantom(),
          }
        }
      }

      impl <T, BigInt, BigUint> ContractHookApi<BigInt> for #contract_struct<T, BigInt, BigUint>
      #api_where
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
        fn storage_store(&self, key: &StorageKey, value: &Vec<u8>) {
          self.api.storage_store(key, value);
        }

        #[inline]
        fn storage_load(&self, key: &StorageKey) -> Vec<u8> {
          self.api.storage_load(key)
        }

        #[inline]
        fn storage_store_bytes32(&self, key: &StorageKey, value: &[u8; 32]) {
          self.api.storage_store_bytes32(key, value);
        }
        
        #[inline]
        fn storage_load_bytes32(&self, key: &StorageKey) -> [u8; 32] {
          self.api.storage_load_bytes32(key)
        }
    
        #[inline]
        fn storage_store_big_int(&self, key: &StorageKey, value: &BigInt) {
          self.api.storage_store_big_int(key, value);
        }
        
        #[inline]
        fn storage_load_big_int(&self, key: &StorageKey) -> BigInt {
          self.api.storage_load_big_int(key)
        }
        
        #[inline]
        fn get_call_value_big_int(&self) -> BigInt {
          self.api.get_call_value_big_int()
        }

        #[inline]
        fn send_tx(&self, to: &Address, amount: &BigInt, message: &str) {
          self.api.send_tx(to, amount, message);
        }

        #[inline]
        fn get_gas_left(&self) -> i64 {
          self.api.get_gas_left()
        }

        #[inline]
        fn sha256(&self, data: &Vec<u8>) -> [u8; 32] {
          self.api.sha256(data)
        }
    
        #[inline]
        fn keccak256(&self, data: &Vec<u8>) -> [u8; 32] {
          self.api.keccak256(data)
        }
      }

      impl <T, BigInt, BigUint> #trait_name<BigInt, BigUint> for #contract_struct<T, BigInt, BigUint> 
      #api_where
      {
        #(#event_impls)*
      }

      impl <T, BigInt, BigUint> #contract_struct<T, BigInt, BigUint>
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

        fn new_arwen_instance() -> #contract_struct<ArwenApiImpl, ArwenBigInt, ArwenBigUint> {
          #contract_struct {
            api: ArwenApiImpl{},
            _phantom1: ArwenBigInt::phantom(), // TODO: figure out a way to make this an *ACTUAL* phantom in no_std
            _phantom2: ArwenBigUint::phantom(),
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
        impl <T, BigInt, BigUint> CallableContract for #contract_struct<T, BigInt, BigUint> 
        #api_where
        {
          fn call(&self, fn_name: &'static str) {
            #function_selector_body
          }
  
          fn clone_contract(&self) -> Box<dyn CallableContract> {
            Box::new(#contract_struct {
              api: self.api.clone(),
              _phantom1: BigInt::phantom(), // TODO: figure out a way to make this an *ACTUAL* phantom in no_std
              _phantom2: BigUint::phantom(),
            })
          }
        }
      })
    }
}
