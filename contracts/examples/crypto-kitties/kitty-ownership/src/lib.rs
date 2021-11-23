#![no_std]
#![allow(clippy::suspicious_operation_groupings)]

elrond_wasm::imports!();

use kitty::{kitty_genes::*, Kitty};
use random::*;

#[elrond_wasm::contract]
pub trait KittyOwnership {
    #[init]
    fn init(
        &self,
        birth_fee: BigUint,
        #[var_args] opt_gene_science_contract_address: OptionalArg<ManagedAddress>,
        #[var_args] opt_kitty_auction_contract_address: OptionalArg<ManagedAddress>,
    ) {
        self.set_birth_fee(birth_fee);

        match opt_gene_science_contract_address {
            OptionalArg::Some(addr) => self.set_gene_science_contract_address(&addr),
            OptionalArg::None => {},
        };

        match opt_kitty_auction_contract_address {
            OptionalArg::Some(addr) => self.set_kitty_auction_contract_address(&addr),
            OptionalArg::None => {},
        };

        self._create_genesis_kitty();
    }

    // endpoints - owner-only

    #[only_owner]
    #[endpoint(setGeneScienceContractAddress)]
    fn set_gene_science_contract_address_endpoint(&self, address: ManagedAddress) -> SCResult<()> {
        self.set_gene_science_contract_address(&address);

        Ok(())
    }

    #[only_owner]
    #[endpoint(setKittyAuctionContractAddress)]
    fn set_kitty_auction_contract_address_endpoint(&self, address: ManagedAddress) -> SCResult<()> {
        self.set_kitty_auction_contract_address(&address);

        Ok(())
    }

    #[only_owner]
    #[endpoint]
    fn claim(&self) -> SCResult<()> {
        self.send().direct_egld(
            &self.blockchain().get_caller(),
            &self
                .blockchain()
                .get_sc_balance(&TokenIdentifier::egld(), 0),
            b"claim",
        );

        Ok(())
    }

    // views/endpoints - ERC721 required

    #[view(totalSupply)]
    fn total_supply(&self) -> u32 {
        self.get_total_kitties() - 1 // not counting genesis Kitty
    }

    #[view(balanceOf)]
    fn balance_of(&self, address: ManagedAddress) -> u32 {
        self.get_nr_owned_kitties(&address)
    }

    #[view(ownerOf)]
    fn owner_of(&self, kitty_id: u32) -> ManagedAddress {
        if self._is_valid_id(kitty_id) {
            self.get_kitty_owner(kitty_id)
        } else {
            ManagedAddress::zero()
        }
    }

    #[endpoint]
    fn approve(&self, to: ManagedAddress, kitty_id: u32) -> SCResult<()> {
        let caller = self.blockchain().get_caller();

        require!(self._is_valid_id(kitty_id), "Invalid kitty id!");
        require!(
            self.get_kitty_owner(kitty_id) == caller,
            "You are not the owner of that kitty!"
        );

        self.set_approved_address(kitty_id, &to);
        self.approve_event(&caller, &to, kitty_id);

        Ok(())
    }

    #[endpoint]
    fn transfer(&self, to: ManagedAddress, kitty_id: u32) -> SCResult<()> {
        let caller = self.blockchain().get_caller();

        require!(self._is_valid_id(kitty_id), "Invalid kitty id!");
        require!(!to.is_zero(), "Can't transfer to default address 0x0!");
        require!(
            to != self.blockchain().get_sc_address(),
            "Can't transfer to this contract!"
        );
        require!(
            self.get_kitty_owner(kitty_id) == caller,
            "You are not the owner of that kitty!"
        );

        self._transfer(&caller, &to, kitty_id);

        Ok(())
    }

    #[endpoint]
    fn transfer_from(
        &self,
        from: ManagedAddress,
        to: ManagedAddress,
        kitty_id: u32,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();

        require!(self._is_valid_id(kitty_id), "Invalid kitty id!");
        require!(!to.is_zero(), "Can't transfer to default address 0x0!");
        require!(
            to != self.blockchain().get_sc_address(),
            "Can't transfer to this contract!"
        );
        require!(
            self.get_kitty_owner(kitty_id) == from,
            "ManagedAddress _from_ is not the owner!"
        );
        require!(
            self.get_kitty_owner(kitty_id) == caller
                || self._get_approved_address_or_default(kitty_id) == caller,
            "You are not the owner of that kitty nor the approved address!"
        );

        self._transfer(&from, &to, kitty_id);

        Ok(())
    }

