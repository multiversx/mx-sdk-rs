derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum Status {
	Inactive,
	Running,
	Ended,
	DistributingPrizes,
}
