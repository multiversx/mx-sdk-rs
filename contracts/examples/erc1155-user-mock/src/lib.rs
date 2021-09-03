#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Erc1155UserMock {
    #[init]
    fn init(&self) {}

    #[endpoint(onERC1155Received)]
    fn on_erc1155_received(
        &self,
        _operator: ManagedAddress,
        _from: ManagedAddress,
        _type_id: BigUint,
        _value: BigUint,
        _data: &[u8],
    ) -> SCResult<()> {
        Ok(())
    }

    #[endpoint(onERC1155BatchReceived)]
    fn on_erc1155_batch_received(
        &self,
        _operator: ManagedAddress,
        _from: ManagedAddress,
        _type_ids: Vec<BigUint>,
        _values: Vec<BigUint>,
        _data: &[u8],
    ) -> SCResult<()> {
        Ok(())
    }
}