    #[view(tokensOfOwner)]
    fn tokens_of_owner(&self, address: ManagedAddress) -> Vec<u32> {
        let nr_owned_kitties = self.get_nr_owned_kitties(&address);
        let total_kitties = self.get_total_kitties();
        let mut kitty_list = Vec::new();

        for kitty_id in 1..total_kitties {
            if nr_owned_kitties as usize == kitty_list.len() {
                break;
            }

            if self.get_kitty_owner(kitty_id) == address {
                kitty_list.push(kitty_id);
            }
        }

        kitty_list
    }

    // endpoints - kitty-auction contract only

    #[endpoint(allowAuctioning)]
    fn allow_auctioning(&self, by: ManagedAddress, kitty_id: u32) -> SCResult<()> {
        let kitty_auction_addr = self._get_kitty_auction_contract_address_or_default();

        require!(
            self.blockchain().get_caller() == kitty_auction_addr,
            "Only auction contract may call this function!"
        );
        require!(self._is_valid_id(kitty_id), "Invalid kitty id!");
        require!(
            by == self.get_kitty_owner(kitty_id)
                || by == self._get_approved_address_or_default(kitty_id),
            "_by_ is not the owner of that kitty nor the approved address!"
        );
        require!(
            !self.get_kitty_by_id(kitty_id).is_pregnant(),
            "Can't auction a pregnant kitty!"
        );

        // transfers ownership to the auction contract
        self._transfer(&by, &kitty_auction_addr, kitty_id);

        Ok(())
    }

    #[endpoint(approveSiringAndReturnKitty)]
    fn approve_siring_and_return_kitty(
        &self,
        approved_address: ManagedAddress,
        kitty_owner: ManagedAddress,
        kitty_id: u32,
    ) -> SCResult<()> {
        let kitty_auction_addr = self._get_kitty_auction_contract_address_or_default();

        require!(
            self.blockchain().get_caller() == kitty_auction_addr,
            "Only auction contract may call this function!"
        );
        require!(self._is_valid_id(kitty_id), "Invalid kitty id!");
        require!(
            kitty_auction_addr == self.get_kitty_owner(kitty_id)
                || kitty_auction_addr == self._get_approved_address_or_default(kitty_id),
            "_by_ is not the owner of that kitty nor the approved address!"
        );

        // return kitty to its original owner after siring auction is complete
        self._transfer(&kitty_auction_addr, &kitty_owner, kitty_id);

        self.set_sire_allowed_address(kitty_id, &approved_address);

        Ok(())
    }

    // create gen zero kitty
    // returns new kitty id
    #[endpoint(createGenZeroKitty)]
    fn create_gen_zero_kitty(&self) -> SCResult<u32> {
        let kitty_auction_addr = self._get_kitty_auction_contract_address_or_default();

        require!(
            self.blockchain().get_caller() == kitty_auction_addr,
            "Only auction contract may call this function!"
        );

        let mut random = Random::new(
            *self.blockchain().get_block_random_seed_legacy(),
            self.blockchain().get_tx_hash_legacy().as_bytes(),
        );
        let genes = KittyGenes::get_random(&mut random);
        let kitty_id = self._create_new_gen_zero_kitty(&genes);

        Ok(kitty_id)
    }

    // views - Kitty Breeding

    #[view(getKittyById)]
    fn get_kitty_by_id_endpoint(&self, kitty_id: u32) -> SCResult<Kitty> {
        if self._is_valid_id(kitty_id) {
            Ok(self.get_kitty_by_id(kitty_id))
        } else {
            sc_error!("kitty does not exist!")
        }
    }

    #[view(isReadyToBreed)]
    fn is_ready_to_breed(&self, kitty_id: u32) -> SCResult<bool> {
        require!(self._is_valid_id(kitty_id), "Invalid kitty id!");

        let kitty = self.get_kitty_by_id(kitty_id);

        Ok(self._is_ready_to_breed(&kitty))
    }

