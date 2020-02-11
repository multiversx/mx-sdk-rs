use super::util::*;

pub struct Callable {
    pub trait_name: proc_macro2::Ident,
    pub callable_impl_name: proc_macro2::Ident,
    pub contract_impl_name: proc_macro2::Ident,
    trait_methods: Vec<syn::TraitItemMethod>,
}

impl Callable {
    pub fn new(args: syn::AttributeArgs, contract_trait: &syn::ItemTrait) -> Self {
        let callable_impl_name = generate_callable_interface_impl_struct_name(&contract_trait.ident);
        let contract_impl_name = extract_struct_name(args);
        let trait_methods = extract_methods(&contract_trait);
        Callable {
            trait_name: contract_trait.ident.clone(),
            callable_impl_name: callable_impl_name,
            contract_impl_name: contract_impl_name,
            trait_methods: trait_methods,
        }
    }
}

impl Callable {
    pub fn extract_method_sigs(&self) -> Vec<proc_macro2::TokenStream> {
        self.trait_methods.iter().map(|m| {
            let msig = &m.sig;
            let sig = quote! {
                #msig;
            };
            sig
        }).collect()
    }

    pub fn generate_method_impl(&self) -> Vec<proc_macro2::TokenStream> {
        self.trait_methods.iter().map(|m| {
            let msig = &m.sig;
            let mut arg_index = -1;
            let arg_push_snippets: Vec<proc_macro2::TokenStream> = 
                msig.decl.inputs
                    .iter()
                    .map(|arg| {
                        let snippet = generate_arg_push_snippet(arg, arg_index);
                        arg_index=arg_index+1;
                        snippet
                    })
                    .collect();

            let msig_str = msig.ident.to_string();
            let sig = quote! {
                #msig {
                    let amount = BigInt::from(0);
                    let mut data = String::from(#msig_str);
                    #(#arg_push_snippets)*
                    self.api.async_call(&self.address, &amount, data.as_str());
                }
            };
            sig
        }).collect()
    }
}

fn generate_push_snippet_for_arg_type(type_path_segment: &syn::PathSegment, pat: &syn::Pat, _arg_index_i32: i32) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "Address" =>
            panic!("[callable] Address arguments not yet supported"),
        "Vec" => {
                match &type_path_segment.arguments {
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
                        if args.len() != 1 {
                            panic!("[callable] Vec type must have exactly 1 generic type argument");
                        }
                        if let syn::GenericArgument::Type(vec_type) = args.first().unwrap().into_value() {
                            match vec_type {                
                                syn::Type::Path(type_path) => {
                                    let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                                    let type_str = type_path_segment.ident.to_string();
                                    match type_str.as_str() {
                                        "u8" => quote!{
                                            panic!("[callable] Vec<u8> arguments not yet supported"),
                                        },
                                        other_type => panic!("[callable] Unsupported type: Vec<{:?}>", other_type)
                                    }
                                },
                                other_type => panic!("[callable] Unsupported Vec generic type: {:?}, not a path", other_type)
                            }
                        } else {
                            panic!("[callable] Vec type arguments must be types")
                        }
                    },
                    _ => panic!("[callable] Vec angle brackets expected")
                }
            },
        "BigInt" =>
            panic!("[callable] BigInt arguments not yet supported"),
        "BigUint" =>
            panic!("[callable] BigUint arguments not yet supported"),
        "i32" =>
            quote!{
                elrond_wasm::str_util::push_i32(&mut data, #pat);
            },
        "i64" =>
            quote!{
                elrond_wasm::str_util::push_i64(&mut data, #pat);
            },
        other_stype_str => {
            panic!("[callable] Unsupported argument type {:?} for arg init snippet", other_stype_str)
        }
    }
}

pub fn generate_arg_push_snippet(arg: &syn::FnArg, arg_index: isize) -> proc_macro2::TokenStream {
    match arg {
        syn::FnArg::SelfRef(ref selfref) => {
            if !selfref.mutability.is_none() || arg_index != -1 {
                panic!("ABI function must have `&self` as its first argument.");
            }
            quote!{}
        },
        syn::FnArg::Captured(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            let arg_index_i32 = arg_index as i32;
            match ty {                
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                    generate_push_snippet_for_arg_type(&type_path_segment, pat, arg_index_i32)
                },             
                syn::Type::Reference(type_reference) => {
                    if type_reference.mutability != None {
                        panic!("Mutable references not supported as contract method arguments");
                    }
                    match &*type_reference.elem {
                        syn::Type::Path(type_path) => {
                            let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                            generate_push_snippet_for_arg_type(&type_path_segment, pat, arg_index_i32)
                        },
                        _ => {
                            panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                        }
                    }
                },
                other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
			}
        }
        other_arg => panic!("Unsupported argument type: {:?}, not captured", other_arg)
    }
}