use super::contract_gen_method::*;
use super::arg_def::*;
//use super::parse_attr::*;
use super::util::*;

fn storage_store_snippet_for_type(type_path_segment: &syn::PathSegment, value_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "BigUint" =>
            quote!{
                self.api.storage_store_big_uint(key, #value_expr);
            },
        _ =>
            quote!{
                match elrond_wasm::serializer::to_bytes(#value_expr) {
                    Ok(bytes) => {
                        self.api.storage_store(key, &bytes);
                    },
                    Err(_) => self.api.signal_error("serialization error")
                }
            }
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
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    storage_store_snippet_for_type(&type_path_segment, &quote!{ #pat })
                },
                _ => {
                    panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                }
            }
        },
        other_arg => panic!("Unsupported argument type. Only path, reference, array or slice allowed. Found: {:?}", other_arg)
    }
}

fn storage_load_snippet_for_type(type_path_segment: &syn::PathSegment) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "BigUint" =>
            quote!{
                self.api.storage_load_big_uint(key)
            },
        _ =>
            quote!{
                let value_bytes = self.api.storage_load(key);
                match elrond_wasm::serializer::from_bytes(value_bytes.as_slice()) {
                    Ok(v) => {
                        v
                    },
                    Err(_) => self.api.signal_error("deserialization error")
                }
            }
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
    let arg_pats: Vec<syn::Pat> = key_args.iter().map(|arg| arg.pat.clone()).collect();
    if key_args.len() == 0 {
        // hardcode key
        quote! {
            let key = &#id_literal;
        }
    } else {
        // build key from arguments
        quote! {
            let key_bytes = match elrond_wasm::serializer::to_bytes(&(&#id_literal, #(#arg_pats),* )) {
                Ok(bytes) => bytes,
                Err(_) => self.api.signal_error("key serialization error")
            };
            let key = key_bytes.as_slice();
        }
    }
}

pub fn generate_getter_impl(m: &Method, identifier: String) -> proc_macro2::TokenStream {
    let msig = m.generate_sig();
    let key_snippet = generate_key_snippet(&m.method_args.as_slice(), identifier);
    match m.return_type.clone() {
        syn::ReturnType::Default => panic!("setter should return some value"),
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