    #[view(isPregnant)]
    fn is_pregnant(&self, kitty_id: u32) -> SCResult<bool> {
        require!(self._is_valid_id(kitty_id), "Invalid kitty id!");

        let kitty = self.get_kitty_by_id(kitty_id);

        Ok(kitty.is_pregnant())
    }

    #[view(canBreedWith)]
    fn can_breed_with(&self, matron_id: u32, sire_id: u32) -> SCResult<bool> {
        require!(self._is_valid_id(matron_id), "Invalid matron id!");
        require!(self._is_valid_id(sire_id), "Invalid sire id!");

        Ok(self._is_valid_mating_pair(matron_id, sire_id)
            && self._is_siring_permitted(matron_id, sire_id))
    }

    // endpoints - Kitty Breeding

    #[endpoint(approveSiring)]
    fn approve_siring(&self, address: ManagedAddress, kitty_id: u32) -> SCResult<()> {
        require!(self._is_valid_id(kitty_id), "Invalid kitty id!");
        require!(
            self.get_kitty_owner(kitty_id) == self.blockchain().get_caller(),
            "You are not the owner of the kitty!"
        );
        require!(
            self._get_sire_allowed_address_or_default(kitty_id)
                .is_zero(),
            "Can't overwrite approved sire address!"
        );

        self.set_sire_allowed_address(kitty_id, &address);

        Ok(())
    }

    #[payable("EGLD")]
    #[endpoint(breedWith)]
    fn breed_with(
        &self,
        #[payment] payment: BigUint,
        matron_id: u32,
        sire_id: u32,
    ) -> SCResult<()> {
        require!(self._is_valid_id(matron_id), "Invalid matron id!");
        require!(self._is_valid_id(sire_id), "Invalid sire id!");

        let auto_birth_fee = self.get_birth_fee();
        let caller = self.blockchain().get_caller();

        require!(payment == auto_birth_fee, "Wrong fee!");
        require!(
            caller == self.get_kitty_owner(matron_id),
            "Only the owner of the matron can call this function!"
        );
        require!(
            self._is_siring_permitted(matron_id, sire_id),
            "Siring not permitted!"
        );

        let matron = self.get_kitty_by_id(matron_id);
        let sire = self.get_kitty_by_id(sire_id);

        require!(
            self._is_ready_to_breed(&matron),
            "Matron not ready to breed!"
        );
        require!(self._is_ready_to_breed(&sire), "Sire not ready to breed!");
        require!(
            self._is_valid_mating_pair(matron_id, sire_id),
            "Not a valid mating pair!"
        );

        self._breed(matron_id, sire_id);

        Ok(())
    }

    #[endpoint(giveBirth)]
    fn give_birth(&self, matron_id: u32) -> SCResult<AsyncCall> {
        require!(self._is_valid_id(matron_id), "Invalid kitty id!");

        let matron = self.get_kitty_by_id(matron_id);

        require!(
            self._is_ready_to_give_birth(&matron),
            "Matron not ready to give birth!"
        );

        let sire_id = matron.siring_with_id;
        let sire = self.get_kitty_by_id(sire_id);

        let gene_science_contract_address = self._get_gene_science_contract_address_or_default();
        if !gene_science_contract_address.is_zero() {
            Ok(self
                .kitty_genetic_alg_proxy(gene_science_contract_address)
                .generate_kitty_genes(matron, sire)
                .async_call()
                .with_callback(
                    self.callbacks()
                        .generate_kitty_genes_callback(matron_id, self.blockchain().get_caller()),
                ))
        } else {
            sc_error!("Gene science contract address not set!")
        }
    }

    // private

    fn _transfer(&self, from: &ManagedAddress, to: &ManagedAddress, kitty_id: u32) {
        if from == to {
            return;
        }

        let mut nr_owned_to = self.get_nr_owned_kitties(to);
        nr_owned_to += 1;

        if !from.is_zero() {
            let mut nr_owned_from = self.get_nr_owned_kitties(from);
            nr_owned_from -= 1;

            self.set_nr_owned_kitties(from, nr_owned_from);
            self.clear_sire_allowed_address(kitty_id);
            self.clear_approved_address(kitty_id);
        }

        self.set_nr_owned_kitties(to, nr_owned_to);
        self.set_kitty_owner(kitty_id, to);

        self.transfer_event(from, to, kitty_id);
    }

