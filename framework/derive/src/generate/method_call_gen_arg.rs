use super::{convert_to_owned_type::*, util::*};
use crate::model::{Method, MethodArgument};

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
        let #var_names = multiversx_sc::io::load_endpoint_args::<Self::Api, #var_types>(#var_names_str);
    }
}

pub fn load_call_result_args_snippet(m: &Method) -> proc_macro2::TokenStream {
    // if no `#[call_result]` present, ignore altogether
    let has_call_result = m
        .method_args
        .iter()
        .any(|arg| arg.metadata.callback_call_result);
    if !has_call_result {
        return quote! {};
    }

    let (call_result_var_names, call_result_var_types, call_result_var_names_str) =
        generate_arg_nested_tuples(m.method_args.as_slice(), |arg| {
            arg.metadata.callback_call_result
        });
    quote! {
        let #call_result_var_names = multiversx_sc::io::load_endpoint_args::<Self::Api,#call_result_var_types>(
            #call_result_var_names_str,
        );
    }
}

pub fn load_legacy_cb_closure_args_snippet(m: &Method) -> proc_macro2::TokenStream {
    let (closure_var_names, closure_var_types, closure_var_names_str) =
        generate_arg_nested_tuples(m.method_args.as_slice(), |arg| {
            arg.is_endpoint_arg() && !arg.metadata.callback_call_result
        });
    quote! {
        let #closure_var_names = multiversx_sc::io::load_multi_args_custom_loader::<Self::Api, _, #closure_var_types>(
            ___cb_closure___.into_arg_loader(),
            #closure_var_names_str,
        );
    }
}

pub fn load_cb_closure_args_snippet(m: &Method) -> proc_macro2::TokenStream {
    let (closure_var_names, closure_var_types, closure_var_names_str) =
        generate_arg_nested_tuples(m.method_args.as_slice(), |arg| {
            arg.is_endpoint_arg() && !arg.metadata.callback_call_result
        });
    quote! {
        let #closure_var_names = multiversx_sc::io::load_callback_closure_args::<
            Self::Api,
            #closure_var_types,
        >(#closure_var_names_str);
    }
}
