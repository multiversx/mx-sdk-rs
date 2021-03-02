use crate::model::{ArgMetadata, ArgPaymentMetadata, MethodArgument};

use super::attributes::*;

// use super::arg_def::*;
// use super::parse_attr::*;
// use super::contract_gen_method::*;


pub fn extract_method_args(m: &syn::TraitItemMethod) -> Vec<MethodArgument> {
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

				let payment = is_payment(pat_typed);
				let payment_token = is_payment_token(&pat_typed);
				let payment_metadata = if payment {
					if payment_token {
						panic!("arguments cannot be annotated with both `#[payment]` and `#[payment_token]`")
					}
					ArgPaymentMetadata::Payment
				} else if payment_token {
					ArgPaymentMetadata::PaymentToken
				} else {
					ArgPaymentMetadata::NotPayment
				};

				let metadata = ArgMetadata {
					payment: payment_metadata,
					var_args: is_var_args(&pat_typed),
					callback_call_result: is_callback_result_arg(&pat_typed),
					event_topic: is_event_topic(&pat_typed),
				};
				let arg = MethodArgument {
					index: -1,
					pat: pat.clone(),
					ty: ty.clone(),
				    remaining_attributes: Vec::new(),
					metadata,
				};

				Some(arg)

				// if  {
				// 	Some(MethodArgument {
				// 		index: -1,
				// 		pat: pat.clone(),
				// 		ty: ty.clone(),
				// 		metadata: ArgMetadata::VarArgs,
				// 		event_topic,
				// 	})
				// } else if  {
				// 	if is_callback_result_arg {
				// 		panic!("Payment arguments cannot be annotated with #[async_result].");
				// 	}
				// 	Some(MethodArgument {
				// 		index: -1,
				// 		pat: pat.clone(),
				// 		ty: ty.clone(),
				// 		metadata: ArgMetadata::Payment,
				// 		event_topic,
				// 	})
				// } else if is_payment_token(&pat_typed) {
				// 	if is_callback_result_arg {
				// 		panic!("Payment arguments cannot be annotated with #[async_result].");
				// 	}
				// 	Some(MethodArgument {
				// 		index: -1,
				// 		pat: pat.clone(),
				// 		ty: ty.clone(),
				// 		metadata: ArgMetadata::PaymentToken,
				// 		event_topic,
				// 	})
				// } else if is_callback_result_arg {
				// 	Some(MethodArgument {
				// 		index: -1,
				// 		pat: pat.clone(),
				// 		ty: ty.clone(),
				// 		metadata: ArgMetadata::AsyncCallResultArg,
				// 		event_topic,
				// 	})
				// } else {
				// 	arg_index += 1;
				// 	Some(MethodArgument {
				// 		index: arg_index as i32,
				// 		pat: pat.clone(),
				// 		ty: ty.clone(),
				// 		metadata: ArgMetadata::Single,
				// 		event_topic,
				// 	})
				// }
			},
		})
		.collect()
}
