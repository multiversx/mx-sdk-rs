use crate::model::{
    CallbackMetadata, EndpointMetadata, EndpointMutabilityMetadata, InitMetadata, Method,
    PublicRole,
};

use super::{
    attributes::{
        is_allow_multiple_var_args, is_callback_raw, is_init, is_only_admin, is_only_owner,
        is_only_user_account, is_upgrade, CallbackAttribute, EndpointAttribute,
        ExternalViewAttribute, LabelAttribute, OutputNameAttribute, PromisesCallbackAttribute,
        ViewAttribute,
    },
    MethodAttributesPass1,
};

fn check_single_role(method: &Method) {
    assert!(matches!(method.public_role, PublicRole::Private),
		"Can only annotate with one of the following arguments: `#[init]`, `#[endpoint]`, `#[view]`, `#[callback]`, `#[callback_raw]`, `#[upgrade]`."
	);
}

pub fn process_init_attribute(
    attr: &syn::Attribute,
    pass_1_data: &MethodAttributesPass1,
    method: &mut Method,
) -> bool {
    if is_init(attr) {
        check_single_role(&*method);
        method.public_role = PublicRole::Init(InitMetadata {
            payable: pass_1_data.payable.clone(),
            allow_multiple_var_args: pass_1_data.allow_multiple_var_args,
        });
        true
    } else {
        false
    }
}

pub fn process_upgrade_attribute(
    attr: &syn::Attribute,
    first_pass_data: &MethodAttributesPass1,
    method: &mut Method,
) -> bool {
    let has_attr = is_upgrade(attr);
    if has_attr {
        check_single_role(&*method);
        method.public_role = PublicRole::Endpoint(EndpointMetadata {
            public_name: proc_macro2::Ident::new("upgrade", proc_macro2::Span::call_site()),
            payable: first_pass_data.payable.clone(),
            only_owner: false,
            only_admin: false,
            only_user_account: false,
            mutability: EndpointMutabilityMetadata::Mutable,
            allow_multiple_var_args: first_pass_data.allow_multiple_var_args,
        });
        true
    } else {
        false
    }
}

pub fn process_allow_multiple_var_args_attribute(
    attr: &syn::Attribute,
    pass_1_data: &mut MethodAttributesPass1,
) -> bool {
    let is_allow_multiple_var_args = is_allow_multiple_var_args(attr);
    if is_allow_multiple_var_args {
        pass_1_data.allow_multiple_var_args = true;
    }

    is_allow_multiple_var_args
}

pub fn process_only_owner_attribute(
    attr: &syn::Attribute,
    pass_1_data: &mut MethodAttributesPass1,
) -> bool {
    let is_only_owner = is_only_owner(attr);
    if is_only_owner {
        pass_1_data.only_owner = true;
    }
    is_only_owner
}

pub fn process_only_admin_attribute(
    attr: &syn::Attribute,
    pass_1_data: &mut MethodAttributesPass1,
) -> bool {
    let is_only_admin = is_only_admin(attr);
    if is_only_admin {
        pass_1_data.only_admin = true;
    }
    is_only_admin
}

pub fn process_only_user_account_attribute(
    attr: &syn::Attribute,
    pass_1_data: &mut MethodAttributesPass1,
) -> bool {
    let is_only_user_account = is_only_user_account(attr);
    if is_only_user_account {
        pass_1_data.only_user_account = true;
    }
    is_only_user_account
}

pub fn process_endpoint_attribute(
    attr: &syn::Attribute,
    pass_1_data: &MethodAttributesPass1,
    method: &mut Method,
) -> bool {
    EndpointAttribute::parse(attr)
        .map(|endpoint_attr| {
            check_single_role(&*method);
            let endpoint_ident = match endpoint_attr.endpoint_name {
                Some(ident) => ident,
                None => method.name.clone(),
            };
            method.public_role = PublicRole::Endpoint(EndpointMetadata {
                public_name: endpoint_ident,
                payable: pass_1_data.payable.clone(),
                only_owner: pass_1_data.only_owner,
                only_admin: pass_1_data.only_admin,
                only_user_account: pass_1_data.only_user_account,
                mutability: EndpointMutabilityMetadata::Mutable,
                allow_multiple_var_args: pass_1_data.allow_multiple_var_args,
            });
        })
        .is_some()
}

pub fn process_view_attribute(
    attr: &syn::Attribute,
    pass_1_data: &MethodAttributesPass1,
    method: &mut Method,
) -> bool {
    ViewAttribute::parse(attr)
        .map(|view_attribute| {
            check_single_role(&*method);
            let view_ident = match view_attribute.view_name {
                Some(ident) => ident,
                None => method.name.clone(),
            };
            method.public_role = PublicRole::Endpoint(EndpointMetadata {
                public_name: view_ident,
                payable: pass_1_data.payable.clone(),
                only_owner: pass_1_data.only_owner,
                only_admin: pass_1_data.only_admin,
                only_user_account: pass_1_data.only_user_account,
                mutability: EndpointMutabilityMetadata::Readonly,
                allow_multiple_var_args: pass_1_data.allow_multiple_var_args,
            });
        })
        .is_some()
}

pub fn process_external_view_attribute(
    attr: &syn::Attribute,
    pass_1_data: &MethodAttributesPass1,
    method: &mut Method,
) -> bool {
    ExternalViewAttribute::parse(attr)
        .map(|external_view_attribute| {
            check_single_role(&*method);
            let view_ident = match external_view_attribute.view_name {
                Some(ident) => ident,
                None => method.name.clone(),
            };
            method.public_role = PublicRole::Endpoint(EndpointMetadata {
                public_name: view_ident,
                payable: pass_1_data.payable.clone(),
                only_owner: pass_1_data.only_owner,
                only_admin: pass_1_data.only_admin,
                only_user_account: pass_1_data.only_user_account,
                mutability: EndpointMutabilityMetadata::Readonly,
                allow_multiple_var_args: pass_1_data.allow_multiple_var_args,
            });
        })
        .is_some()
}

pub fn process_callback_raw_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    if is_callback_raw(attr) {
        check_single_role(&*method);
        method.public_role = PublicRole::CallbackRaw;
        true
    } else {
        false
    }
}

pub fn process_callback_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    CallbackAttribute::parse(attr)
        .map(|callback_attr| {
            check_single_role(&*method);
            let callback_ident = match callback_attr.callback_name {
                Some(ident) => ident,
                None => method.name.clone(),
            };
            method.public_role = PublicRole::Callback(CallbackMetadata {
                callback_name: callback_ident,
                allow_multiple_var_args: method.is_allow_multiple_var_args(),
            });
        })
        .is_some()
}

pub fn process_promises_callback_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    PromisesCallbackAttribute::parse(attr)
        .map(|callback_attr| {
            check_single_role(&*method);
            let callback_ident = match callback_attr.callback_name {
                Some(ident) => ident,
                None => method.name.clone(),
            };
            method.public_role = PublicRole::CallbackPromise(CallbackMetadata {
                callback_name: callback_ident,
                allow_multiple_var_args: method.is_allow_multiple_var_args(),
            });
        })
        .is_some()
}

pub fn process_output_names_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    OutputNameAttribute::parse(attr)
        .map(|output_name_attr| {
            method.output_names.push(output_name_attr.output_name);
        })
        .is_some()
}

pub fn process_label_names_attribute(attr: &syn::Attribute, method: &mut Method) -> bool {
    LabelAttribute::parse(attr)
        .map(|label_attr| {
            method.label_names.push(label_attr.label);
        })
        .is_some()
}
