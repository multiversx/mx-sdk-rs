
pub fn contract_imports() -> proc_macro2::TokenStream {
    quote! {
        use elrond_wasm::{Box, Vec, String};
        use elrond_wasm::{Address, StorageKey, ErrorMessage};
        use elrond_wasm::{ContractHookApi, ContractIOApi, BigIntApi, BigUintApi};
        use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
    }
}

pub fn big_int_where() -> proc_macro2::TokenStream {
    quote! {
        where 
            BigInt: BigIntApi + 'static,
            BigUint: BigUintApi<BigInt> + 'static,
            for<'b> BigInt: AddAssign<&'b BigInt>,
            for<'b> BigInt: SubAssign<&'b BigInt>,
            for<'b> BigInt: MulAssign<&'b BigInt>,
            for<'b> BigInt: DivAssign<&'b BigInt>,
            for<'b> BigInt: RemAssign<&'b BigInt>,
    }
}

pub fn api_where() -> proc_macro2::TokenStream {
    let bi_where = big_int_where();

    quote! {
      #bi_where
        T: ContractHookApi<BigInt> + ContractIOApi<BigInt, BigUint> + Clone + 'static,
    }
}

pub fn contract_trait_api_impl(contract_struct: &syn::Ident) -> proc_macro2::TokenStream {
    let api_where = api_where();
    quote! {
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
        fn get_balance(&self, address: &Address) -> BigInt {
          self.api.get_balance(address)
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
        fn storage_store_i64(&self, key: &StorageKey, value: i64) {
          self.api.storage_store_i64(key, value);
        }
        
        #[inline]
        fn storage_load_i64(&self, key: &StorageKey) -> Option<i64> {
          self.api.storage_load_i64(key)
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
        fn async_call(&self, to: &Address, amount: &BigInt, data: &str) {
          self.api.async_call(to, amount, data);
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
    }
}
