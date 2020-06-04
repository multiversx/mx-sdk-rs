
use super::arg_def::*;
use super::util::*;

fn arg_serialize_push_single(
        type_path_segment: &syn::PathSegment,
        arg_accumulator: &proc_macro2::TokenStream,
        var_name: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {

    let type_str = type_path_segment.ident.to_string();
    match type_str.as_str() {
        // "Address" | "StorageKey" | "H256" => quote!{
        //     #arg_accumulator.push_argument_bytes(#var_name.as_bytes());
        // },
        // "Vec" => {
        //         match &type_path_segment.arguments {
        //             syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{args, ..}) => {
        //                 if args.len() != 1 {
        //                     panic!("[callable] Vec type must have exactly 1 generic type argument");
        //                 }
        //                 if let syn::GenericArgument::Type(vec_type) = args.first().unwrap() {
        //                     match vec_type {
        //                         syn::Type::Path(type_path) => {
        //                             let type_path_segment = type_path.path.segments.last().unwrap().clone();
        //                             let type_str = type_path_segment.ident.to_string();
        //                             match type_str.as_str() {
        //                                 "u8" => quote!{
        //                                     #arg_accumulator.push_argument_bytes(#var_name.as_slice());
        //                                 },
        //                                 other_type => panic!("[callable] Unsupported type: Vec<{:?}>", other_type)
        //                             }
        //                         },
        //                         other_type => panic!("[callable] Unsupported Vec generic type: {:?}, not a path", other_type)
        //                     }
        //                 } else {
        //                     panic!("[callable] Vec type arguments must be types")
        //                 }
        //             },
        //             _ => panic!("[callable] Vec angle brackets expected")
        //         }
        //     },
        "BigInt" =>
            panic!("[callable] BigInt arguments not yet supported"),
        "BigUint" =>
            quote!{
                #arg_accumulator.push_argument_bytes(#var_name.to_bytes_be().as_slice());
            },
        _ =>
            quote!{
                #var_name.using_top_encoded(|bytes| {
                    #arg_accumulator.push_argument_bytes(bytes);
                });
            },
    }
}

pub fn arg_serialize_push(
        arg: &MethodArg,
        arg_accumulator: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {

    let pat = &arg.pat;
    let var_name = quote!{ #pat };
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            arg_serialize_push_single(&type_path_segment, &arg_accumulator, &var_name)
        },
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("Mutable references not supported as contract method arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_path_segment = type_path.path.segments.last().unwrap().clone();
                    arg_serialize_push_single(&type_path_segment, arg_accumulator, &var_name)
                },
                _ => {
                    panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
                }
            }
        },
        other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
    }
}

pub fn arg_serialize_push_multi(
        arg: &MethodArg,
        arg_accumulator: &proc_macro2::TokenStream,
        var_name: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match &arg.ty {
        syn::Type::Path(type_path) => {
            let type_path_segment = type_path.path.segments.last().unwrap().clone();
            let type_str = type_path_segment.ident.to_string();
            match type_str.as_str() {
                "Vec" => {
                    let vec_generic_type_segm = generic_type_single_arg_segment(&"Vec", &type_path_segment);
                    arg_serialize_push_single(&vec_generic_type_segm, arg_accumulator, var_name)
                },
                other_stype_str => {
                    panic!("Unsupported argument type {:?} for multi argument", other_stype_str)
                }
            }
        },
        other_arg => panic!("Unsupported argument type: {:?}, neither path nor reference", other_arg)
    }
}