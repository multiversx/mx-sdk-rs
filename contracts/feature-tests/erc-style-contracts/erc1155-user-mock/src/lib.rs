#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Erc1155UserMock {
    #[init]
    fn init(&self) {}

    #[endpoint(onERC1155Received)]
    fn on_erc1155_received(
        &self,
        _operator: ManagedAddress,
        _from: ManagedAddress,
        _type_id: BaseBigUint,
        _value: BaseBigUint,
        _data: ManagedBuffer,
    ) {
    }

    #[endpoint(onERC1155BatchReceived)]
    fn on_erc1155_batch_received(
        &self,
        _operator: ManagedAddress,
        _from: ManagedAddress,
        _type_ids: ManagedVec<BaseBigUint>,
        _values: ManagedVec<BaseBigUint>,
        _data: ManagedBuffer,
    ) {
    }
}
