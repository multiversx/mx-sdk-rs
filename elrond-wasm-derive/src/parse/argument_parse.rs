use super::attributes::*;
use crate::model::{ArgMetadata, ArgPaymentMetadata, MethodArgument};

pub fn extract_method_args(m: &syn::TraitItemMethod) -> Vec<MethodArgument> {
	if m.sig.inputs.is_empty() {
		missing_self_panic(m);
	}

	let mut receiver_processed = false;
	m.sig
		.inputs
		.iter()
		.filter_map(|arg| match arg {
			syn::FnArg::Receiver(ref selfref) => {
				if selfref.mutability.is_some() || receiver_processed {
					missing_self_panic(m);
				}
				receiver_processed = true;
				None
			},
			syn::FnArg::Typed(pat_typed) => {
				if !receiver_processed {
					missing_self_panic(m);
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
					pat: pat.clone(),
					ty: ty.clone(),
					remaining_attributes: Vec::new(),
					metadata,
				};

				Some(arg)
			},
		})
		.collect()
}

fn missing_self_panic(m: &syn::TraitItemMethod) -> ! {
	panic!(
		"Trait method `{}` must have `&self` as its first argument.",
		m.sig.ident.to_string()
	)
}
