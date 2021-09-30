use super::{
    arg_regular::*, method_gen::generate_arg_call_name, only_owner_gen::*, payable_gen::*, util::*,
};
use crate::model::Method;

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

pub fn generate_call_method_body(m: &Method) -> proc_macro2::TokenStream {
    if m.has_variable_nr_args() {
        generate_call_method_body_variable_nr_args(m)
    } else {
        generate_call_method_body_fixed_args(m)
    }
}

pub fn generate_call_method_body_fixed_args(m: &Method) -> proc_macro2::TokenStream {
    let payable_snippet = generate_payable_snippet(m);
    let only_owner_snippet = generate_only_owner_snippet(m);

    let mut arg_index = -1i32;
    let arg_init_snippets: Vec<proc_macro2::TokenStream> = m
        .method_args
        .iter()
        .map(|arg| {
            assert!(
                !arg.metadata.var_args,
                "var_args not accepted in function generate_call_method_fixed_args"
            );

            if arg.is_endpoint_arg() {
                arg_index += 1;
                let pat = &arg.pat;
                let arg_get = generate_load_single_arg(arg, &quote! { #arg_index });
                quote! {
                    let #pat = #arg_get;
                }
            } else {
                quote! {}
            }
        })
        .collect();

    let call = generate_call_to_method_expr(m);
    let body_with_result = generate_body_with_result(&m.return_type, &call);
    let nr_args = arg_index + 1;

    quote! {
        #payable_snippet
        #only_owner_snippet
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.raw_vm_api(), #nr_args);
        #(#arg_init_snippets)*
        #body_with_result
    }
}

fn generate_call_method_body_variable_nr_args(m: &Method) -> proc_macro2::TokenStream {
    let payable_snippet = generate_payable_snippet(m);
    let only_owner_snippet = generate_only_owner_snippet(m);

    let arg_init_snippets: Vec<proc_macro2::TokenStream> = m
        .method_args
        .iter()
        .map(|arg| {
            if arg.is_endpoint_arg() {
                generate_load_dyn_arg(arg, &quote! { &mut ___arg_loader })
            } else {
                quote! {}
            }
        })
        .collect();

    let call = generate_call_to_method_expr(m);
    let body_with_result = generate_body_with_result(&m.return_type, &call);

    quote! {
        #payable_snippet

        #only_owner_snippet

        let mut ___arg_loader = EndpointDynArgLoader::new(self.raw_vm_api());

        #(#arg_init_snippets)*

        ___arg_loader.assert_no_more_args();

        #body_with_result
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
                elrond_wasm::io::EndpointResult::finish(&result, self.raw_vm_api());
            }
        },
    }
}
