use super::util::*;
use crate::model::{
    ContractTrait, EndpointLocationMetadata, EndpointMutabilityMetadata, Method, PublicRole,
};

fn generate_endpoint_snippet(
    m: &Method,
    endpoint_name: &str,
    only_owner: bool,
    mutability: EndpointMutabilityMetadata,
    location: EndpointLocationMetadata,
) -> proc_macro2::TokenStream {
    let endpoint_docs = &m.docs;
    let payable_in_tokens = m.payable_metadata().abi_strings();

    let input_snippets: Vec<proc_macro2::TokenStream> = m
        .method_args
        .iter()
        .filter_map(|arg| {
            if arg.metadata.payment.is_payment_arg() {
                None
            } else {
                let mut arg_type = arg.ty.clone();
                clear_all_type_lifetimes(&mut arg_type);
                let arg_name = &arg.pat;
                let arg_name_str = quote! { #arg_name }.to_string();
                Some(quote! {
                    endpoint_abi.add_input::<#arg_type>(#arg_name_str);
                    contract_abi.add_type_descriptions::<#arg_type>();
                })
            }
        })
        .collect();

    let output_names = &m.output_names;
    let output_snippet = match &m.return_type {
        syn::ReturnType::Default => quote! {},
        syn::ReturnType::Type(_, ty) => {
            let mut res_type = ty.clone();
            clear_all_type_lifetimes(&mut res_type);
            quote! {
                endpoint_abi.add_output::<#res_type>(&[ #(#output_names),* ]);
                contract_abi.add_type_descriptions::<#res_type>();
            }
        },
    };
    let mutability_tokens = mutability.to_tokens();
    let location_tokens = location.to_tokens();

    quote! {
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi{
            docs: &[ #(#endpoint_docs),* ],
            name: #endpoint_name,
            only_owner: #only_owner,
            mutability: #mutability_tokens,
            location: #location_tokens,
            payable_in_tokens: &[ #(#payable_in_tokens),* ],
            inputs: elrond_wasm::types::heap::Vec::new(),
            outputs: elrond_wasm::types::heap::Vec::new(),
        };
        #(#input_snippets)*
        #output_snippet
    }
}

fn generate_endpoint_snippets(contract: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract
        .methods
        .iter()
        .filter_map(|m| match &m.public_role {
            PublicRole::Init(_) => {
                let endpoint_def = generate_endpoint_snippet(
                    m,
                    "init",
                    false,
                    EndpointMutabilityMetadata::Mutable,
                    EndpointLocationMetadata::MainContract,
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.constructors.push(endpoint_abi);
                })
            },
            PublicRole::Endpoint(endpoint_metadata) => {
                let endpoint_name_str = endpoint_metadata.public_name.to_string();
                let endpoint_def = generate_endpoint_snippet(
                    m,
                    &endpoint_name_str,
                    endpoint_metadata.only_owner,
                    endpoint_metadata.mutability.clone(),
                    endpoint_metadata.location.clone(),
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.endpoints.push(endpoint_abi);
                })
            },
            _ => None,
        })
        .collect()
}

fn generate_event_snippet(m: &Method, event_name: &str) -> proc_macro2::TokenStream {
    let event_docs = &m.docs;
    let input_snippets: Vec<proc_macro2::TokenStream> = m
        .method_args
        .iter()
        .filter_map(|arg| {
            let mut arg_type = arg.ty.clone();
            let indexed = arg.metadata.event_topic;
            clear_all_type_lifetimes(&mut arg_type);
            let arg_name = &arg.pat;
            let arg_name_str = quote! { #arg_name }.to_string();
            Some(quote! {
                event_abi.add_input::<#arg_type>(#arg_name_str, #indexed);
                contract_abi.add_type_descriptions::<#arg_type>();
            })
        })
        .collect();

    quote! {
        let mut event_abi = elrond_wasm::abi::EventAbi{
            docs: &[ #(#event_docs),* ],
            identifier: #event_name,
            inputs: elrond_wasm::types::heap::Vec::new(),
        };
        #(#input_snippets)*
    }
}

fn generate_event_snippets(contract: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract
        .methods
        .iter()
        .filter_map(|m| match &m.public_role {
            PublicRole::Event(event_metadata) => {
                let event_name_str = &event_metadata.event_identifier;
                let event_def = generate_event_snippet(m, &event_name_str);
                Some(quote! {
                    #event_def
                    contract_abi.events.push(event_abi);
                })
            },
            _ => None,
        })
        .collect()
}

fn has_callback(contract: &ContractTrait) -> bool {
    contract.methods.iter().any(|m| {
        matches!(
            m.public_role,
            PublicRole::Callback(_) | PublicRole::CallbackRaw
        )
    })
}

fn generate_supertrait_snippets(contract: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract
			.supertraits
			.iter()
			.map(|supertrait| {
				let module_path = &supertrait.module_path;
				quote! {
					contract_abi.coalesce(<#module_path AbiProvider as elrond_wasm::contract_base::ContractAbiProvider>::abi());
				}
			})
			.collect()
}

fn generate_abi_method_body(
    contract: &ContractTrait,
    is_contract_main: bool,
) -> proc_macro2::TokenStream {
    let contract_docs = &contract.docs;
    let contract_name = &contract.trait_name.to_string();
    let endpoint_snippets = generate_endpoint_snippets(contract);
    let event_snippets = generate_event_snippets(contract);
    let has_callbacks = has_callback(contract);
    let supertrait_snippets: Vec<proc_macro2::TokenStream> = if is_contract_main {
        generate_supertrait_snippets(contract)
    } else {
        Vec::new()
    };

    quote! {
        let mut contract_abi = elrond_wasm::abi::ContractAbi {
            build_info: elrond_wasm::abi::BuildInfoAbi {
                contract_crate: elrond_wasm::abi::ContractCrateBuildAbi {
                    name: env!("CARGO_PKG_NAME"),
                    version: env!("CARGO_PKG_VERSION"),
                    git_version: elrond_wasm::abi::git_version!(fallback = ""),
                },
                framework: elrond_wasm::abi::FrameworkBuildAbi::create(),
            },
            docs: &[ #(#contract_docs),* ],
            name: #contract_name,
            constructors: elrond_wasm::types::heap::Vec::new(),
            endpoints: elrond_wasm::types::heap::Vec::new(),
            events: elrond_wasm::types::heap::Vec::new(),
            has_callback: #has_callbacks,
            type_descriptions: <elrond_wasm::abi::TypeDescriptionContainerImpl as elrond_wasm::abi::TypeDescriptionContainer>::new(),
        };
        #(#endpoint_snippets)*
        #(#event_snippets)*
        #(#supertrait_snippets)*
        contract_abi
    }
}

pub fn generate_abi_provider(
    contract: &ContractTrait,
    is_contract_main: bool,
) -> proc_macro2::TokenStream {
    let abi_body = generate_abi_method_body(contract, is_contract_main);
    quote! {
        pub struct AbiProvider {}

        impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
            type Api = elrond_wasm::api::uncallable::UncallableApi;

            fn abi() -> elrond_wasm::abi::ContractAbi {
                #abi_body
            }
        }
    }
}
