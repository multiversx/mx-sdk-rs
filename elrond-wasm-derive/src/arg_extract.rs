use super::arg_def::*;
use super::parse_attr::*;

pub fn extract_method_args(
	m: &syn::TraitItemMethod,
	is_method_payable: bool,
	allow_callback_args: bool,
) -> Vec<MethodArg> {
	let mut arg_index: isize = -1; // ignore the first argument, which is &self
	let mut receiver_processed = false;
	m.sig
		.inputs
		.iter()
		.filter_map(|arg| {
			match arg {
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

					let is_callback_arg = is_callback_arg(&pat_typed);
					if is_callback_arg && !allow_callback_args {
						panic!("Callback args not allowed here");
					}

					if let Some(multi_attr) = MultiAttribute::parse(&pat_typed) {
						Some(MethodArg {
							index: -1,
							pat: pat.clone(),
							ty: ty.clone(),
							is_callback_arg,
							metadata: ArgMetadata::Multi(multi_attr),
						})
					} else if is_var_args(&pat_typed) {
						Some(MethodArg {
							index: -1,
							pat: pat.clone(),
							ty: ty.clone(),
							is_callback_arg,
							metadata: ArgMetadata::VarArgs,
						})
					} else if is_payment(&pat_typed) {
						if !is_method_payable {
							panic!("Cannot have payment arguments to non-payable methods.");
						}
						if is_callback_arg {
							panic!("Payment arguments cannot be annotated with #[callback_arg].");
						}
						Some(MethodArg {
							index: -1,
							pat: pat.clone(),
							ty: ty.clone(), // TODO: check that it is BigUint
							is_callback_arg,
							metadata: ArgMetadata::Payment,
						})
					} else {
						arg_index += 1;
						Some(MethodArg {
							index: arg_index as i32,
							pat: pat.clone(),
							ty: ty.clone(),
							is_callback_arg,
							metadata: ArgMetadata::Single,
						})
					}
				},
			}
		})
		.collect()
}
