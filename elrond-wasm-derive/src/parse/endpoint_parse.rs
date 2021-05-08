use crate::model::{CallbackMetadata, EndpointMetadata, InitMetadata, PublicRole};

use super::{
	attributes::{
		is_callback_raw_decl, is_init, CallbackAttribute, EndpointAttribute, ViewAttribute,
	},
	process_payable,
};

pub fn process_public_role(m: &syn::TraitItemMethod) -> PublicRole {
	let endpoint_attr_opt = EndpointAttribute::parse(m);
	let view_attr_opt = ViewAttribute::parse(m);
	let callback_attr_opt = CallbackAttribute::parse(m);
	let callback_raw = is_callback_raw_decl(m);

	let payable = process_payable(m);

	// init
	let init = is_init(m);
	if init {
		if endpoint_attr_opt.is_some() {
			panic!("Cannot annotate with both #[init] and #[endpoint].");
		}
		if view_attr_opt.is_some() {
			panic!("Cannot annotate with both #[init] and #[view].");
		}
		if callback_attr_opt.is_some() {
			panic!("Cannot annotate with both #[init] and #[callback].");
		}
		if callback_raw {
			panic!("Cannot annotate with both #[init] and #[callback_raw].");
		}
		return PublicRole::Init(InitMetadata { payable });
	}

	// endpoint
	if let Some(endpoint_attr) = endpoint_attr_opt {
		if view_attr_opt.is_some() {
			panic!("Cannot annotate with both #[endpoint] and #[view].");
		}
		if callback_attr_opt.is_some() {
			panic!("Cannot annotate with both #[endpoint] and #[callback].");
		}
		if callback_raw {
			panic!("Cannot annotate with both #[endpoint] and #[callback_raw].");
		}
		let endpoint_ident = match endpoint_attr.endpoint_name {
			Some(ident) => ident,
			None => m.sig.ident.clone(),
		};
		return PublicRole::Endpoint(EndpointMetadata {
			public_name: endpoint_ident,
			payable,
		});
	}

	// view
	if let Some(view_attr) = view_attr_opt {
		if callback_attr_opt.is_some() {
			panic!("Cannot annotate with both #[view] and #[callback].");
		}
		if callback_raw {
			panic!("Cannot annotate with both #[view] and #[callback_raw].");
		}
		let view_ident = match view_attr.view_name {
			Some(ident) => ident,
			None => m.sig.ident.clone(),
		};
		return PublicRole::Endpoint(EndpointMetadata {
			public_name: view_ident,
			payable,
		});
	}

	if let Some(callback_attr) = callback_attr_opt {
		if callback_raw {
			panic!("Cannot annotate with both #[callback] and #[callback_raw].");
		}
		let callback_ident = match callback_attr.callback_name {
			Some(ident) => ident,
			None => m.sig.ident.clone(),
		};
		return PublicRole::Callback(CallbackMetadata {
			callback_name: callback_ident,
		});
	}

	if callback_raw {
		return PublicRole::CallbackRaw;
	}

	PublicRole::Private
}
