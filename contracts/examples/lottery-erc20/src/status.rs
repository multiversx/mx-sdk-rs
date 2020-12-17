use elrond_wasm::elrond_codec::*;

#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub enum Status {
	Inactive,
	Running,
	Ended,
	DistributingPrizes,
}
