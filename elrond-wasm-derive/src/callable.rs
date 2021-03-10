use crate::generate::callable_gen::{extract_pub_method_sigs, generate_method_impl};
use crate::parse::parse_callable_trait;

pub fn process_callable(
	args: proc_macro::TokenStream,
	input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let args_input = parse_macro_input!(args as syn::AttributeArgs);
	let proc_input = parse_macro_input!(input as syn::ItemTrait);

	let callable = parse_callable_trait(args_input, &proc_input);
	// TODO: add validation

	let method_sigs = extract_pub_method_sigs(&callable);
	let trait_name = callable.trait_name.clone();
	let callable_impl_name = callable.contract_impl_name.clone();
	//let contract_impl_name = callable.contract_impl_name.clone();

	let method_impls = generate_method_impl(&callable);

	// this definition is common to release and debug mode
	let main_definition = quote! {
		pub trait #trait_name<BigInt, BigUint>
		where
			BigUint: BigUintApi + 'static,
			BigInt: BigIntApi<BigUint> + 'static,
		{
			#(#method_sigs)*
		}

		pub struct #callable_impl_name<SA, BigInt, BigUint>
		where
			BigUint: BigUintApi + 'static,
			BigInt: BigIntApi<BigUint> + 'static,
			SA: SendApi<BigUint> + Clone + 'static,
		{
			pub api: SA,
			pub address: Address,
			pub token: elrond_wasm::types::TokenIdentifier,
			pub payment: BigUint,
			_phantom1: core::marker::PhantomData<BigInt>,
			_phantom2: core::marker::PhantomData<BigUint>,
		}

		impl<SA, BigInt, BigUint> elrond_wasm::types::ContractProxy<SA, BigInt, BigUint> for #callable_impl_name<SA, BigInt, BigUint>
		where
			BigUint: BigUintApi + 'static,
			BigInt: BigIntApi<BigUint> + 'static,
			SA: SendApi<BigUint> + Clone + 'static,
		{
			fn new(api: SA, address: Address) -> Self {
				#callable_impl_name {
					api,
					address,
					token: elrond_wasm::types::TokenIdentifier::egld(),
					payment: BigUint::zero(),
					_phantom1: core::marker::PhantomData,
					_phantom2: core::marker::PhantomData,
				}
			}

			fn with_token_transfer(mut self, token: TokenIdentifier, payment: BigUint) -> Self {
				self.token = token;
				self.payment = payment;
				self
			}
		}

		impl<SA, BigInt, BigUint> #trait_name<BigInt, BigUint> for #callable_impl_name<SA, BigInt, BigUint>
		where
			BigUint: BigUintApi + 'static,
			BigInt: BigIntApi<BigUint> + 'static,
			SA: SendApi<BigUint> + Clone + 'static,
		{
			#(#method_impls)*
		}
	};

	proc_macro::TokenStream::from(quote! {
	  #main_definition
	})
}
