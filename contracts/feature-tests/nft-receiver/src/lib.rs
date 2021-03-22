#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract(NftReceiverImpl)]
pub trait NftReceiver {
	#[init]
	fn init(&self) {}

	#[payable("*")]
	#[endpoint(acceptAndReturnCallData)]
	fn accept_and_return_call_data(&self) -> MultiResultVec<BoxedBytes> {
		let mut result = Vec::new();
		let token_type = self.call_value().esdt_token_type();
		let token_identifier = self.call_value().token();
		let nonce = self.call_value().esdt_token_nonce();
		let amount = self.call_value().esdt_value();

		result.push(token_type.as_type_name().into());
		result.push(token_identifier.as_esdt_identifier().into());
		result.push(nonce.to_be_bytes()[..].into());
		result.push(amount.to_bytes_be().as_slice().into());

		result.into()
	}

	#[payable("*")]
	#[endpoint(acceptNft)]
	fn accept_nft(&self) -> SCResult<()> {
		Ok(())
	}

	#[payable("*")]
	#[endpoint(rejectNft)]
	fn reject_nft(&self) -> SCResult<()> {
		sc_error!("NFT rejected!")
	}
}
