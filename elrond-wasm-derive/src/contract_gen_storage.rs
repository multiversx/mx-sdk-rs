use super::contract_gen_method::*;
use super::arg_def::*;
use super::util::*;

fn storage_store_default_impl(value_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote!{
        #value_expr.using_top_encoded(|bytes| {
            self.api.storage_store(key, bytes);
        });
    }
}

fn storage_store_snippet_for_type(_type_path_segment: &syn::PathSegment, value_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        elrond_wasm::storage_set(&self.api, key, #value_expr);
    }
}

fn storage_store_snippet(arg: &MethodArg) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            storage_store_snippet_for_type(&type_path_segment, &quote!{ & #pat })
        },             
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported in setters");
            }
            let value_expr = &quote!{ #pat };
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    storage_store_snippet_for_type(&type_path_segment, value_expr)
                },
                _ => storage_store_default_impl(value_expr)
            }
        },
        other_arg => panic!("Unsupported argument type. Only path, reference, array or slice allowed. Found: {:?}", other_arg)
    }
}

fn storage_load_snippet_for_type(_type_path_segment: &syn::PathSegment) -> proc_macro2::TokenStream {
    quote! {
        elrond_wasm::storage_get(&self.api, key)
    }
}

fn storage_load_snippet(ty: &syn::Type) -> proc_macro2::TokenStream {
    match ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            storage_load_snippet_for_type(&type_path_segment)
        },             
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported in setters");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    storage_load_snippet_for_type(&type_path_segment)
                },
                _ => {
                    panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                }
            }
        },
        other_arg => panic!("Unsupported argument type. Only path, reference, array or slice allowed. Found: {:?}", other_arg)
    }
}

fn generate_key_snippet(key_args: &[MethodArg], identifier: String) -> proc_macro2::TokenStream {
    let id_literal = array_literal(identifier.as_bytes());
    if key_args.len() == 0 {
        // hardcode key
        quote! {
            let key = &#id_literal;
        }
    } else {
        // build key from arguments
        let key_appends: Vec<proc_macro2::TokenStream> = key_args.iter().map(|arg| {
            let arg_pat = &arg.pat;
            quote! {
                #arg_pat.dep_encode_to(&mut key_bytes);
            }
        }).collect();
        quote! {
            let mut key_bytes: Vec<u8> = #id_literal.to_vec();
            #(#key_appends)*
            let key = key_bytes.as_slice();
        }
    }
}

pub fn generate_getter_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
    let msig = m.generate_sig();
    let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
    match m.return_type.clone() {
        syn::ReturnType::Default => panic!("getter should return some value"),
        syn::ReturnType::Type(_, ty) => {
            let load_snippet = storage_load_snippet(&ty);
            quote! {
                #msig {
                    #key_snippet
                    #load_snippet
                }
            }
        },
    }
}

pub fn generate_setter_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
    let msig = m.generate_sig();
    if m.method_args.len() == 0 {
        panic!("setter must have at least one argument, for the value");
    }
    if m.return_type != syn::ReturnType::Default {
        panic!("setter should not return anything");
    }
    let key_args = &m.method_args[..m.method_args.len()-1];
    let key_snippet = generate_key_snippet(key_args, identifier);
    let value_arg = &m.method_args[m.method_args.len()-1];
    let store_snippet = storage_store_snippet(value_arg);
    quote! {
        #msig {
            #key_snippet
            #store_snippet
        }
    }
}
