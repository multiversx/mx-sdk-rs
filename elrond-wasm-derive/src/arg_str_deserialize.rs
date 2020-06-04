
use super::arg_def::*;
use super::util::*;

pub fn arg_deserialize_next(cb_deser_expr: &proc_macro2::TokenStream, _arg: &MethodArg) -> proc_macro2::TokenStream {
    let main_err = byte_slice_literal(&b"callback argument deserialization error"[..]);
    let type_name_bytes = byte_slice_literal(&[]);
    let hex_bytes = byte_slice_literal(&b"hex"[..]);
    let hex_err_bytes = byte_slice_literal(&[]);
    quote! {
        match #cb_deser_expr.next_argument() {
            Ok(Some(arg_bytes)) => {
                match elrond_wasm::esd_light::decode_from_byte_slice(arg_bytes.as_slice()) {
                    Ok(v) => v,
                    Err(de_err) => self.api.signal_esd_light_error(#main_err, #type_name_bytes, de_err.message_bytes()),
                }
            },
            _ => self.api.signal_esd_light_error(#main_err, #hex_bytes, #hex_err_bytes),
        }
    }

    // match &arg.ty {
    //     syn::Type::Path(type_path) => {
    //         let type_path_segment = type_path.path.segments.last().unwrap().clone();
    //         arg_deserialize_next_single(&type_path_segment)
    //     },             
    //     syn::Type::Reference(type_reference) => {
    //         if type_reference.mutability.is_some() {
    //             panic!("Mutable references not supported as contract method arguments");
    //         }
    //         match &*type_reference.elem {
    //             syn::Type::Path(type_path) => {
    //                 let type_path_segment = type_path.path.segments.last().unwrap().clone();
    //                 arg_deserialize_next_single(&type_path_segment)
    //             },
    //             _ => {
    //                 panic!("Unsupported reference argument type, reference does not contain type path: {:?}", type_reference)
    //             }
    //         }
    //     },
    //     other_arg => panic!("Unsupported argument type. Only path, reference, array or slice allowed. Found: {:?}", other_arg)
    // }
}

// fn arg_deserialize_next_single(type_path_segment: &syn::PathSegment) -> proc_macro2::TokenStream {
//     let type_str = type_path_segment.ident.to_string();
//     match type_str.as_str() {
//         "Address" | "StorageKey" | "H256" =>
//             quote!{
//                 match cb_data_deserializer.next_h256() {
//                     elrond_wasm::DeserializerResult::NoMore => self.api.signal_error(err_msg::ARG_ASYNC_RETURN_WRONG_NUMBER),
//                     elrond_wasm::DeserializerResult::Err(e) => self.api.signal_error(e),
//                     elrond_wasm::DeserializerResult::Res(h256) => h256,
//                 }
//             },
//         other_stype_str => {
//             panic!("Unsupported argument type {:?} for callback arg init snippet", other_stype_str)
//         }
//     }
// }
