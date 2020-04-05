
//use super::contract_gen::*;
use super::parse_attr::*;
use super::util::*;

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
    Payment,
    Multi(MultiAttribute),
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

                    if let Some(multi_attr) = MultiAttribute::parse(&pat_typed) {
                        Some(MethodArg{
                            index: -1, // TODO: move to metadata
                            pat: pat.clone(),
                            ty: ty.clone(), // TODO: check that it is BigUint
                            metadata: ArgMetadata::Multi(multi_attr),
                        })
                    } else if is_payment(&pat_typed) {
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
        syn::Type::Path(_) | syn::Type::Array(_) => quote!{ #pat },
        syn::Type::Reference(_) => quote!{ &#pat },
        other_arg => panic!("Unsupported argument type {:?} in generate_arg_call_name", other_arg),
    }
}

fn generate_snippet_for_arg_type(type_path_segment: &syn::PathSegment, arg_index_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "Address" =>
            quote!{
                self.api.get_argument_address(#arg_index_expr)
            },
        "Vec" => {
                let vec_generic_type_segm = generic_type_single_arg_segment(&"Vec", &type_path_segment);
                let type_str = vec_generic_type_segm.ident.to_string();
                match type_str.as_str() {
                    "u8" => quote!{
                        self.api.get_argument_vec(#arg_index_expr)
                    },
                    other_type => panic!("Unsupported type: Vec<{:?}>", other_type)
                }
            },
        "BigInt" =>
            quote!{
                self.api.get_argument_big_int(#arg_index_expr)
            },
        "BigUint" =>
            quote!{
                self.api.get_argument_big_uint(#arg_index_expr)
            },
        "i64" =>
            quote!{
                self.api.get_argument_i64(#arg_index_expr)
            },
        "i32" =>
            quote!{
                self.api.get_argument_i32(#arg_index_expr)
            },
        "u32" =>
            quote!{
                self.api.get_argument_u32(#arg_index_expr)
            },
        "isize" =>
            quote!{
                self.api.get_argument_isize(#arg_index_expr)
            },
        "usize" =>
            quote!{
                self.api.get_argument_usize(#arg_index_expr)
            },
        "i8" =>
            quote!{
                self.api.get_argument_i8(#arg_index_expr)
            },
        "u8" =>
            quote!{
                self.api.get_argument_u8(#arg_index_expr)
            },
        "bool" =>
            quote!{
                self.api.get_argument_i64(#arg_index_expr) != 0
            },
        other_stype_str => {
            panic!("Unsupported argument type {:?} for arg init snippet", other_stype_str)
        }
    }
}

pub fn generate_get_arg_snippet(arg: &MethodArg, arg_index_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            generate_snippet_for_arg_type(&type_path_segment, arg_index_expr)
        },             
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    generate_snippet_for_arg_type(&type_path_segment, arg_index_expr)
                },
                _ => {
                    panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                }
            }
        },
        syn::Type::Array(arr) => {
            let arr_len = &arr.len;
            match &*arr.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    let type_str = type_path_segment.ident.to_string();
                    match type_str.as_str() {
                        "u8" => {
                            quote! {
                                {
                                    let mut arr = [0u8; #arr_len];
                                    self.api.copy_argument_to_slice(#arg_index_expr, &mut arr);
                                    arr
                                }
                            }
                        },
                        _ => panic!("Only array of u8 allowed as arguments")
                    }
                },
                _ => panic!("Array type is not Path. Only array of u8 allowed as arguments")
            }
        },
        other_arg => panic!("Unsupported argument type. Only path, reference, array or slice allowed. Found: {:?}", other_arg)
    }
}

pub fn generate_multi_arg_push_snippet(arg: &MethodArg, arg_index_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            let type_str = type_path_segment.ident.to_string();
            match type_str.as_str() {
                "Vec" => {
                    let vec_generic_type_segm = generic_type_single_arg_segment(&"Vec", &type_path_segment);
                    let get_snippet = generate_snippet_for_arg_type(&vec_generic_type_segm, arg_index_expr);
                    let pat = &arg.pat;
                    quote! {
                        #pat.push(#get_snippet);
                    }
                },
                other_stype_str => {
                    panic!("Unsupported argument type {:?} for multi argument", other_stype_str)
                }
            }
        },
        other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
    }
}
