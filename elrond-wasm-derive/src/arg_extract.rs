use crate::contract_gen_method::MethodPayableMetadata;

use super::arg_def::*;
use super::parse_attr::*;
// use super::contract_gen_method::*;

pub fn extract_payment(
	mpm: MethodPayableMetadata,
	processed_args: &[MethodArg],
) -> Option<MethodArg> {
	let mut payment_arg = None;
	processed_args.iter().for_each(|ma| {
		if matches!(ma.metadata, ArgMetadata::Payment) {
			if payment_arg.is_some() {
				panic!("only one #[payment] argument allowed");
			}
			if matches!(
				mpm,
				MethodPayableMetadata::NotPayable | MethodPayableMetadata::NoMetadata
			) {
				panic!("#[payment] argument not allowed in non-payable methods");
			}
			payment_arg = Some(ma.clone());
		}
	});
	payment_arg
}

pub fn extract_payment_token(
	mpm: MethodPayableMetadata,
	processed_args: &[MethodArg],
) -> Option<MethodArg> {
	let mut payment_token_arg = None;
	processed_args.iter().for_each(|ma| {
		if matches!(ma.metadata, ArgMetadata::PaymentToken) {
			if payment_token_arg.is_some() {
				panic!("only one #[payment_token] argument allowed");
			}
			if matches!(
				mpm,
				MethodPayableMetadata::NotPayable | MethodPayableMetadata::NoMetadata
			) {
				panic!("#[payment_token] argument not allowed in non-payable methods");
			}
			payment_token_arg = Some(ma.clone());
		}
	});
	payment_token_arg
}

pub fn extract_method_args(m: &syn::TraitItemMethod) -> Vec<MethodArg> {
	let mut arg_index: isize = -1; // ignore the first argument, which is &self
	let mut receiver_processed = false;
	m.sig
		.inputs
		.iter()
		.filter_map(|arg| match arg {
			syn::FnArg::Receiver(ref selfref) => {
				if selfref.mutability.is_some() || receiver_processed {
					panic!("Trait method must have `&self` as its first argument.");
				}
				receiver_processed = true;
				None
			},
			syn::FnArg::Typed(pat_typed) => {
				if !receiver_processed {
					panic!("Trait method must have `&self` as its first argument.");
				}
				let pat = &*pat_typed.pat;
				let ty = &*pat_typed.ty;

				let is_callback_result_arg = is_callback_result_arg(&pat_typed);

				if is_var_args(&pat_typed) {
					Some(MethodArg {
						index: -1,
						pat: pat.clone(),
						ty: ty.clone(),
						metadata: ArgMetadata::VarArgs,
					})
				} else if is_payment(&pat_typed) {
					if is_callback_result_arg {
						panic!("Payment arguments cannot be annotated with #[async_result].");
					}
					Some(MethodArg {
						index: -1,
						pat: pat.clone(),
						ty: ty.clone(),
						metadata: ArgMetadata::Payment,
					})
				} else if is_payment_token(&pat_typed) {
					if is_callback_result_arg {
						panic!("Payment arguments cannot be annotated with #[async_result].");
					}
					Some(MethodArg {
						index: -1,
						pat: pat.clone(),
						ty: ty.clone(),
						metadata: ArgMetadata::PaymentToken,
					})
				} else if is_callback_result_arg {
					Some(MethodArg {
						index: -1,
						pat: pat.clone(),
						ty: ty.clone(),
						metadata: ArgMetadata::AsyncCallResultArg,
					})
				} else {
					arg_index += 1;
					Some(MethodArg {
						index: arg_index as i32,
						pat: pat.clone(),
						ty: ty.clone(),
						metadata: ArgMetadata::Single,
					})
				}
			},
		})
		.collect()
}
