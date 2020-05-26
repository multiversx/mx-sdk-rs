use super::contract_gen_method::*;
use super::arg_def::*;
//use super::parse_attr::*;
use super::util::*;

fn generate_topic_conversion_code(topic_index: usize, arg: &MethodArg) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    match &arg.ty {
        syn::Type::Reference(type_reference) => {
            if type_reference.mutability.is_some() {
                panic!("[Event topic] Mutable references not supported as contract method arguments");
            }
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_str = type_path.path.segments.last().unwrap().ident.to_string();
                    match type_str.as_str() {
                        "Address" | "StorageKey" | "H256" =>
                            quote!{
                                #pat.copy_to_array(&mut topics[#topic_index]);
                            },
                        "BigInt" =>
                            panic!("[Event data] BigInt argument type currently not supported"),
                        "BigUint" =>
                            quote!{
                                #pat.copy_to_array_big_endian_pad_right(&mut topics[#topic_index]);
                            },
                        other_stype_str => {
                            panic!("[Event topic] Unsupported reference argument type: {:?}", other_stype_str)
                        }
                    }
                },
                _ => {
                    panic!("[Event topic] Unsupported reference argument type: {:?}", type_reference)
                }
            }
            
        },
        other_arg => panic!("[Event topic] Unsupported argument type: {:?}, should be reference", other_arg)
    }
}

pub fn generate_event_impl(m: &Method, event_id_bytes: Vec<u8>) -> proc_macro2::TokenStream {
    let nr_args_no_self = m.method_args.len();
    if nr_args_no_self == 0 {
        panic!("events need at least 1 argument, for the data");
    }
    let nr_topics = nr_args_no_self as usize; // -1 data, +1 event id

    let mut topic_index: usize = 1;
    let topic_conv_snippets: Vec<proc_macro2::TokenStream> = 
        m.method_args
            .iter()
            .map(|arg| {
                let result =
                    if topic_index < nr_args_no_self {
                        let conversion = generate_topic_conversion_code(topic_index, arg);
                        quote! {
                            #conversion
                        }
                    } else {
                        let pat = &arg.pat;
                        quote! {
                            let data_vec = match elrond_wasm::esd_serde::to_bytes(#pat) {
                                Ok(data_bytes) => data_bytes,
                                Err(sd_err) => {
                                    self.api.signal_sd_error("event serialization error", "data", sd_err);
                                }
                            };
                        }
                    };
                topic_index += 1;
                result
            })
            .collect();
    let msig = m.generate_sig();
    let event_id_literal = array_literal(event_id_bytes.as_slice());
    quote! {
        #msig {
            let mut topics = [[0u8; 32]; #nr_topics];
            topics[0] = #event_id_literal;
            #(#topic_conv_snippets)*
            self.api.write_log(&topics[..], &data_vec.as_slice());
        }
    }
}
