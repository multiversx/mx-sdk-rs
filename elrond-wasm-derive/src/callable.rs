use super::*;
use super::callable_gen::*;

pub fn process_callable(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

    let args_input = parse_macro_input!(args as syn::AttributeArgs);
    let proc_input = parse_macro_input!(input as syn::ItemTrait);

    let callable = Callable::new(args_input, &proc_input);

    let method_sigs = callable.extract_method_sigs();
    let trait_name = callable.trait_name.clone();
    //let callable_impl_name = callable.contract_impl_name.clone();
    //let contract_impl_name = callable.contract_impl_name.clone();

    let method_impls = callable.generate_method_impl();

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
        T: ContractHookApi<BigInt> + ContractIOApi<BigInt, BigUint> + Clone + 'static,
    };

    // this definition is common to release and debug mode
    let main_definition = quote! {
      pub trait #trait_name<BigInt, BigUint>
      #bi_where
      {
        #(#method_sigs)*
      }

      impl<T, BigInt, BigUint> #trait_name<BigInt, BigUint> for OtherContractHandle<T, BigInt, BigUint>
      #api_where
      {
        #(#method_impls)*
      }
    };

    if wasm32_mode() {
      // release mode adds endpoints for wasmer 
      proc_macro::TokenStream::from(quote! {
        #main_definition
      })
    } else {
      // debug mode adds the contract interface, that we use for the mocks
      // this interface also relies on "call" methods with no parameter and a function selector
      proc_macro::TokenStream::from(quote! {
        #main_definition
      })
    }
}
