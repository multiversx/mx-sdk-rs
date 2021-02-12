#![no_std]

elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm_derive::contract(Erc1155Impl)]
pub trait Erc1155 {
	#[init]
	fn init(&self) {}

	// endpoints

	#[endpoint(safeTransferFrom)]
	fn safe_transfer_from(
		&self,
		from: Address,
		to: Address,
		id: BigUint,
		value: BigUint,
		data: &[u8],
	) -> SCResult<()> {
		Ok(())
	}

	#[endpoint(safeBatchTransferFrom)]
	fn safe_batch_transfer_from(
		&self,
		from: Address,
		to: Address,
		ids: &[BigUint],
		values: &[BigUint],
		data: &[u8],
	) -> SCResult<()> {
		Ok(())
	}

	#[endpoint(setApprovalForAll)]
	fn set_approved_for_all(&self, operator: Address, approved: bool) -> SCResult<()> {
		Ok(())
	}

	// views

	#[view(supportsInterface)]
	fn supports_interface(&self, interface_id: u32) -> bool {
		interface_id == 0x01ffc9a7 ||    // ERC-165 support (i.e. `bytes4(keccak256('supportsInterface(bytes4)'))`).
			interface_id == 0x4e2312e0 // ERC-1155 `ERC1155TokenReceiver` support (i.e. `bytes4(keccak256("onERC1155Received(address,address,uint256,uint256,bytes)")) ^ bytes4(keccak256("onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"))`).
	}

	#[view(balanceOf)]
	fn balance_of(&self, owner: Address, id: BigUint) -> BigUint {
		BigUint::zero()
	}

	// returns balance for each (owner, id) pair
	#[view(balanceOfBatch)]
	fn balance_of_batch(&self, owners: &[Address], ids: &[BigUint]) -> Vec<BigUint> {
		Vec::new()
	}

	#[view(isApprovedForAll)]
	fn is_approval_for_all(&self, owner: Address, operator: Address) -> bool {
		true
	}

	// Events

	/*
	#[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
	fn transfer_single_event(&self, operator: &Address, from: &Address, to: &Address, id: &BigUint, value: &BigUint);

	#[event("0x0000000000000000000000000000000000000000000000000000000000000002")]
	fn transfer_batch_event(&self, operator: &Address, from: &Address, to: &Address, ids: &Vec<BigUint>, values: &Vec<BigUint>);

	#[event("0x0000000000000000000000000000000000000000000000000000000000000003")]
	fn approval_for_all_event(&self, owner: &Address, operator: &Address, approved: bool);

	#[event("0x0000000000000000000000000000000000000000000000000000000000000004")]
	fn uri_event(&self, new_uri: &[u8], id: &BigUint); // maybe use &str
	*/
}
