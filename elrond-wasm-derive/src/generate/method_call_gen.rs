use super::{
    arg_regular::*, method_gen::generate_arg_call_name, only_owner_gen::*, payable_gen::*, util::*,
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
    let call_method_body = generate_call_method_body(m);
    quote! {
        #[inline]
        fn #call_method_ident (&self) {
            #call_method_body
        }
    }
}

/// Generates an expression of the form `(a1, (a2, ... (an, ())))`, where `a1 ... an` are the results of applying `arg_mapping` on each argument.
pub fn generate_arg_nested_tuple<F>(
    method_args: &[MethodArgument],
    arg_mapping: F,
) -> proc_macro2::TokenStream
where
    F: Fn(&MethodArgument) -> proc_macro2::TokenStream,
{
    method_args
        .iter()
        .rev()
        .filter(|arg| arg.is_endpoint_arg())
        .fold(quote! {()}, |ts, arg| {
            let arg_tokens = arg_mapping(arg);
            quote! { ( #arg_tokens, #ts ) }
        })
}

pub fn generate_call_method_arg_load(m: &Method) -> proc_macro2::TokenStream {
    let pat_nt = generate_arg_nested_tuple(m.method_args.as_slice(), |arg| {
        quote::ToTokens::to_token_stream(&arg.pat)
    });
    let pat_types = generate_arg_nested_tuple(m.method_args.as_slice(), |arg| {
        convert_to_owned_type(&arg.ty)
    });
    let pat_names = generate_arg_nested_tuple(m.method_args.as_slice(), |arg| {
        quote::ToTokens::to_token_stream(&pat_string(&arg.pat))
    });
    quote! {
        let #pat_nt = elrond_wasm::io::load_endpoint_args::<Self::Api, #pat_types>(#pat_names);
    }
}

pub fn generate_call_method_body(m: &Method) -> proc_macro2::TokenStream {
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

// pub fn generate_call_method_body_fixed_args(m: &Method) -> proc_macro2::TokenStream {
//     let api_static_init = snippets::call_method_api_static_init();
//     let payable_snippet = generate_payable_snippet(m);
//     let only_owner_snippet = generate_only_owner_snippet(m);

//     let mut arg_index = -1i32;
//     let arg_init_snippets: Vec<proc_macro2::TokenStream> = m
//         .method_args
//         .iter()
//         .map(|arg| {
//             assert!(
//                 !arg.metadata.var_args,
//                 "var_args not accepted in function generate_call_method_fixed_args"
//             );

//             if arg.is_endpoint_arg() {
//                 arg_index += 1;
//                 let pat = &arg.pat;
//                 let arg_get = generate_load_single_arg(arg, &quote! { #arg_index });
//                 quote! {
//                     let #pat = #arg_get;
//                 }
//             } else {
//                 quote! {}
//             }
//         })
//         .collect();

//     let call = generate_call_to_method_expr(m);
//     let body_with_result = generate_body_with_result(&m.return_type, &call);
//     let nr_args = arg_index + 1;

//     quote! {
//         #api_static_init
//         #payable_snippet
//         #only_owner_snippet
//         elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
//             &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
//             #nr_args);
//         #(#arg_init_snippets)*
//         #body_with_result
//     }
// }

// fn generate_call_method_body_variable_nr_args(m: &Method) -> proc_macro2::TokenStream {
//     let api_static_init = snippets::call_method_api_static_init();
//     let payable_snippet = generate_payable_snippet(m);
//     let only_owner_snippet = generate_only_owner_snippet(m);

//     let arg_init_snippets: Vec<proc_macro2::TokenStream> = m
//         .method_args
//         .iter()
//         .map(|arg| {
//             if arg.is_endpoint_arg() {
//                 generate_load_dyn_arg(arg, &quote! { &mut ___arg_loader })
//             } else {
//                 quote! {}
//             }
//         })
//         .collect();

//     let call = generate_call_to_method_expr(m);
//     let body_with_result = generate_body_with_result(&m.return_type, &call);

//     quote! {
//         #api_static_init

//         #payable_snippet

//         #only_owner_snippet

//         let mut ___arg_loader = elrond_wasm::io::EndpointDynArgLoader::<Self::Api>::new();

//         #(#arg_init_snippets)*

//         elrond_wasm::io::assert_no_more_args::<Self::Api, _>(&___arg_loader);

//         #body_with_result
//     }
// }

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
