use elrond_wasm::elrond_codec::*;

#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub enum UserStatus {
	New,
	Registered,
	Withdrawn,
}
