
//use super::contract_gen::*;
use super::parse_attr::*;

#[derive(Clone, Debug)]
pub struct MethodArg {
    pub index: i32,
    pub pat: syn::Pat,
    pub ty: syn::Type,
    pub metadata: ArgMetadata
}

#[derive(Clone, Debug)]
pub enum ArgMetadata {
    None,
    Payment
}

pub fn extract_method_args(m: &syn::TraitItemMethod, is_method_payable: bool) -> Vec<MethodArg> {
    let mut arg_index: isize = -1; // ignore the first argument, which is &self
    m.sig.inputs
        .iter()
        .filter_map(|arg| {
            let arg_opt = match arg {
                syn::FnArg::Receiver(ref selfref) => {
                    if selfref.mutability.is_some() || arg_index != -1 {
                        panic!("Trait method must have `&self` as its first argument.");
                    }
                    None
                },
                syn::FnArg::Typed(pat_typed) => {
                    let pat = &*pat_typed.pat;
                    let ty = &*pat_typed.ty;

                    if is_payment(&pat_typed) {
                        if !is_method_payable {
                            panic!("Cannot have payment arguments to non-payable methods.");
                        }
                        Some(MethodArg{
                            index: -1, // TODO: move to metadata
                            pat: pat.clone(),
                            ty: ty.clone(), // TODO: check that it is BigUint
                            metadata: ArgMetadata::Payment,
                        })
                    } else {
                        arg_index=arg_index+1;
                        Some(MethodArg{
                            index: arg_index as i32,
                            pat: pat.clone(),
                            ty: ty.clone(),
                            metadata: ArgMetadata::None,
                        })
                    }
                }
            };
            
            arg_opt
        })
        .collect()
}

pub fn generate_arg_call_name(arg: &MethodArg) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    match &arg.ty {                
        syn::Type::Path(_) => quote!{ #pat },
        syn::Type::Reference(_) => quote!{ &#pat },
        other_arg => panic!("Unsupported argument type {:?} in generate_arg_call_name", other_arg),
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
                        if let syn::GenericArgument::Type(vec_type) = args.first().unwrap() {
                            match vec_type {                
                                syn::Type::Path(type_path) => {
                                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
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

pub fn generate_arg_init_snippet(arg: &MethodArg, arg_offset: i32) -> proc_macro2::TokenStream {
    let arg_index = arg.index + arg_offset;
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            generate_snippet_for_arg_type(&type_path_segment, &arg.pat, arg_index)
        },             
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    generate_snippet_for_arg_type(&type_path_segment, &arg.pat, arg_index)
                },
                _ => {
                    panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                }
            }
        },
        other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
    }
}
