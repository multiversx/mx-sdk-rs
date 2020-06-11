

use super::arg_def::*;
use super::util::*;


fn arg_regular_single(type_path_segment: &syn::PathSegment, arg_index_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        "Address" | "StorageKey" | "H256" =>
            quote!{
                self.api.get_argument_address(#arg_index_expr)
            },
        // "Vec" => {
        //         let vec_generic_type_segm = generic_type_single_arg_segment(&"Vec", &type_path_segment);
        //         let type_str = vec_generic_type_segm.ident.to_string();
        //         match type_str.as_str() {
        //             "u8" => quote!{
        //                 self.api.get_argument_vec(#arg_index_expr)
        //             },
        //             other_type => panic!("Unsupported type: Vec<{:?}>", other_type)
        //         }
        //     },
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
        "u64" =>
            quote!{
                self.api.get_argument_u64(#arg_index_expr)
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
        type_name => {
            let main_err = byte_slice_literal(&b"argument deserialization error"[..]);
            let type_name_bytes = byte_slice_literal(type_name.as_bytes());
            quote!{
                {
                    let arg_bytes = self.api.get_argument_vec(#arg_index_expr);
                    match elrond_wasm::esd_light::decode_from_byte_slice(arg_bytes.as_slice()) {
                        Ok(v) => v,
                        Err(de_err) => self.api.signal_esd_light_error(#main_err, #type_name_bytes, de_err.message_bytes()),
                    }
                }
            }
        },
    }
}

pub fn arg_regular(arg: &MethodArg, arg_index_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            arg_regular_single(&type_path_segment, arg_index_expr)
        },             
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    arg_regular_single(&type_path_segment, arg_index_expr)
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
        syn::Type::Tuple(syn::TypeTuple{elems, ..}) => {
            // allow empty tuples as arguments, nothing happens
            if elems.len() == 0 {
                quote! {
                    ()
                }
            } else {
                panic!("Only empty tuples accepted as arguments for now")
            }
        },
        other_arg => panic!("Unsupported argument type. Only path, reference, array or slice allowed. Found: {:?}", other_arg)
    }
}

pub fn arg_regular_multi(arg: &MethodArg, arg_index_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            let type_str = type_path_segment.ident.to_string();
            match type_str.as_str() {
                "Vec" => {
                    let vec_generic_type_segm = generic_type_single_arg_segment(&"Vec", &type_path_segment);
                    let get_snippet = arg_regular_single(&vec_generic_type_segm, arg_index_expr);
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

pub fn arg_regular_callback(
            arg: &MethodArg, 
            arg_index_expr: &proc_macro2::TokenStream,
            nr_args_expr: &proc_macro2::TokenStream,
        ) -> proc_macro2::TokenStream {
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            let type_str = type_path_segment.ident.to_string();
            match type_str.as_str() {
                "AsyncCallResult" => {
                    let type_name = &"AsyncCallResult";
                    //let vec_generic_type_segm = generic_type_single_arg_segment(&"AsyncCallResult", &type_path_segment);
                    let get_snippet = match &type_path_segment.arguments {
                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
                            if args.len() != 1 {
                                panic!("{} type must have exactly 1 generic type argument", type_name);
                            }
                            if let syn::GenericArgument::Type(gen_type) = args.first().unwrap() {
                                match gen_type {                
                                    syn::Type::Path(type_path) => {
                                        let generic_type_segm = type_path.path.segments.last().unwrap().clone();
                                        let type_str = generic_type_segm.ident.to_string();
                                        if type_str == "VarArgs" {
                                            let var_args_generic_type_segm = generic_type_single_arg_segment(&"VarArgs", &generic_type_segm);
                                            let var_arg_elem = arg_regular_single(&var_args_generic_type_segm, arg_index_expr);
                                            quote! {
                                                {
                                                    let mut var_args: #gen_type = Vec::new();
                                                    while #arg_index_expr < #nr_args_expr {
                                                        var_args.push(#var_arg_elem);
                                                        #arg_index_expr += 1;
                                                    }
                                                    var_args
                                                }
                                            }
                                        } else {
                                            let checked_arg_index_expr = quote!{
                                                {
                                                    if #arg_index_expr >= #nr_args_expr {
                                                        self.api.signal_error(err_msg::ARG_WRONG_NUMBER);
                                                    }
                                                    #arg_index_expr += 1;
                                                    #arg_index_expr - 1
                                                }
                                            };
                                            arg_regular_single(&generic_type_segm, &checked_arg_index_expr)
                                        }
                                    },
                                    syn::Type::Tuple(syn::TypeTuple{elems, ..}) => {
                                        // allow empty tuple (for now)
                                        if elems.len() == 0 {
                                            quote! { () }
                                        } else {
                                            panic!("Only empty tuples accepted in AsyncCallResult")
                                        }
                                    },
                                    other_type => panic!("Unsupported {} generic type: {:?}, not a path", type_name, other_type)
                                }
                            } else {
                                panic!("{} type arguments must be types", type_name)
                            }
                        },
                        _ => panic!("{} angle brackets expected", type_name)
                    };
                    // let get_snippet = arg_regular_single(&vec_generic_type_segm, arg_index_expr); // TODO: support tuples
                    // let pat = &arg.pat;
                    quote! {
                        match { let err_code = self.api.get_argument_i64(#arg_index_expr); #arg_index_expr += 1; err_code } {
                            0 => AsyncCallResult::Ok(#get_snippet),
                            err_code => AsyncCallResult::Err(AsyncCallError{
                                err_code: err_code as i32,
                                err_msg: self.api.get_argument_vec(#arg_index_expr),
                            })
                        }
                    }
                },
                other_stype_str => {
                    panic!("Unsupported argument type {:?} for callback argument", other_stype_str)
                }
            }
        },
        other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
    }
}


