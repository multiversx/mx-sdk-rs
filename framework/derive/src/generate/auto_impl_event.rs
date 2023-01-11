use super::{method_gen, util::*};
use crate::model::{Method, MethodArgument};

pub fn generate_event_impl(m: &Method, event_identifier: &str) -> proc_macro2::TokenStream {
    let mut data_arg: Option<&MethodArgument> = None;
    let mut topic_args = Vec::<&MethodArgument>::new();
    for arg in &m.method_args {
        if arg.metadata.event_topic {
            topic_args.push(arg);
        } else if data_arg.is_none() {
            data_arg = Some(arg);
        } else {
            panic!("only 1 data argument allowed in event log");
        }
    }

    let topic_push_snippets: Vec<proc_macro2::TokenStream> = topic_args
        .iter()
        .map(|arg| {
            let topic_pat = &arg.pat;
            quote! {
                multiversx_sc::log_util::serialize_event_topic(&mut ___topic_accumulator___, #topic_pat);
            }
        })
        .collect();
    let data_buffer_snippet = if let Some(data_arg) = data_arg {
        let data_pat = &data_arg.pat;
        quote! {
            let ___data_buffer___ = multiversx_sc::log_util::serialize_log_data(#data_pat);
        }
    } else {
        quote! {
            let ___data_buffer___ = multiversx_sc::types::ManagedBuffer::<Self::Api>::new();
        }
    };

    let msig = method_gen::generate_sig_with_attributes(m);
    let event_identifier_literal = byte_slice_literal(event_identifier.as_bytes());
    quote! {
        #msig {
            let mut ___topic_accumulator___ = multiversx_sc::log_util::event_topic_accumulator::<Self::Api>(
                #event_identifier_literal,
            );
            #(#topic_push_snippets)*
            #data_buffer_snippet
            multiversx_sc::log_util::write_log(&___topic_accumulator___, &___data_buffer___);
        }
    }
}

/// Still only used in legacy event logs.
fn generate_topic_conversion_code(
    topic_index: usize,
    arg: &MethodArgument,
) -> proc_macro2::TokenStream {
    let pat = &arg.pat;
    match &arg.ty {
        syn::Type::Reference(type_reference) => {
            assert!(
                type_reference.mutability.is_none(),
                "[Event topic] Mutable references not supported as contract method arguments"
            );
            match &*type_reference.elem {
                syn::Type::Path(type_path) => {
                    let type_str = type_path.path.segments.last().unwrap().ident.to_string();
                    match type_str.as_str() {
                        "Address" | "H256" => quote! {
                            #pat.copy_to_array(&mut topics[#topic_index]);
                        },
                        "BigInt" => {
                            panic!("[Event data] BigInt argument type currently not supported")
                        },
                        "BigUint" => quote! {
                            #pat.copy_to_array_big_endian_pad_right(&mut topics[#topic_index]);
                        },
                        other_stype_str => panic!(
                            "[Event topic] Unsupported reference argument type: {other_stype_str:?}"
                        ),
                    }
                },
                _ => {
                    panic!("[Event topic] Unsupported reference argument type: {type_reference:?}")
                },
            }
        },
        other_arg => {
            panic!("[Event topic] Unsupported argument type: {other_arg:?}, should be reference")
        },
    }
}

pub fn generate_legacy_event_impl(m: &Method, event_id_bytes: &[u8]) -> proc_macro2::TokenStream {
    let nr_args_no_self = m.method_args.len();
    assert!(
        nr_args_no_self != 0,
        "events need at least 1 argument, for the data"
    );
    let nr_topics = nr_args_no_self; // -1 data, +1 event id

    let mut topic_index: usize = 1;
    let topic_conv_snippets: Vec<proc_macro2::TokenStream> = m
        .method_args
        .iter()
        .map(|arg| {
            let result = if topic_index < nr_args_no_self {
                let conversion = generate_topic_conversion_code(topic_index, arg);
                quote! {
                    #conversion
                }
            } else {
                let pat = &arg.pat;
                quote! {
                    let data_vec = match multiversx_sc::codec::top_encode_to_vec_u8(&#pat) {
                        Result::Ok(data_vec) => data_vec,
                        Result::Err(encode_err) => multiversx_sc::api::ErrorApiImpl::signal_error(
                            &Self::Api::error_api_impl(),
                            encode_err.message_bytes()
                        ),
                    };
                }
            };
            topic_index += 1;
            result
        })
        .collect();
    let msig = method_gen::generate_sig_with_attributes(m);
    let event_id_literal = array_literal(event_id_bytes);
    quote! {
        #msig {
            let mut topics = [[0u8; 32]; #nr_topics];
            topics[0] = #event_id_literal;
            #(#topic_conv_snippets)*
            multiversx_sc::api::LogApiImpl::write_legacy_log(
                &Self::Api::log_api_impl(),
                &topics[..],
                &data_vec.as_slice()
            );
        }
    }
}
