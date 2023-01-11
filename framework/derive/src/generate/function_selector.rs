use super::{supertrait_gen, util::*};
use crate::model::{ContractTrait, Method, PublicRole};

fn endpoint_match_arm(
    m: &Method,
    endpoint_name: &str,
    match_guard: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let fn_ident = &m.name;
    let call_method_ident = generate_call_method_name(fn_ident);
    quote! {
        #endpoint_name #match_guard =>
        {
            self.#call_method_ident();
            true
        },
    }
}

pub fn generate_function_selector_body(contract: &ContractTrait) -> proc_macro2::TokenStream {
    let match_arms: Vec<proc_macro2::TokenStream> = contract
        .methods
        .iter()
        .filter_map(|m| match &m.public_role {
            PublicRole::Init(_) => Some(endpoint_match_arm(
                m,
                "init",
                quote! { if !<Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() },
            )),
            PublicRole::Endpoint(endpoint_metadata) => Some(endpoint_match_arm(
                m,
                endpoint_metadata.public_name.to_string().as_str(),
                quote! {},
            )),
            PublicRole::CallbackPromise(callback_metadata) => Some(endpoint_match_arm(
                m,
                callback_metadata.callback_name.to_string().as_str(),
                quote! {},
            )),
            _ => None,
        })
        .collect();
    let module_calls =
        supertrait_gen::function_selector_module_calls(contract.supertraits.as_slice());
    quote! {
        if match fn_name {
            "callBack" => {
                self::EndpointWrappers::callback(self);
                return true;
            },
            "init" if <Self::Api as multiversx_sc::api::VMApi>::external_view_init_override() => {
                multiversx_sc::external_view_contract::external_view_contract_constructor::<Self::Api>();
                return true;
            },
            #(#match_arms)*
            other => false
        } {
            return true;
        }
        #(#module_calls)*
        false
    }
}
