use super::{
    convert_to_owned_type::*, method_gen::generate_arg_call_name, only_owner_gen::*,
    payable_gen::*, util::*,
};
use crate::{
    generate::snippets,
    model::{Method, MethodArgument},
};

pub fn generate_call_to_method_expr(m: &Method) -> proc_macro2::TokenStream {
    let fn_ident = &m.name;
    let arg_values: Vec<proc_macro2::TokenStream> =
        m.method_args.iter().map(generate_arg_call_name).collect();
    quote! {
        self.#fn_ident (#(#arg_values),*)
    }
}

pub fn generate_call_method(m: &Method) -> proc_macro2::TokenStream {
    let call_method_ident = generate_call_method_name(&m.name);
    let call_method_body = generate_endpoint_call_method_body(m);
    quote! {
        #[inline]
        fn #call_method_ident (&self) {
            #call_method_body
        }
    }
}

pub fn generate_promises_callback_call_method(m: &Method) -> proc_macro2::TokenStream {
    let call_method_ident = generate_call_method_name(&m.name);
    let call_method_body = generate_promises_callback_call_method_body(m);
    quote! {
        #[inline]
        fn #call_method_ident (&self) {
            #call_method_body
        }
    }
}

/// Generates an expression of the form `(a1, (a2, ... (an, ())))`,
/// where `a1 ... an` are the results of applying `arg_mapping` on each argument.
pub fn generate_arg_nested_tuple<ArgFilter, ArgMapping>(
    method_args: &[MethodArgument],
    arg_filter: ArgFilter,
    arg_mapping: ArgMapping,
) -> proc_macro2::TokenStream
where
    ArgFilter: Fn(&MethodArgument) -> bool,
    ArgMapping: Fn(&MethodArgument) -> proc_macro2::TokenStream,
{
    method_args
        .iter()
        .rev()
        .filter(|arg| arg_filter(arg))
        .fold(quote! {()}, |ts, arg| {
            let arg_tokens = arg_mapping(arg);
            quote! { ( #arg_tokens, #ts ) }
        })
}

/// In one go it generates the var names, var types, and var_names as string, all as nested tuples.
pub fn generate_arg_nested_tuples<ArgFilter>(
    method_args: &[MethodArgument],
    arg_filter: ArgFilter,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
)
where
    ArgFilter: Fn(&MethodArgument) -> bool,
{
    let var_names = generate_arg_nested_tuple(
        method_args,
        |arg| arg_filter(arg),
        |arg| quote::ToTokens::to_token_stream(&arg.pat),
    );
    let var_types = generate_arg_nested_tuple(
        method_args,
        |arg| arg_filter(arg),
        |arg| convert_to_owned_type(&arg.ty),
    );
    let var_names_str = generate_arg_nested_tuple(
        method_args,
        |arg| arg_filter(arg),
        |arg| quote::ToTokens::to_token_stream(&pat_string(&arg.pat)),
    );
    (var_names, var_types, var_names_str)
}

pub fn generate_call_method_arg_load(m: &Method) -> proc_macro2::TokenStream {
    let (var_names, var_types, var_names_str) =
        generate_arg_nested_tuples(m.method_args.as_slice(), |arg| arg.is_endpoint_arg());
    quote! {
        let #var_names = elrond_wasm::io::load_endpoint_args::<Self::Api, #var_types>(#var_names_str);
    }
}

pub fn generate_body_with_result(
    return_type: &syn::ReturnType,
    mbody: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match return_type {
        syn::ReturnType::Default => quote! {
            #mbody;
        },
        syn::ReturnType::Type(_, _) => {
            quote! {
                let result = #mbody;
                elrond_wasm::io::finish_multi::<Self::Api, _>(&result);
            }
        },
    }
}


pub fn generate_endpoint_call_method_body(m: &Method) -> proc_macro2::TokenStream {
    let api_static_init = snippets::call_method_api_static_init();
    let payable_snippet = generate_payable_snippet(m);
    let only_owner_snippet = generate_only_owner_snippet(m);
    let arg_load = generate_call_method_arg_load(m);

    let call = generate_call_to_method_expr(m);
    let body_with_result = generate_body_with_result(&m.return_type, &call);

    quote! {
        #api_static_init
        #payable_snippet
        #only_owner_snippet
        #arg_load
        #body_with_result
    }
}


pub fn generate_promises_callback_call_method_body(m: &Method) -> proc_macro2::TokenStream {
    let api_static_init = snippets::call_method_api_static_init();
    let payable_snippet = generate_payable_snippet(m);
    let arg_load = generate_call_method_arg_load(m);

    let call = generate_call_to_method_expr(m);
    let body_with_result = generate_body_with_result(&m.return_type, &call);

    quote! {
        #api_static_init
        #payable_snippet
        #arg_load
        #body_with_result
    }
}
