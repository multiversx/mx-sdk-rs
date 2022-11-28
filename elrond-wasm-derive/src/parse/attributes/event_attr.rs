use super::{attr_names::*, util::*};

pub struct EventAttribute {
    pub identifier: String,
}

impl EventAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<Self> {
        is_attr_one_string_arg(attr, ATTR_EVENT).map(|arg_str| EventAttribute {
            identifier: arg_str,
        })
    }
}

pub struct LegacyEventAttribute {
    pub identifier: Vec<u8>,
}

impl LegacyEventAttribute {
    pub fn parse(attr: &syn::Attribute) -> Option<LegacyEventAttribute> {
        match is_attr_one_string_arg(attr, ATTR_LEGACY_EVENT) {
            None => None,
            Some(event_str) => {
                assert!(
                    event_str.starts_with("0x"),
                    "event id should start with '0x'"
                );
                assert!(
                    event_str.len() == 64 + 2,
                    "event id should be 64 characters long (32 bytes)"
                );
                let substr = &event_str[2..];
                let result_str = substr.to_string();
                match hex::decode(result_str) {
                    Ok(v) => Some(LegacyEventAttribute { identifier: v }),
                    Err(_) => panic!("could not parse event id"),
                }
            },
        }
    }
}
