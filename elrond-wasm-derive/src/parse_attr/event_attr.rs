use super::attr_names::*;
use super::util::*;

pub struct EventAttribute {
	pub identifier: Vec<u8>,
}

impl EventAttribute {
	pub fn parse(m: &syn::TraitItemMethod) -> Option<EventAttribute> {
		match find_attr_one_string_arg(m, ATTR_EVENT) {
			None => None,
			Some(event_str) => {
				if !event_str.starts_with("0x") {
					panic!("event id should start with '0x'");
				}
				if event_str.len() != 64 + 2 {
					panic!("event id should be 64 characters long (32 bytes)");
				}
				let substr = &event_str[2..];
				let result_str = substr.to_string();
				match hex::decode(result_str) {
					Ok(v) => Some(EventAttribute { identifier: v }),
					Err(_) => panic!("could not parse event id"),
				}
			},
		}
	}
}
