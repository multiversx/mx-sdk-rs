#![no_std]
#![allow(clippy::suspicious_operation_groupings)]

multiversx_sc::imports!();

use core::cmp::max;

use kitty::{kitty_genes::*, Kitty};
use random::*;

#[multiversx_sc::contract]
pub trait KittyOwnership {
    #[init]
    fn init(
        &self,
        birth_fee: BigUint,
        opt_gene_science_contract_address: OptionalValue<ManagedAddress>,
        opt_kitty_auction_contract_address: OptionalValue<ManagedAddress>,
    ) {
        self.birth_fee().set(birth_fee);

        if let OptionalValue::Some(addr) = opt_gene_science_contract_address {
            self.gene_science_contract_address().set(&addr);
        }
        if let OptionalValue::Some(addr) = opt_kitty_auction_contract_address {
            self.kitty_auction_contract_address().set(&addr);
        }

        self.create_genesis_kitty();
    }

    // endpoints - owner-only

    #[only_owner]
    #[endpoint(setGeneScienceContractAddress)]
    fn set_gene_science_contract_address_endpoint(&self, address: ManagedAddress) {
        self.gene_science_contract_address().set(&address);
    }

    #[only_owner]
    #[endpoint(setKittyAuctionContractAddress)]
    fn set_kitty_auction_contract_address_endpoint(&self, address: ManagedAddress) {
        self.kitty_auction_contract_address().set(&address);
    }

