
use super::contract_gen::*;

pub fn generate_arg_call_name(arg: &syn::FnArg) -> Option<proc_macro2::TokenStream> {
    match arg {
        syn::FnArg::SelfRef(_) => None,
        syn::FnArg::Captured(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            match ty {                
                syn::Type::Path(_) => Some(quote!{ #pat }),
                syn::Type::Reference(_) => Some(quote!{ &#pat }),
                other_arg => panic!("Unsupported argument type {:?} in generate_arg_call_name", other_arg),
            }
        },
        other_arg => panic!("Unsupported argument type {:?} in generate_arg_call_name, neither self, nor captured", other_arg)
    }
}

fn generate_snippet_for_arg_type(type_path_segment: &syn::PathSegment, pat: &syn::Pat, arg_index_i32: i32) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "Address" =>
            quote!{
                let #pat: Address = self.api.get_argument_address(#arg_index_i32);
            },
        "Vec" => {
                match &type_path_segment.arguments {
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
                        if args.len() != 1 {
                            panic!("Vec type must have exactly 1 generic type argument");
                        }
                        if let syn::GenericArgument::Type(vec_type) = args.first().unwrap().into_value() {
                            match vec_type {                
                                syn::Type::Path(type_path) => {
                                    let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                                    let type_str = type_path_segment.ident.to_string();
                                    match type_str.as_str() {
                                        "u8" => quote!{
                                            let #pat: Vec<u8> = self.api.get_argument_vec(#arg_index_i32);
                                        },
                                        other_type => panic!("Unsupported type: Vec<{:?}>", other_type)
                                    }
                                },
                                other_type => panic!("Unsupported Vec generic type: {:?}, not a path", other_type)
                            }
                        } else {
                            panic!("Vec type arguments must be types")
                        }
                    },
                    _ => panic!("Vec angle brackets expected")
                }
            },
        "BigInt" =>
            quote!{
                let #pat = self.api.get_argument_big_int(#arg_index_i32);
            },
        "BigUint" =>
            quote!{
                let #pat = self.api.get_argument_big_uint(#arg_index_i32);
            },
        "i64" =>
            quote!{
                let #pat: i64 = self.api.get_argument_i64(#arg_index_i32);
            },
        other_stype_str => {
            panic!("Unsupported argument type {:?} for arg init snippet", other_stype_str)
        }
    }
}

pub fn generate_arg_init_snippet(arg: &PublicArg, arg_offset: i32) -> proc_macro2::TokenStream {
    match &arg.syn_arg {
        syn::FnArg::Captured(arg_captured) => {
            let pat = &arg_captured.pat;
            let ty = &arg_captured.ty;
            let arg_index = arg.index + arg_offset;
            match ty {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                    generate_snippet_for_arg_type(&type_path_segment, pat, arg_index)
                },             
                syn::Type::Reference(type_reference) => {
                    if type_reference.mutability != None {
                        panic!("Mutable references not supported as contract method arguments");
                    }
                    match &*type_reference.elem {
                        syn::Type::Path(type_path) => {
                            let type_path_segment = type_path.path.segments.last().unwrap().value().clone();
                            generate_snippet_for_arg_type(&type_path_segment, pat, arg_index)
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