    // checks should be done in the caller function
    // returns the newly created kitten id
    fn _create_new_kitty(
        &self,
        matron_id: u32,
        sire_id: u32,
        generation: u16,
        genes: &KittyGenes,
        owner: &ManagedAddress,
    ) -> u32 {
        let mut total_kitties = self.get_total_kitties();
        let new_kitty_id = total_kitties;
        let kitty = Kitty::new(
            genes,
            self.blockchain().get_block_timestamp(),
            matron_id,
            sire_id,
            generation,
        );

        total_kitties += 1;
        self.set_total_kitties(total_kitties);
        self.set_kitty_at_id(new_kitty_id, &kitty);

        self._transfer(&ManagedAddress::zero(), owner, new_kitty_id);

        new_kitty_id
    }

    fn _create_new_gen_zero_kitty(&self, genes: &KittyGenes) -> u32 {
        self._create_new_kitty(0, 0, 0, genes, &self.get_kitty_auction_contract_address())
    }

    fn _create_genesis_kitty(&self) {
        let genesis_kitty = Kitty::default();

        self._create_new_kitty(
            genesis_kitty.matron_id,
            genesis_kitty.sire_id,
            genesis_kitty.generation,
            &genesis_kitty.genes,
            &ManagedAddress::zero(),
        );
    }

    fn _trigger_cooldown(&self, kitty: &mut Kitty) {
        let cooldown = kitty.get_next_cooldown_time();
        kitty.cooldown_end = self.blockchain().get_block_timestamp() + cooldown;
    }

    fn _breed(&self, matron_id: u32, sire_id: u32) {
        let mut matron = self.get_kitty_by_id(matron_id);
        let mut sire = self.get_kitty_by_id(sire_id);

        // mark matron as pregnant
        matron.siring_with_id = sire_id;

        self._trigger_cooldown(&mut matron);
        self._trigger_cooldown(&mut sire);

        self.clear_sire_allowed_address(matron_id);
        self.clear_sire_allowed_address(sire_id);

        self.set_kitty_at_id(matron_id, &matron);
        self.set_kitty_at_id(sire_id, &sire);
    }

    // private - Kitty checks. These should be in the Kitty struct,
    // but unfortunately, they need access to the contract-only functions

    fn _is_valid_id(&self, kitty_id: u32) -> bool {
        kitty_id != 0 && kitty_id < self.get_total_kitties()
    }

    fn _is_ready_to_breed(&self, kitty: &Kitty) -> bool {
        kitty.siring_with_id == 0 && kitty.cooldown_end < self.blockchain().get_block_timestamp()
    }

    fn _is_siring_permitted(&self, matron_id: u32, sire_id: u32) -> bool {
        let sire_owner = self.get_kitty_owner(sire_id);
        let matron_owner = self.get_kitty_owner(matron_id);
        let sire_approved_address = self._get_sire_allowed_address_or_default(sire_id);

        sire_owner == matron_owner || matron_owner == sire_approved_address
    }

    fn _is_ready_to_give_birth(&self, matron: &Kitty) -> bool {
        matron.siring_with_id != 0 && matron.cooldown_end < self.blockchain().get_block_timestamp()
    }

