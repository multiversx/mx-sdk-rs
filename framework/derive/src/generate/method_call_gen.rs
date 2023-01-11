use crate::{
    generate::{
        method_call_gen_arg::{
            generate_call_method_arg_load, load_call_result_args_snippet,
            load_cb_closure_args_snippet,
        },
        method_gen::generate_arg_call_name,
        payable_gen::*,
        restricted_caller_gen::*,
        snippets,
        util::*,
    },
    model::Method,
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
                multiversx_sc::io::finish_multi::<Self::Api, _>(&result);
            }
        },
    }
}

pub fn generate_endpoint_call_method_body(m: &Method) -> proc_macro2::TokenStream {
    let api_static_init = snippets::call_method_api_static_init();
    let payable_snippet = generate_payable_snippet(m);
    let only_owner_snippet = generate_only_owner_snippet(m);
    let only_admin_snippet = generate_only_admin_snippet(m);
    let only_user_account_snippet = generate_only_user_account_snippet(m);
    let arg_load = generate_call_method_arg_load(m);

    let call = generate_call_to_method_expr(m);
    let body_with_result = generate_body_with_result(&m.return_type, &call);

    quote! {
        #api_static_init
        #payable_snippet
        #only_owner_snippet
        #only_admin_snippet
        #only_user_account_snippet
        #arg_load
        #body_with_result
    }
}

pub fn generate_promises_callback_call_method_body(m: &Method) -> proc_macro2::TokenStream {
    let api_static_init = snippets::call_method_api_static_init();
    let payable_snippet = generate_payable_snippet(m);
    let cb_closure_args_snippet = load_cb_closure_args_snippet(m);
    let call_result_args_snippet = load_call_result_args_snippet(m);

    let call = generate_call_to_method_expr(m);
    let body_with_result = generate_body_with_result(&m.return_type, &call);

    quote! {
        #api_static_init
        #payable_snippet
        #cb_closure_args_snippet
        #call_result_args_snippet
        #body_with_result
    }
}
