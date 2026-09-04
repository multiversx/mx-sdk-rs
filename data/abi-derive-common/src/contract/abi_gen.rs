use crate::TypeAbiImportCrate;
use crate::contract::model::{
    AutoImpl, ContractTrait, EndpointMutabilityMetadata, EndpointTypeMetadata, Method, MethodImpl,
    PublicRole,
};
use crate::contract::util::clear_all_type_lifetimes;
use crate::type_abi_derive::import_tokens;

fn generate_endpoint_snippet(
    m: &Method,
    endpoint_name: &str,
    only_owner: bool,
    only_admin: bool,
    mutability: EndpointMutabilityMetadata,
    endpoint_type: EndpointTypeMetadata,
    allow_multiple_var_args: bool,
    import: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let endpoint_docs = &m.docs;
    let rust_method_name = m.name.to_string();
    let title_tokens = m
        .title
        .as_ref()
        .map(|title| quote! { .with_title(#title) })
        .unwrap_or_default();
    let only_owner_tokens = if only_owner {
        quote! { .with_only_owner() }
    } else {
        quote! {}
    };
    let only_admin_tokens = if only_admin {
        quote! { .with_only_admin() }
    } else {
        quote! {}
    };
    let allow_multiple_var_args_tokens = if allow_multiple_var_args {
        quote! { .with_allow_multiple_var_args() }
    } else {
        quote! {}
    };

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
        }
    };

    let label_names = &m.label_names;
    let mutability_tokens = mutability.to_tokens(import);
    let endpoint_type_tokens = endpoint_type.to_tokens(import);

    quote! {
        let mut endpoint_abi = #import::EndpointAbi::new(
            #endpoint_name,
            #rust_method_name,
            #mutability_tokens,
            #endpoint_type_tokens,
        )
        #(.with_docs(#endpoint_docs))*
        #title_tokens
        #only_owner_tokens
        #only_admin_tokens
        #(.with_label(#label_names))*
        #(.with_payable_token(#payable_in_tokens))*
        #allow_multiple_var_args_tokens
        ;

        #(#input_snippets)*
        #output_snippet
    }
}

fn generate_endpoint_snippets(
    contract: &ContractTrait,
    import: &proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
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
                    m.is_allow_multiple_var_args(),
                    import,
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.constructors.push(endpoint_abi);
                })
            }
            PublicRole::Upgrade(_) => {
                let endpoint_def = generate_endpoint_snippet(
                    m,
                    "upgrade",
                    false,
                    false,
                    EndpointMutabilityMetadata::Mutable,
                    EndpointTypeMetadata::Upgrade,
                    m.is_allow_multiple_var_args(),
                    import,
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.upgrade_constructors.push(endpoint_abi);
                })
            }
            PublicRole::Endpoint(endpoint_metadata) => {
                let endpoint_def = generate_endpoint_snippet(
                    m,
                    &endpoint_metadata.public_name.to_string(),
                    endpoint_metadata.only_owner,
                    endpoint_metadata.only_admin,
                    endpoint_metadata.mutability.clone(),
                    EndpointTypeMetadata::Endpoint,
                    endpoint_metadata.allow_multiple_var_args,
                    import,
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.endpoints.push(endpoint_abi);
                })
            }
            PublicRole::CallbackPromise(callback_metadata) => {
                let endpoint_def = generate_endpoint_snippet(
                    m,
                    &callback_metadata.callback_name.to_string(),
                    false,
                    false,
                    EndpointMutabilityMetadata::Mutable,
                    EndpointTypeMetadata::PromisesCallback,
                    m.is_allow_multiple_var_args(),
                    import,
                );
                Some(quote! {
                    #endpoint_def
                    contract_abi.promise_callbacks.push(endpoint_abi);
                })
            }
            _ => None,
        })
        .collect()
}

fn generate_event_snippet(
    m: &Method,
    event_name: &str,
    import: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
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
        let mut event_abi = #import::EventAbi::new(
            &[ #(#event_docs),* ],
            #event_name,
        );
        #(#input_snippets)*
    }
}

