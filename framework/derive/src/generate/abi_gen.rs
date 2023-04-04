use super::util::*;
use crate::model::{
    AutoImpl, ContractTrait, EndpointMutabilityMetadata, EndpointTypeMetadata, Method, MethodImpl,
    PublicRole,
};

fn generate_endpoint_snippet(
    m: &Method,
    endpoint_name: &str,
    only_owner: bool,
    only_admin: bool,
    mutability: EndpointMutabilityMetadata,
    endpoint_type: EndpointTypeMetadata,
) -> proc_macro2::TokenStream {
    let endpoint_docs = &m.docs;
    let rust_method_name = m.name.to_string();
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

    let label_names = &m.label_names;
    let mutability_tokens = mutability.to_tokens();
    let endpoint_type_tokens = endpoint_type.to_tokens();

    quote! {
        let mut endpoint_abi = multiversx_sc::abi::EndpointAbi{
            docs: &[ #(#endpoint_docs),* ],
            name: #endpoint_name,
            rust_method_name: #rust_method_name,
            only_owner: #only_owner,
            only_admin: #only_admin,
            mutability: #mutability_tokens,
            endpoint_type: #endpoint_type_tokens,
            payable_in_tokens: &[ #(#payable_in_tokens),* ],
            inputs: multiversx_sc::types::heap::Vec::new(),
            outputs: multiversx_sc::types::heap::Vec::new(),
            labels: &[ #(#label_names),* ],
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
                    false,
                    EndpointMutabilityMetadata::Mutable,
                    EndpointTypeMetadata::Init,
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.constructors.push(endpoint_abi);
                })
            },
            PublicRole::Endpoint(endpoint_metadata) => {
                let endpoint_def = generate_endpoint_snippet(
                    m,
                    &endpoint_metadata.public_name.to_string(),
                    endpoint_metadata.only_owner,
                    endpoint_metadata.only_admin,
                    endpoint_metadata.mutability.clone(),
                    EndpointTypeMetadata::Endpoint,
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.endpoints.push(endpoint_abi);
                })
            },
            PublicRole::CallbackPromise(callback_metadata) => {
                let endpoint_def = generate_endpoint_snippet(
                    m,
                    &callback_metadata.callback_name.to_string(),
                    false,
                    false,
                    EndpointMutabilityMetadata::Mutable,
                    EndpointTypeMetadata::PromisesCallback,
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.promise_callbacks.push(endpoint_abi);
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
        .map(|arg| {
            let mut arg_type = arg.ty.clone();
            let indexed = arg.metadata.event_topic;
            clear_all_type_lifetimes(&mut arg_type);
            let arg_name = &arg.pat;
            let arg_name_str = quote! { #arg_name }.to_string();
            quote! {
                event_abi.add_input::<#arg_type>(#arg_name_str, #indexed);
                contract_abi.add_type_descriptions::<#arg_type>();
            }
        })
        .collect();

    quote! {
        let mut event_abi = multiversx_sc::abi::EventAbi{
            docs: &[ #(#event_docs),* ],
            identifier: #event_name,
            inputs: multiversx_sc::types::heap::Vec::new(),
        };
        #(#input_snippets)*
    }
}

fn generate_event_snippets(contract: &ContractTrait) -> Vec<proc_macro2::TokenStream> {
    contract
        .methods
        .iter()
        .filter_map(|m| {
            if let MethodImpl::Generated(AutoImpl::Event { identifier }) = &m.implementation {
                let event_def = generate_event_snippet(m, identifier);
                Some(quote! {
                    #event_def
                    contract_abi.events.push(event_abi);
                })
            } else {
                None
            }
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
					contract_abi.coalesce(<#module_path AbiProvider as multiversx_sc::contract_base::ContractAbiProvider>::abi());
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
        let mut contract_abi = multiversx_sc::abi::ContractAbi {
            build_info: multiversx_sc::abi::BuildInfoAbi {
                contract_crate: multiversx_sc::abi::ContractCrateBuildAbi {
                    name: env!("CARGO_PKG_NAME"),
                    version: env!("CARGO_PKG_VERSION"),
                    git_version: "",
                },
                framework: multiversx_sc::abi::FrameworkBuildAbi::create(),
            },
            docs: &[ #(#contract_docs),* ],
            name: #contract_name,
            constructors: multiversx_sc::types::heap::Vec::new(),
            endpoints: multiversx_sc::types::heap::Vec::new(),
            promise_callbacks: multiversx_sc::types::heap::Vec::new(),
            events: multiversx_sc::types::heap::Vec::new(),
            has_callback: #has_callbacks,
            type_descriptions: <multiversx_sc::abi::TypeDescriptionContainerImpl as multiversx_sc::abi::TypeDescriptionContainer>::new(),
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

        impl multiversx_sc::contract_base::ContractAbiProvider for AbiProvider {
            type Api = multiversx_sc::api::uncallable::UncallableApi;

            fn abi() -> multiversx_sc::abi::ContractAbi {
                #abi_body
            }
        }
    }
}