    fn _is_valid_mating_pair(&self, matron_id: u32, sire_id: u32) -> bool {
        let matron = self.get_kitty_by_id(matron_id);
        let sire = self.get_kitty_by_id(sire_id);

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

    fn _get_gene_science_contract_address_or_default(&self) -> ManagedAddress {
        if self.is_empty_gene_science_contract_address() {
            ManagedAddress::zero()
        } else {
            self.get_gene_science_contract_address()
        }
    }

    fn _get_kitty_auction_contract_address_or_default(&self) -> ManagedAddress {
        if self.is_empty_kitty_auction_contract_address() {
            ManagedAddress::zero()
        } else {
            self.get_kitty_auction_contract_address()
        }
    }

    fn _get_approved_address_or_default(&self, kitty_id: u32) -> ManagedAddress {
        if self.is_empty_approved_address(kitty_id) {
            ManagedAddress::zero()
        } else {
            self.get_approved_address(kitty_id)
        }
    }

    fn _get_sire_allowed_address_or_default(&self, kitty_id: u32) -> ManagedAddress {
        if self.is_empty_sire_allowed_address(kitty_id) {
            ManagedAddress::zero()
        } else {
            self.get_sire_allowed_address(kitty_id)
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
                let mut matron = self.get_kitty_by_id(matron_id);
                let sire_id = matron.siring_with_id;
                let mut sire = self.get_kitty_by_id(sire_id);

                let new_kitty_generation: u16; // MAX(gen_matron, gen_sire) + 1
                if matron.generation > sire.generation {
                    new_kitty_generation = matron.generation + 1;
                } else {
                    new_kitty_generation = sire.generation + 1;
                }

                // new kitty goes to the owner of the matron
                let new_kitty_owner = self.get_kitty_owner(matron_id);
                let _new_kitty_id = self._create_new_kitty(
                    matron_id,
                    sire_id,
                    new_kitty_generation,
                    &genes,
                    &new_kitty_owner,
                );

                // update matron kitty
                matron.siring_with_id = 0;
                matron.nr_children += 1;
                self.set_kitty_at_id(matron_id, &matron);

                // update sire kitty
                sire.nr_children += 1;
                self.set_kitty_at_id(sire_id, &sire);

                // send birth fee to caller
                let fee = self.get_birth_fee();
                self.send()
                    .direct_egld(&original_caller, &fee, b"birth fee");
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

    #[storage_get("geneScienceContractAddress")]
    fn get_gene_science_contract_address(&self) -> ManagedAddress;

    #[storage_set("geneScienceContractAddress")]
    fn set_gene_science_contract_address(&self, address: &ManagedAddress);

    #[storage_is_empty("geneScienceContractAddress")]
    fn is_empty_gene_science_contract_address(&self) -> bool;

    #[storage_get("kittyAuctionContractAddress")]
    fn get_kitty_auction_contract_address(&self) -> ManagedAddress;

    #[storage_set("kittyAuctionContractAddress")]
    fn set_kitty_auction_contract_address(&self, address: &ManagedAddress);

    #[storage_is_empty("kittyAuctionContractAddress")]
    fn is_empty_kitty_auction_contract_address(&self) -> bool;

    #[view(birthFee)]
    #[storage_get("birthFee")]
    fn get_birth_fee(&self) -> BigUint;

    #[storage_set("birthFee")]
    fn set_birth_fee(&self, fee: BigUint);

    // storage - Kitties

    #[storage_get("totalKitties")]
    fn get_total_kitties(&self) -> u32;

    #[storage_set("totalKitties")]
    fn set_total_kitties(&self, total_kitties: u32);

    #[storage_get("kitty")]
    fn get_kitty_by_id(&self, kitty_id: u32) -> Kitty;

    #[storage_set("kitty")]
    fn set_kitty_at_id(&self, kitty_id: u32, kitty: &Kitty);

    #[storage_get("owner")]
    fn get_kitty_owner(&self, kitty_id: u32) -> ManagedAddress;

    #[storage_set("owner")]
    fn set_kitty_owner(&self, kitty_id: u32, owner: &ManagedAddress);

    #[storage_get("nrOwnedKitties")]
    fn get_nr_owned_kitties(&self, address: &ManagedAddress) -> u32;

    #[storage_set("nrOwnedKitties")]
    fn set_nr_owned_kitties(&self, address: &ManagedAddress, nr_owned: u32);

    #[storage_get("approvedAddress")]
    fn get_approved_address(&self, kitty_id: u32) -> ManagedAddress;

    #[storage_set("approvedAddress")]
    fn set_approved_address(&self, kitty_id: u32, address: &ManagedAddress);

    #[storage_clear("approvedAddress")]
    fn clear_approved_address(&self, kitty_id: u32);

    #[storage_is_empty("approvedAddress")]
    fn is_empty_approved_address(&self, kitty_id: u32) -> bool;

    #[storage_get("sireAllowedAddress")]
    fn get_sire_allowed_address(&self, kitty_id: u32) -> ManagedAddress;

    #[storage_set("sireAllowedAddress")]
    fn set_sire_allowed_address(&self, kitty_id: u32, address: &ManagedAddress);

    #[storage_clear("sireAllowedAddress")]
    fn clear_sire_allowed_address(&self, kitty_id: u32);

    #[storage_is_empty("sireAllowedAddress")]
    fn is_empty_sire_allowed_address(&self, kitty_id: u32) -> bool;

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