    #[only_owner]
    #[endpoint]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        let egld_balance = self
            .blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0);

        self.send().direct_egld(&caller, &egld_balance);
    }

    // views/endpoints - ERC721 required

    #[view(totalSupply)]
    fn total_supply(&self) -> u32 {
        self.total_kitties().get() - 1 // not counting genesis Kitty
    }

    #[view(balanceOf)]
    fn balance_of(&self, address: ManagedAddress) -> u32 {
        self.nr_owned_kitties(&address).get()
    }

    #[view(ownerOf)]
    fn owner_of(&self, kitty_id: u32) -> ManagedAddress {
        if self.is_valid_id(kitty_id) {
            self.kitty_owner(kitty_id).get()
        } else {
            ManagedAddress::zero()
        }
    }

    #[endpoint]
    fn approve(&self, to: ManagedAddress, kitty_id: u32) {
        let caller = self.blockchain().get_caller();

        require!(self.is_valid_id(kitty_id), "Invalid kitty id!");
        require!(
            self.kitty_owner(kitty_id).get() == caller,
            "You are not the owner of that kitty!"
        );

        self.approved_address(kitty_id).set(&to);
        self.approve_event(&caller, &to, kitty_id);
    }

    #[endpoint]
    fn transfer(&self, to: ManagedAddress, kitty_id: u32) {
        let caller = self.blockchain().get_caller();

        require!(self.is_valid_id(kitty_id), "Invalid kitty id!");
        require!(!to.is_zero(), "Can't transfer to default address 0x0!");
        require!(
            to != self.blockchain().get_sc_address(),
            "Can't transfer to this contract!"
        );
        require!(
            self.kitty_owner(kitty_id).get() == caller,
            "You are not the owner of that kitty!"
        );

        self.perform_transfer(&caller, &to, kitty_id);
    }

    #[endpoint]
    fn transfer_from(&self, from: ManagedAddress, to: ManagedAddress, kitty_id: u32) {
        let caller = self.blockchain().get_caller();

        require!(self.is_valid_id(kitty_id), "Invalid kitty id!");
        require!(!to.is_zero(), "Can't transfer to default address 0x0!");
        require!(
            to != self.blockchain().get_sc_address(),
            "Can't transfer to this contract!"
        );
        require!(
            self.kitty_owner(kitty_id).get() == from,
            "ManagedAddress _from_ is not the owner!"
        );
        require!(
            self.kitty_owner(kitty_id).get() == caller
                || self.get_approved_address_or_default(kitty_id) == caller,
            "You are not the owner of that kitty nor the approved address!"
        );

        self.perform_transfer(&from, &to, kitty_id);
    }

    #[view(tokensOfOwner)]
    fn tokens_of_owner(&self, address: ManagedAddress) -> MultiValueEncoded<u32> {
        let nr_owned_kitties = self.nr_owned_kitties(&address).get();
        let total_kitties = self.total_kitties().get();
        let mut kitty_list = ManagedVec::new();
        let mut list_len = 0; // more efficient than calling the API over and over

        for kitty_id in 1..total_kitties {
            if nr_owned_kitties as usize == list_len {
                break;
            }

            if self.kitty_owner(kitty_id).get() == address {
                kitty_list.push(kitty_id);
                list_len += 1;
            }
        }

        kitty_list.into()
    }

    // endpoints - kitty-auction contract only

    #[endpoint(allowAuctioning)]
    fn allow_auctioning(&self, by: ManagedAddress, kitty_id: u32) {
        let kitty_auction_addr = self.get_kitty_auction_contract_address_or_default();

        require!(
            self.blockchain().get_caller() == kitty_auction_addr,
            "Only auction contract may call this function!"
        );
        require!(self.is_valid_id(kitty_id), "Invalid kitty id!");
        require!(
            by == self.kitty_owner(kitty_id).get()
                || by == self.get_approved_address_or_default(kitty_id),
            "{:x} is not the owner of that kitty nor the approved address!",
            by
        );
        require!(
            !self.kitty_by_id(kitty_id).get().is_pregnant(),
            "Can't auction a pregnant kitty!"
        );

        // transfers ownership to the auction contract
        self.perform_transfer(&by, &kitty_auction_addr, kitty_id);
    }

    #[endpoint(approveSiringAndReturnKitty)]
    fn approve_siring_and_return_kitty(
        &self,
        approved_address: ManagedAddress,
        kitty_owner: ManagedAddress,
        kitty_id: u32,
    ) {
        let kitty_auction_addr = self.get_kitty_auction_contract_address_or_default();

        require!(
            self.blockchain().get_caller() == kitty_auction_addr,
            "Only auction contract may call this function!"
        );
        require!(self.is_valid_id(kitty_id), "Invalid kitty id!");
        require!(
            kitty_auction_addr == self.kitty_owner(kitty_id).get()
                || kitty_auction_addr == self.get_approved_address_or_default(kitty_id),
            "{:x} is not the owner of that kitty nor the approved address!",
            kitty_auction_addr
        );

        // return kitty to its original owner after siring auction is complete
        self.perform_transfer(&kitty_auction_addr, &kitty_owner, kitty_id);

        self.sire_allowed_address(kitty_id).set(&approved_address);
    }

    // create gen zero kitty
    // returns new kitty id
    #[endpoint(createGenZeroKitty)]
    fn create_gen_zero_kitty(&self) -> u32 {
        let kitty_auction_addr = self.get_kitty_auction_contract_address_or_default();

        require!(
            self.blockchain().get_caller() == kitty_auction_addr,
            "Only auction contract may call this function!"
        );

        let mut random = Random::new(
            self.blockchain().get_block_random_seed(),
            self.blockchain().get_tx_hash(),
        );
        let genes = KittyGenes::get_random(&mut random);

        self.create_new_gen_zero_kitty(genes)
    }

    // views - Kitty Breeding

    #[view(getKittyById)]
    fn get_kitty_by_id_endpoint(&self, kitty_id: u32) -> Kitty {
        if self.is_valid_id(kitty_id) {
            self.kitty_by_id(kitty_id).get()
        } else {
            sc_panic!("kitty does not exist!")
        }
    }

    #[view(isReadyToBreed)]
    fn is_ready_to_breed(&self, kitty_id: u32) -> bool {
        require!(self.is_valid_id(kitty_id), "Invalid kitty id!");

        let kitty = self.kitty_by_id(kitty_id).get();

        self.is_kitty_ready_to_breed(&kitty)
    }

    #[view(isPregnant)]
    fn is_pregnant(&self, kitty_id: u32) -> bool {
        require!(self.is_valid_id(kitty_id), "Invalid kitty id!");

        let kitty = self.kitty_by_id(kitty_id).get();

        kitty.is_pregnant()
    }

    #[view(canBreedWith)]
    fn can_breed_with(&self, matron_id: u32, sire_id: u32) -> bool {
        require!(self.is_valid_id(matron_id), "Invalid matron id!");
        require!(self.is_valid_id(sire_id), "Invalid sire id!");

        self.is_valid_mating_pair(matron_id, sire_id)
            && self.is_siring_permitted(matron_id, sire_id)
    }

    // endpoints - Kitty Breeding

    #[endpoint(approveSiring)]
    fn approve_siring(&self, address: ManagedAddress, kitty_id: u32) {
        require!(self.is_valid_id(kitty_id), "Invalid kitty id!");
        require!(
            self.kitty_owner(kitty_id).get() == self.blockchain().get_caller(),
            "You are not the owner of the kitty!"
        );
        require!(
            self.get_sire_allowed_address_or_default(kitty_id).is_zero(),
            "Can't overwrite approved sire address!"
        );

        self.sire_allowed_address(kitty_id).set(&address);
    }

    #[payable("EGLD")]
    #[endpoint(breedWith)]
    fn breed_with(&self, matron_id: u32, sire_id: u32) {
        require!(self.is_valid_id(matron_id), "Invalid matron id!");
        require!(self.is_valid_id(sire_id), "Invalid sire id!");

        let payment = self.call_value().egld_value();
        let auto_birth_fee = self.birth_fee().get();
        let caller = self.blockchain().get_caller();

        require!(*payment == auto_birth_fee, "Wrong fee!");
        require!(
            caller == self.kitty_owner(matron_id).get(),
            "Only the owner of the matron can call this function!"
        );
        require!(
            self.is_siring_permitted(matron_id, sire_id),
            "Siring not permitted!"
        );

        let matron = self.kitty_by_id(matron_id).get();
        let sire = self.kitty_by_id(sire_id).get();

        require!(
            self.is_kitty_ready_to_breed(&matron),
            "Matron not ready to breed!"
        );
        require!(
            self.is_kitty_ready_to_breed(&sire),
            "Sire not ready to breed!"
        );
        require!(
            self.is_valid_mating_pair(matron_id, sire_id),
            "Not a valid mating pair!"
        );

        self.breed(matron_id, sire_id);
    }

    #[endpoint(giveBirth)]
    fn give_birth(&self, matron_id: u32) {
        require!(self.is_valid_id(matron_id), "Invalid kitty id!");

        let matron = self.kitty_by_id(matron_id).get();

        require!(
            self.is_ready_to_give_birth(&matron),
            "Matron not ready to give birth!"
        );

        let sire_id = matron.siring_with_id;
        let sire = self.kitty_by_id(sire_id).get();

        let gene_science_contract_address = self.get_gene_science_contract_address_or_default();
        if !gene_science_contract_address.is_zero() {
            self.kitty_genetic_alg_proxy(gene_science_contract_address)
                .generate_kitty_genes(matron, sire)
                .async_call()
                .with_callback(
                    self.callbacks()
                        .generate_kitty_genes_callback(matron_id, self.blockchain().get_caller()),
                )
                .call_and_exit()
        } else {
            sc_panic!("Gene science contract address not set!")
        }
    }

    // private

    fn perform_transfer(&self, from: &ManagedAddress, to: &ManagedAddress, kitty_id: u32) {
        if from == to {
            return;
        }

        let mut nr_owned_to = self.nr_owned_kitties(to).get();
        nr_owned_to += 1;

        if !from.is_zero() {
            let mut nr_owned_from = self.nr_owned_kitties(from).get();
            nr_owned_from -= 1;

            self.nr_owned_kitties(from).set(nr_owned_from);
            self.sire_allowed_address(kitty_id).clear();
            self.approved_address(kitty_id).clear();
        }

        self.nr_owned_kitties(to).set(nr_owned_to);
        self.kitty_owner(kitty_id).set(to);

        self.transfer_event(from, to, kitty_id);
    }

    // checks should be done in the caller function
    // returns the newly created kitten id
    fn create_new_kitty(
        &self,
        matron_id: u32,
        sire_id: u32,
        generation: u16,
        genes: KittyGenes,
        owner: &ManagedAddress,
    ) -> u32 {
        let mut total_kitties = self.total_kitties().get();
        let new_kitty_id = total_kitties;
        let kitty = Kitty::new(
            genes,
            self.blockchain().get_block_timestamp(),
            matron_id,
            sire_id,
            generation,
        );

        total_kitties += 1;
        self.total_kitties().set(total_kitties);
        self.kitty_by_id(new_kitty_id).set(kitty);

        self.perform_transfer(&ManagedAddress::zero(), owner, new_kitty_id);

        new_kitty_id
    }

    fn create_new_gen_zero_kitty(&self, genes: KittyGenes) -> u32 {
        let kitty_auction_addr = self.kitty_auction_contract_address().get();
        self.create_new_kitty(0, 0, 0, genes, &kitty_auction_addr)
    }

    fn create_genesis_kitty(&self) {
        let genesis_kitty = Kitty::default();

        self.create_new_kitty(
            genesis_kitty.matron_id,
            genesis_kitty.sire_id,
            genesis_kitty.generation,
            genesis_kitty.genes,
            &ManagedAddress::zero(),
        );
    }

    fn trigger_cooldown(&self, kitty: &mut Kitty) {
        let cooldown = kitty.get_next_cooldown_time();
        kitty.cooldown_end = self.blockchain().get_block_timestamp() + cooldown;
    }

    fn breed(&self, matron_id: u32, sire_id: u32) {
        let mut matron = self.kitty_by_id(matron_id).get();
        let mut sire = self.kitty_by_id(sire_id).get();

        // mark matron as pregnant
        matron.siring_with_id = sire_id;

        self.trigger_cooldown(&mut matron);
        self.trigger_cooldown(&mut sire);

        self.sire_allowed_address(matron_id).clear();
        self.sire_allowed_address(sire_id).clear();

        self.kitty_by_id(matron_id).set(&matron);
        self.kitty_by_id(sire_id).set(&sire);
    }

    // private - Kitty checks. These should be in the Kitty struct,
    // but unfortunately, they need access to the contract-only functions

    fn is_valid_id(&self, kitty_id: u32) -> bool {
        kitty_id != 0 && kitty_id < self.total_kitties().get()
    }

    fn is_kitty_ready_to_breed(&self, kitty: &Kitty) -> bool {
        kitty.siring_with_id == 0 && kitty.cooldown_end < self.blockchain().get_block_timestamp()
    }

    fn is_siring_permitted(&self, matron_id: u32, sire_id: u32) -> bool {
        let sire_owner = self.kitty_owner(sire_id).get();
        let matron_owner = self.kitty_owner(matron_id).get();
        let sire_approved_address = self.get_sire_allowed_address_or_default(sire_id);

        sire_owner == matron_owner || matron_owner == sire_approved_address
    }

    fn is_ready_to_give_birth(&self, matron: &Kitty) -> bool {
        matron.siring_with_id != 0 && matron.cooldown_end < self.blockchain().get_block_timestamp()
    }

    fn is_valid_mating_pair(&self, matron_id: u32, sire_id: u32) -> bool {
        let matron = self.kitty_by_id(matron_id).get();
        let sire = self.kitty_by_id(sire_id).get();

        // can't breed with itself
        if matron_id == sire_id {
            return false;
        }

        // can't breed with their parents
        if matron.matron_id == sire_id || matron.sire_id == sire_id {
            return false;
        }
        if sire.matron_id == matron_id || sire.sire_id == matron_id {
            return false;
        }

        // for gen zero kitties
        if sire.matron_id == 0 || matron.matron_id == 0 {
            return true;
        }

        // can't breed with full or half siblings
        if sire.matron_id == matron.matron_id || sire.matron_id == matron.sire_id {
            return false;
        }
        if sire.sire_id == matron.matron_id || sire.sire_id == matron.sire_id {
            return false;
        }

        true
    }

    // getters

    fn get_gene_science_contract_address_or_default(&self) -> ManagedAddress {
        if self.gene_science_contract_address().is_empty() {
            ManagedAddress::zero()
        } else {
            self.gene_science_contract_address().get()
        }
    }

    fn get_kitty_auction_contract_address_or_default(&self) -> ManagedAddress {
        if self.kitty_auction_contract_address().is_empty() {
            ManagedAddress::zero()
        } else {
            self.kitty_auction_contract_address().get()
        }
    }

    fn get_approved_address_or_default(&self, kitty_id: u32) -> ManagedAddress {
        if self.approved_address(kitty_id).is_empty() {
            ManagedAddress::zero()
        } else {
            self.approved_address(kitty_id).get()
        }
    }

    fn get_sire_allowed_address_or_default(&self, kitty_id: u32) -> ManagedAddress {
        if self.sire_allowed_address(kitty_id).is_empty() {
            ManagedAddress::zero()
        } else {
            self.sire_allowed_address(kitty_id).get()
        }
    }

    // callbacks

    #[callback]
    fn generate_kitty_genes_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<KittyGenes>,
        matron_id: u32,
        original_caller: ManagedAddress,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(genes) => {
                let mut matron = self.kitty_by_id(matron_id).get();
                let sire_id = matron.siring_with_id;
                let mut sire = self.kitty_by_id(sire_id).get();

                let new_kitty_generation = max(matron.generation, sire.generation) + 1;

                // new kitty goes to the owner of the matron
                let new_kitty_owner = self.kitty_owner(matron_id).get();
                let _new_kitty_id = self.create_new_kitty(
                    matron_id,
                    sire_id,
                    new_kitty_generation,
                    genes,
                    &new_kitty_owner,
                );

                // update matron kitty
                matron.siring_with_id = 0;
                matron.nr_children += 1;
                self.kitty_by_id(matron_id).set(&matron);

                // update sire kitty
                sire.nr_children += 1;
                self.kitty_by_id(sire_id).set(&sire);

                // send birth fee to caller
                let fee = self.birth_fee().get();
                self.send().direct_egld(&original_caller, &fee);
            },
            ManagedAsyncCallResult::Err(_) => {
                // this can only fail if the kitty_genes contract address is invalid
                // in which case, the only thing we can do is call this again later
            },
        }
    }

    // proxy

    #[proxy]
    fn kitty_genetic_alg_proxy(&self, to: ManagedAddress) -> kitty_genetic_alg::Proxy<Self::Api>;

    // storage - General

    #[storage_mapper("geneScienceContractAddress")]
    fn gene_science_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("kittyAuctionContractAddress")]
    fn kitty_auction_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(birthFee)]
    #[storage_mapper("birthFee")]
    fn birth_fee(&self) -> SingleValueMapper<BigUint>;

    // storage - Kitties

    #[storage_mapper("totalKitties")]
    fn total_kitties(&self) -> SingleValueMapper<u32>;

    #[storage_mapper("kitty")]
    fn kitty_by_id(&self, kitty_id: u32) -> SingleValueMapper<Kitty>;

    #[storage_mapper("owner")]
    fn kitty_owner(&self, kitty_id: u32) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("nrOwnedKitties")]
    fn nr_owned_kitties(&self, address: &ManagedAddress) -> SingleValueMapper<u32>;

    #[storage_mapper("approvedAddress")]
    fn approved_address(&self, kitty_id: u32) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("sireAllowedAddress")]
    fn sire_allowed_address(&self, kitty_id: u32) -> SingleValueMapper<ManagedAddress>;

    // events

    #[event("transfer")]
    fn transfer_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] to: &ManagedAddress,
        #[indexed] token_id: u32,
    );

    #[event("approve")]
    fn approve_event(
        &self,
        #[indexed] owner: &ManagedAddress,
        #[indexed] approved: &ManagedAddress,
        #[indexed] token_id: u32,
    );
}
