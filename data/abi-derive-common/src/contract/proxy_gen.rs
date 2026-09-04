use quote::format_ident;

use crate::TypeAbiImportCrate;
use crate::contract::model::{ContractTrait, Method, PublicRole};
use crate::contract::util::clear_all_type_lifetimes;
use crate::type_abi_derive::import_tokens;

/// The three proxy-generating roles: a constructor (`IntoDeploy`), the upgrade constructor
/// (`IntoUpgrade`), or a regular endpoint call (`IntoCall`), identified by its wire name.
/// Every other `PublicRole` (callbacks, private methods) has no external call surface and is
/// skipped.
enum ProxyMethodKind {
    Deploy,
    Upgrade,
    Call(String),
}

/// Builds one dedicated `impl<T, ..> #methods_name<T> where T: IntoXxx<Payment, Output> { .. }`
/// block for a single method, mirroring `contracts/examples/adder/src/adder_abi.rs`'s
/// hand-written pattern: every method gets its own impl block, since `Output` (and, for payable
/// methods, `Payment`) differs per method.
fn generate_proxy_method(
    m: &Method,
    kind: &ProxyMethodKind,
    methods_name: &syn::Ident,
    import: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let rust_name = &m.name;

    let args: Vec<_> = m
        .method_args
        .iter()
        .filter(|arg| arg.is_endpoint_arg())
        .collect();
    let arg_generics: Vec<syn::Ident> = (0..args.len())
        .map(|i| format_ident!("Arg{}", i))
        .collect();
    let arg_types: Vec<syn::Type> = args
        .iter()
        .map(|arg| {
            let mut ty = arg.ty.clone();
            clear_all_type_lifetimes(&mut ty);
            ty
        })
        .collect();
    let arg_pats: Vec<&syn::Pat> = args.iter().map(|arg| &arg.pat).collect();

    let output_ty = match &m.return_type {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => {
            let mut ty = (**ty).clone();
            clear_all_type_lifetimes(&mut ty);
            quote! { #ty }
        }
    };

    // A `NotPayable` endpoint hardcodes the marker, exactly like `adder_abi.rs`. A payable
    // endpoint (e.g. kitty's `#[payable("EGLD")] breedWith`) instead takes a fresh `Payment`
    // generic as its first parameter, leaving the caller free to supply whatever payment type
    // the underlying `T` accepts (the framework binding in `tx_proxy_abi_impl.rs` is what
    // actually constrains it, e.g. to EGLD-only for deploys/upgrades).
    let payable = m.payable_metadata().is_payable();
    let payment_ty: proc_macro2::TokenStream = if payable {
        quote! { Payment }
    } else {
        quote! { #import::NotPayable }
    };
    let extra_generic: Option<proc_macro2::TokenStream> = payable.then(|| quote! { Payment, });
    let payment_param: Option<proc_macro2::TokenStream> =
        payable.then(|| quote! { payment: Payment, });
    let payment_arg: proc_macro2::TokenStream = if payable {
        quote! { payment }
    } else {
        quote! { #import::NotPayable }
    };

    let (into_trait, into_method, call_args): (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
    ) = match kind {
        ProxyMethodKind::Deploy => (
            quote! { #import::IntoDeploy<#payment_ty, #output_ty> },
            quote! { into_deploy },
            quote! { #payment_arg },
        ),
        ProxyMethodKind::Upgrade => (
            quote! { #import::IntoUpgrade<#payment_ty, #output_ty> },
            quote! { into_upgrade },
            quote! { #payment_arg },
        ),
        ProxyMethodKind::Call(wire_name) => (
            quote! { #import::IntoCall<#payment_ty, #output_ty> },
            quote! { into_call },
            quote! { #payment_arg, #wire_name },
        ),
    };

    // `Payment` can't be an impl-level generic: it wouldn't appear in the impl's self type
    // (`#methods_name<T>`), only in the where clause and method body, which rustc rejects as an
    // unconstrained type parameter (E0207). It's a method-level generic instead, alongside the
    // `ArgN` ones, with the `T: IntoXxx<Payment, Output>` bound moved onto the method itself.
    quote! {
        impl<T> #methods_name<T> {
            pub fn #rust_name<#extra_generic #(#arg_generics: #import::ProxyArg<#arg_types>),*>(
                self,
                #payment_param
                #(#arg_pats: #arg_generics),*
            ) -> <T as #into_trait>::Out
            where
                T: #into_trait,
            {
                self.base_tx
                    .#into_method(#call_args)
                    #(.apply_argument(&#arg_pats))*
            }
        }
    }
}

/// Generates a proxy pair (`#proxy_name` / `#proxy_name`+`Methods`) for the given contract,
/// following the `AbiProxyTrait<T>`/`IntoCall`/`IntoDeploy`/`IntoUpgrade` pattern from
/// `multiversx_sc_abi::proxy_abi_traits`, matching `adder_abi.rs`'s hand-written shape.
///
/// `proxy_name` is caller-supplied rather than derived from the trait name, so there is no
/// magic naming: it is the exact struct name that ends up callable at the use site (e.g. via
/// `.abi_typed(proxy_name)`). The methods struct is always `proxy_name` suffixed with
/// `Methods`, matching every other proxy-methods struct in the codebase.
///
/// Only `Init`, `Upgrade` and `Endpoint` methods have an external call surface; everything else
/// (callbacks, private methods) is skipped, same as `abi_gen`'s ABI-only counterpart.
pub fn generate_abi_proxy(
    contract: &ContractTrait,
    import_crate: TypeAbiImportCrate,
    proxy_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let import = import_tokens(import_crate);
    let methods_name = format_ident!("{}Methods", proxy_name);

    let method_impls: Vec<proc_macro2::TokenStream> = contract
        .methods
        .iter()
        .filter_map(|m| {
            let kind = match &m.public_role {
                PublicRole::Init(_) => ProxyMethodKind::Deploy,
                PublicRole::Upgrade(_) => ProxyMethodKind::Upgrade,
                PublicRole::Endpoint(endpoint_metadata) => {
                    ProxyMethodKind::Call(endpoint_metadata.public_name.to_string())
                }
                _ => return None,
            };
            Some(generate_proxy_method(m, &kind, &methods_name, &import))
        })
        .collect();

    quote! {
        use #import::ApplyArgument as _;

        pub struct #proxy_name;

        impl<T> #import::AbiProxyTrait<T> for #proxy_name {
            type Methods = #methods_name<T>;

            fn proxy_methods(self, base_tx: T) -> Self::Methods {
                #methods_name { base_tx }
            }
        }

        pub struct #methods_name<T> {
            base_tx: T,
        }

        #(#method_impls)*
    }
}
