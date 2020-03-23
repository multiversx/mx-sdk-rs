
pub fn contract_imports() -> proc_macro2::TokenStream {
    quote! {
        use elrond_wasm::{Box, Vec, String};
        use elrond_wasm::{Address, StorageKey, ErrorMessage};
        use elrond_wasm::{ContractHookApi, ContractIOApi, BigIntApi, BigUintApi};
        use core::ops::{Add, Sub, Mul, Div, Rem};
        use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
        use core::ops::{BitAnd, BitOr, BitXor, Shr, Shl};
        use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShrAssign, ShlAssign};
    }
}

pub fn big_int_where() -> proc_macro2::TokenStream {
    quote! {
        where 
            BigUint: BigUintApi + 'static,
            for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output=BigUint>,
            for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output=BigUint>,
            for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output=BigUint>,
            for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output=BigUint>,
            for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output=BigUint>,
            for<'b> BigUint: AddAssign<&'b BigUint>,
            for<'b> BigUint: SubAssign<&'b BigUint>,
            for<'b> BigUint: MulAssign<&'b BigUint>,
            for<'b> BigUint: DivAssign<&'b BigUint>,
            for<'b> BigUint: RemAssign<&'b BigUint>,
            for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output=BigUint>,
            for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output=BigUint>,
            for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output=BigUint>,
            for<'b> BigUint: BitAndAssign<&'b BigUint>,
            for<'b> BigUint: BitOrAssign<&'b BigUint>,
            for<'b> BigUint: BitXorAssign<&'b BigUint>,
            for<'a> &'a BigUint: Shr<i32, Output=BigUint>,
            for<'a> &'a BigUint: Shl<i32, Output=BigUint>,

            BigInt: BigIntApi<BigUint> + 'static,
            for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output=BigInt>,
            for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output=BigInt>,
            for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output=BigInt>,
            for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output=BigInt>,
            for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output=BigInt>,
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
        T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + Clone + 'static,
    }
}

pub fn contract_trait_api_impl(contract_struct: &syn::Path) -> proc_macro2::TokenStream {
    let api_where = api_where();
    quote! {
      impl <T, BigInt, BigUint> ContractHookApi<BigInt, BigUint> for #contract_struct<T, BigInt, BigUint>
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
        fn get_balance(&self, address: &Address) -> BigUint {
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
        fn storage_store_big_uint(&self, key: &StorageKey, value: &BigUint) {
          self.api.storage_store_big_uint(key, value);
        }
        
        #[inline]
        fn storage_load_big_uint(&self, key: &StorageKey) -> BigUint {
          self.api.storage_load_big_uint(key)
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
        fn get_call_value_big_uint(&self) -> BigUint {
          self.api.get_call_value_big_uint()
        }

        #[inline]
        fn send_tx(&self, to: &Address, amount: &BigUint, message: &str) {
          self.api.send_tx(to, amount, message);
        }

        #[inline]
        fn async_call(&self, to: &Address, amount: &BigUint, data: &str) {
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
