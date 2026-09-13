#![allow(dead_code)]

use crate::Kitty;
use multiversx_sc::abi::imports::*;

/// Framework-agnostic ABI description of the `kitty-ownership` contract's interface. Mirrors
/// `kitty_ownership_proxy.rs`, but written directly against pure ABI types, with no dependency
/// on `multiversx-sc`/`VMApi`.
#[multiversx_sc_abi::contract_abi(call = KittyOwnershipCall)]
pub trait KittyOwnershipAbi {
    #[allow_multiple_var_args]
    #[init]
    fn init(
        &self,
        birth_fee: BigUintAbi,
        opt_gene_science_contract_address: OptionalValue<Address>,
        opt_kitty_auction_contract_address: OptionalValue<Address>,
    );

    #[endpoint(setGeneScienceContractAddress)]
    fn set_gene_science_contract_address_endpoint(&self, address: Address);

    #[endpoint(setKittyAuctionContractAddress)]
    fn set_kitty_auction_contract_address_endpoint(&self, address: Address);

    #[endpoint]
    fn claim(&self);

    #[view(totalSupply)]
    fn total_supply(&self) -> u32;

    #[view(balanceOf)]
    fn balance_of(&self, address: Address) -> u32;

    #[view(ownerOf)]
    fn owner_of(&self, kitty_id: u32) -> Address;

    #[endpoint]
    fn approve(&self, to: Address, kitty_id: u32);

    #[endpoint]
    fn transfer(&self, to: Address, kitty_id: u32);

    #[endpoint]
    fn transfer_from(&self, from: Address, to: Address, kitty_id: u32);

    #[view(tokensOfOwner)]
    fn tokens_of_owner(&self, address: Address) -> MultiValueVec<u32>;

    #[endpoint(allowAuctioning)]
    fn allow_auctioning(&self, by: Address, kitty_id: u32);

    #[endpoint(approveSiringAndReturnKitty)]
    fn approve_siring_and_return_kitty(
        &self,
        approved_address: Address,
        kitty_owner: Address,
        kitty_id: u32,
    );

    #[endpoint(createGenZeroKitty)]
    fn create_gen_zero_kitty(&self) -> u32;

    #[view(getKittyById)]
    fn get_kitty_by_id_endpoint(&self, kitty_id: u32) -> Kitty;

    #[view(isReadyToBreed)]
    fn is_ready_to_breed(&self, kitty_id: u32) -> bool;

    #[view(isPregnant)]
    fn is_pregnant(&self, kitty_id: u32) -> bool;

    #[view(canBreedWith)]
    fn can_breed_with(&self, matron_id: u32, sire_id: u32) -> bool;

    #[endpoint(approveSiring)]
    fn approve_siring(&self, address: Address, kitty_id: u32);

    #[payable("EGLD")]
    #[endpoint(breedWith)]
    fn breed_with(&self, matron_id: u32, sire_id: u32);

    #[endpoint(giveBirth)]
    fn give_birth(&self, matron_id: u32);

    #[view(birthFee)]
    fn birth_fee(&self) -> BigUintAbi;
}
