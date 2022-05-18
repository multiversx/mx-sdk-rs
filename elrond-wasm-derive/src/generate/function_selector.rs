use super::{supertrait_gen, util::*};
use crate::model::{ContractTrait, EndpointLocationMetadata, Method, PublicRole};

fn endpoint_match_arm(
    m: &Method,
    endpoint_name: &str,
    location: &EndpointLocationMetadata,
) -> proc_macro2::TokenStream {
    let fn_ident = &m.name;
    let call_method_ident = generate_call_method_name(fn_ident);
    let endpoint_name_str = array_literal(endpoint_name.to_string().as_bytes());
    let location_tokens = location.to_tokens();
    quote! {
        #endpoint_name_str if <Self::Api as elrond_wasm::api::VMApi>::has_location(#location_tokens) =>
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
                &EndpointLocationMetadata::MainContract,
            )),
            PublicRole::Endpoint(endpoint_metadata) => Some(endpoint_match_arm(
                m,
                endpoint_metadata.public_name.to_string().as_str(),
                &endpoint_metadata.location,
            )),
            PublicRole::CallbackPromise(callback_metadata) => Some(endpoint_match_arm(
                m,
                callback_metadata.callback_name.to_string().as_str(),
                &EndpointLocationMetadata::MainContract,
            )),
            _ => None,
        })
        .collect();
    let module_calls =
        supertrait_gen::function_selector_module_calls(contract.supertraits.as_slice());
    quote! {
        if match fn_name {
            b"callBack" if <Self::Api as elrond_wasm::api::VMApi>::has_location(elrond_wasm::abi::EndpointLocationAbi::MainContract) => {
                self::EndpointWrappers::callback(self);
                return true;
            },
            b"init" if <Self::Api as elrond_wasm::api::VMApi>::has_location(elrond_wasm::abi::EndpointLocationAbi::ViewContract) => {
                elrond_wasm::external_view_contract::external_view_contract_constructor::<Self::Api>();
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
