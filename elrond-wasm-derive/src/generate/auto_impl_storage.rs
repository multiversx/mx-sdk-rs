use super::{method_gen, util::*};
use crate::model::{Method, MethodArgument};

fn generate_key_snippet(key_args: &[MethodArgument], identifier: &str) -> proc_macro2::TokenStream {
    let id_literal = byte_str_literal(identifier.as_bytes());

    // build base key from arguments
    let key_appends: Vec<proc_macro2::TokenStream> = key_args
        .iter()
        .map(|arg| {
            let arg_pat = &arg.pat;
            quote! {
                ___key___.append_item(& #arg_pat);
            }
        })
        .collect();
    quote! {
        let mut ___key___ = elrond_wasm::storage::StorageKey::<Self::Api>::new(
            self.raw_vm_api(),
            &#id_literal[..],
        );
        #(#key_appends)*
    }
}

pub fn generate_getter_impl(m: &Method, identifier: &str) -> proc_macro2::TokenStream {
    let msig = method_gen::generate_sig_with_attributes(m);
    let key_snippet = generate_key_snippet(m.method_args.as_slice(), identifier);
    match m.return_type.clone() {
        syn::ReturnType::Default => panic!("getter should return some value"),
        syn::ReturnType::Type(_, _ty) => {
            quote! {
                #msig {
                    #key_snippet
                    elrond_wasm::storage::storage_get(self.raw_vm_api(), &___key___)
                }
            }
        },
    }
}

pub fn generate_setter_impl(m: &Method, identifier: &str) -> proc_macro2::TokenStream {
    let msig = method_gen::generate_sig_with_attributes(m);
    assert!(
        !m.method_args.is_empty(),
        "setter must have at least one argument, for the value"
    );
    assert!(
        m.return_type == syn::ReturnType::Default,
        "setter should not return anything"
    );
    let key_args = &m.method_args[..m.method_args.len() - 1];
    let key_snippet = generate_key_snippet(key_args, identifier);
    let value_arg = &m.method_args[m.method_args.len() - 1];
    let pat = &value_arg.pat;
    quote! {
        #msig {
            #key_snippet
            elrond_wasm::storage::storage_set(self.raw_vm_api(), &___key___, & #pat);
        }
    }
}

pub fn generate_mapper_impl(m: &Method, identifier: &str) -> proc_macro2::TokenStream {
    let msig = method_gen::generate_sig_with_attributes(m);
    let key_snippet = generate_key_snippet(m.method_args.as_slice(), identifier);
    match m.return_type.clone() {
        syn::ReturnType::Default => panic!("getter should return some value"),
        syn::ReturnType::Type(_, ty) => {
            quote! {
                #msig {
                    #key_snippet
                    <#ty as elrond_wasm::storage::mappers::StorageMapper<Self::Api>>::new(
                        self.raw_vm_api(),
                        ___key___
                    )
                }
            }
        },
    }
}

pub fn generate_is_empty_impl(m: &Method, identifier: &str) -> proc_macro2::TokenStream {
    let msig = method_gen::generate_sig_with_attributes(m);
    let key_snippet = generate_key_snippet(m.method_args.as_slice(), identifier);
    quote! {
        #msig {
            #key_snippet
            elrond_wasm::storage::storage_get_len(self.raw_vm_api(), &___key___) == 0
        }
    }
}

pub fn generate_clear_impl(m: &Method, identifier: &str) -> proc_macro2::TokenStream {
    let msig = method_gen::generate_sig_with_attributes(m);
    assert!(
        m.return_type == syn::ReturnType::Default,
        "storage clear should not return anything"
    );
    let key_snippet = generate_key_snippet(m.method_args.as_slice(), identifier);
    quote! {
        #msig {
            #key_snippet
            elrond_wasm::storage::storage_clear(self.raw_vm_api(), &___key___);
        }
    }
}
