

use super::arg_def::*;
use super::util::*;

pub fn generate_load_single_arg(arg: &MethodArg, arg_index_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let arg_ty = &arg.ty;
    let arg_name_literal = pat_literal(&arg.pat);
    match &arg.ty {
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            let referenced_type = &*type_reference.elem;
            quote! {
                & elrond_wasm::load_single_arg::<T, BigInt, BigUint, #referenced_type>(&self.api, #arg_index_expr, #arg_name_literal)
            }
        },
        _ => {
            quote! {
                elrond_wasm::load_single_arg::<T, BigInt, BigUint, #arg_ty>(&self.api, #arg_index_expr, #arg_name_literal)
            }
        },
    }
}

pub fn generate_load_dyn_arg(arg: &MethodArg,
        loader_expr: &proc_macro2::TokenStream,
        err_handler_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {

    let pat = &arg.pat;
    let arg_ty = &arg.ty;
    let arg_name_literal = pat_literal(pat);
    match &arg.ty {
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            let referenced_type = &*type_reference.elem;
            quote! {
                let #pat: & #referenced_type = &elrond_wasm::load_dyn_arg(#loader_expr, #err_handler_expr, #arg_name_literal);
            }
        },
        _ => {
            quote! {
                let #pat: #arg_ty = elrond_wasm::load_dyn_arg(#loader_expr, #err_handler_expr, #arg_name_literal);
            }
        },
    }
}

pub fn generate_load_dyn_multi_arg(arg: &MethodArg,
    loader_expr: &proc_macro2::TokenStream,
    err_handler_expr: &proc_macro2::TokenStream,
    num_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {

    let pat = &arg.pat;
    let arg_ty = &arg.ty;
    let arg_name_literal = pat_literal(pat);
    match &arg.ty {
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            let referenced_type = &*type_reference.elem;
            quote! {
                let #pat: & #referenced_type = &elrond_wasm::load_dyn_multi_arg(#loader_expr, #err_handler_expr, #arg_name_literal, #num_expr);
            }
        },
        _ => {
            quote! {
                let #pat: #arg_ty = elrond_wasm::load_dyn_multi_arg(#loader_expr, #err_handler_expr, #arg_name_literal, #num_expr);
            }
        },
    }
}