fn generate_event_snippets(
    contract: &ContractTrait,
    import: &proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    contract
        .methods
        .iter()
        .filter_map(|m| {
            if let MethodImpl::Generated(AutoImpl::Event { identifier }) = &m.implementation {
                let event_def = generate_event_snippet(m, identifier, import);
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

fn generate_supertrait_snippets(
    contract: &ContractTrait,
    provider_trait_path: &proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    contract
        .supertraits
        .iter()
        .map(|supertrait| {
            let module_path = &supertrait.module_path;
            quote! {
                contract_abi.coalesce(<#module_path AbiProvider as #provider_trait_path>::abi());
            }
        })
        .collect()
}

fn generate_esdt_attribute_snippets(
    contract: &ContractTrait,
    import: &proc_macro2::TokenStream,
) -> Vec<proc_macro2::TokenStream> {
    contract
        .trait_attributes
        .esdt_attribute
        .iter()
        .map(|esdt_attr| {
            let ticker = &esdt_attr.ticker;
            let ty = &esdt_attr.ty;
            quote! {
                contract_abi.esdt_attributes.push(#import::EsdtAttributeAbi::new::<#ty>(#ticker));
                contract_abi.add_type_descriptions::<#ty>();
            }
        })
        .collect()
}

fn generate_abi_method_body(
    contract: &ContractTrait,
    is_contract_main: bool,
    import_crate: TypeAbiImportCrate,
    provider_trait_path: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let import = import_tokens(import_crate);
    let contract_docs = &contract.docs;
    let contract_name = &contract.trait_name.to_string();
    let endpoint_snippets = generate_endpoint_snippets(contract, &import);
    let event_snippets = generate_event_snippets(contract, &import);
    let has_callbacks = has_callback(contract);
    let supertrait_snippets: Vec<proc_macro2::TokenStream> = if is_contract_main {
        generate_supertrait_snippets(contract, provider_trait_path)
    } else {
        Vec::new()
    };
    let esdt_attributes = if !&contract.trait_attributes.esdt_attribute.is_empty() {
        generate_esdt_attribute_snippets(contract, &import)
    } else {
        Vec::new()
    };

    let framework_build_info = match import_crate {
        TypeAbiImportCrate::MultiversxSc => quote! { multiversx_sc::framework_build_abi() },
        TypeAbiImportCrate::MultiversxScAbi => quote! { #import::FrameworkBuildAbi::default() },
    };

    quote! {
        let mut contract_abi = #import::ContractAbi::new(
            #import::BuildInfoAbi {
                rustc: None,
                contract_crate: #import::ContractCrateBuildAbi::new(
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                ),
                abi: Some(#import::FrameworkBuildAbi::abi_crate()),
                framework: #framework_build_info,
            },
            &[ #(#contract_docs),* ],
            #contract_name,
            #has_callbacks,
        );
        #(#endpoint_snippets)*
        #(#event_snippets)*
        #(#supertrait_snippets)*
        #(#esdt_attributes)*
        contract_abi
    }
}

/// Generates the `AbiProvider` struct and a single impl block providing its `fn abi()`.
///
/// `provider_trait_path` is the full path of the `ContractAbiProvider`-shaped trait to
/// implement (e.g. `multiversx_sc::contract_base::ContractAbiProvider` or
/// `multiversx_sc_abi::ContractAbiProvider`), and `extra_impl_items` are spliced into the
/// *same* impl block (e.g. `type Api = ...;`). Everything must live in one impl block:
/// the generated `fn abi()` body references bare `Self::Api` wherever a managed type such
/// as `BigUint<Self::Api>` shows up in an endpoint signature, and bare `Self::X` only
/// resolves against the trait currently being implemented.
pub fn generate_abi_provider(
    contract: &ContractTrait,
    is_contract_main: bool,
    import_crate: TypeAbiImportCrate,
    provider_trait_path: &proc_macro2::TokenStream,
    extra_impl_items: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let import = import_tokens(import_crate);
    let abi_body =
        generate_abi_method_body(contract, is_contract_main, import_crate, provider_trait_path);
    quote! {
        pub struct AbiProvider {}

        impl #provider_trait_path for AbiProvider {
            #extra_impl_items

            fn abi() -> #import::ContractAbi {
                #abi_body
            }
        }
    }
}
