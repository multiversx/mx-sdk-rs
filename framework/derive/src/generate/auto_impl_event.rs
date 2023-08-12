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
