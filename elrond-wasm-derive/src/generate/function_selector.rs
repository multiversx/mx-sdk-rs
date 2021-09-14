use super::{supertrait_gen, util::*};
use crate::model::{ContractTrait, Method, PublicRole};

fn function_selector_match_arm(m: &Method, endpoint_name: &str) -> proc_macro2::TokenStream {
    let fn_ident = &m.name;
    let call_method_ident = generate_call_method_name(fn_ident);
    let endpoint_name_str = array_literal(endpoint_name.to_string().as_bytes());
    quote! {
        #endpoint_name_str =>
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
            PublicRole::Init(_) => Some(function_selector_match_arm(m, "init")),
            PublicRole::Endpoint(endpoint_metadata) => Some(function_selector_match_arm(
                m,
                endpoint_metadata.public_name.to_string().as_str(),
            )),
            _ => None,
        })
        .collect();
    let module_calls =
        supertrait_gen::function_selector_module_calls(contract.supertraits.as_slice());
    quote! {
        if match fn_name {
            b"callBack" => {
                self::EndpointWrappers::callback(self);
                return true;
            }
            #(#match_arms)*
            other => false
        } {
            return true;
        }
        #(#module_calls)*
        false
    }
}
