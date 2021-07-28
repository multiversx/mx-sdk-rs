#![no_std]

elrond_wasm::imports!();

pub mod auction;
use auction::*;

#[elrond_wasm::contract]
pub trait KittyAuction {
    #[init]
    fn init(
        &self,
        gen_zero_kitty_starting_price: Self::BigUint,
        gen_zero_kitty_ending_price: Self::BigUint,
        gen_zero_kitty_auction_duration: u64,
        #[var_args] opt_kitty_ownership_contract_address: OptionalArg<Address>,
    ) {
        self.gen_zero_kitty_starting_price()
            .set(&gen_zero_kitty_starting_price);
        self.gen_zero_kitty_ending_price()
            .set(&gen_zero_kitty_ending_price);
        self.gen_zero_kitty_auction_duration()
            .set(&gen_zero_kitty_auction_duration);

        match opt_kitty_ownership_contract_address {
            OptionalArg::Some(addr) => self.kitty_ownership_contract_address().set(&addr),
            OptionalArg::None => {},
        }
    }

    // endpoints - owner-only

    #[only_owner]
    #[endpoint(setKittyOwnershipContractAddress)]
    fn set_kitty_ownership_contract_address_endpoint(&self, address: Address) -> SCResult<()> {
        self.kitty_ownership_contract_address().set(&address);

        Ok(())
    }

    #[only_owner]
    #[endpoint(createAndAuctionGenZeroKitty)]
    fn create_and_auction_gen_zero_kitty(&self) -> SCResult<AsyncCall<Self::SendApi>> {
        let kitty_ownership_contract_address =
            self.get_kitty_ownership_contract_address_or_default();
        if kitty_ownership_contract_address != Address::zero() {
            Ok(self
                .kitty_ownership_proxy(kitty_ownership_contract_address)
                .create_gen_zero_kitty()
                .async_call()
                .with_callback(self.callbacks().create_gen_zero_kitty_callback()))
        } else {
            sc_error!("Kitty Ownership contract address not set!")
        }
    }

    // views

    #[view(isUpForAuction)]
    fn is_up_for_auction(&self, kitty_id: u32) -> bool {
        !self.auction(kitty_id).is_empty()
    }

    #[view(getAuctionStatus)]
    fn get_auction_status(&self, kitty_id: u32) -> SCResult<Auction<Self::BigUint>> {
        require!(
            self.is_up_for_auction(kitty_id),
            "Kitty is not up for auction!"
        );

        Ok(self.auction(kitty_id).get())
    }

    #[view(getCurrentWinningBid)]
    fn get_current_winning_bid(&self, kitty_id: u32) -> SCResult<Self::BigUint> {
        require!(
            self.is_up_for_auction(kitty_id),
            "Kitty is not up for auction!"
        );

        Ok(self.auction(kitty_id).get().current_bid)
    }

    // endpoints

    #[endpoint(createSaleAuction)]
    fn create_sale_auction(
        &self,
        kitty_id: u32,
        starting_price: Self::BigUint,
        ending_price: Self::BigUint,
        duration: u64,
    ) -> SCResult<OptionalResult<AsyncCall<Self::SendApi>>> {
        let deadline = self.blockchain().get_block_timestamp() + duration;

        require!(
            !self.is_up_for_auction(kitty_id),
            "kitty already auctioned!"
        );
        require!(starting_price > 0, "starting price must be higher than 0!");
        require!(
            starting_price < ending_price,
            "starting price must be less than ending price!"
        );
        require!(
            deadline > self.blockchain().get_block_timestamp(),
            "deadline can't be in the past!"
        );

        Ok(self.create_auction(
            AuctionType::Selling,
            kitty_id,
            starting_price,
            ending_price,
            deadline,
        ))
    }

    #[endpoint(createSiringAuction)]
    fn create_siring_auction(
        &self,
        kitty_id: u32,
        starting_price: Self::BigUint,
        ending_price: Self::BigUint,
        duration: u64,
    ) -> SCResult<OptionalResult<AsyncCall<Self::SendApi>>> {
        let deadline = self.blockchain().get_block_timestamp() + duration;

        require!(
            !self.is_up_for_auction(kitty_id),
            "kitty already auctioned!"
        );
        require!(starting_price > 0, "starting price must be higher than 0!");
        require!(
            starting_price < ending_price,
            "starting price must be less than ending price!"
        );
        require!(
            deadline > self.blockchain().get_block_timestamp(),
            "deadline can't be in the past!"
        );

        Ok(self.create_auction(
            AuctionType::Siring,
            kitty_id,
            starting_price,
            ending_price,
            deadline,
        ))
    }

    #[payable("EGLD")]
    #[endpoint]
    fn bid(&self, kitty_id: u32, #[payment] payment: Self::BigUint) -> SCResult<()> {
        require!(
            self.is_up_for_auction(kitty_id),
            "Kitty is not up for auction!"
        );

        let caller = self.blockchain().get_caller();
        let mut auction = self.auction(kitty_id).get();

        require!(
            caller != auction.kitty_owner,
            "can't bid on your own kitty!"
        );
        require!(
            self.blockchain().get_block_timestamp() < auction.deadline,
            "auction ended already!"
        );
        require!(
            payment >= auction.starting_price,
            "bid amount must be higher than or equal to starting price!"
        );
        require!(
            payment > auction.current_bid,
            "bid amount must be higher than current winning bid!"
        );
        require!(
            payment <= auction.ending_price,
            "bid amount must be less than or equal to ending price!"
        );

        // refund losing bid
        if auction.current_winner != Address::zero() {
            self.send()
                .direct_egld(&auction.current_winner, &auction.current_bid, b"bid refund");
        }

        // update auction bid and winner
        auction.current_bid = payment;
        auction.current_winner = caller;
        self.auction(kitty_id).set(&auction);

        Ok(())
    }

    #[endpoint(endAuction)]
    fn end_auction(&self, kitty_id: u32) -> SCResult<OptionalResult<AsyncCall<Self::SendApi>>> {
        require!(
            self.is_up_for_auction(kitty_id),
            "kitty is not up for auction!"
        );

        let auction = self.auction(kitty_id).get();

        require!(
            self.blockchain().get_block_timestamp() > auction.deadline
                || auction.current_bid == auction.ending_price,
            "auction has not ended yet!"
        );

        if auction.current_winner != Address::zero() {
            match auction.auction_type {
                AuctionType::Selling => Ok(self.transfer_to(auction.current_winner, kitty_id)),
                AuctionType::Siring => Ok(self.approve_siring_and_return_kitty(
                    auction.current_winner,
                    auction.kitty_owner,
                    kitty_id,
                )),
            }
        } else {
            // return kitty to its owner
            Ok(self.transfer_to(auction.kitty_owner, kitty_id))
        }
    }

    // private

    fn create_auction(
        &self,
        auction_type: AuctionType,
        kitty_id: u32,
        starting_price: Self::BigUint,
        ending_price: Self::BigUint,
        deadline: u64,
    ) -> OptionalResult<AsyncCall<Self::SendApi>> {
        let caller = self.blockchain().get_caller();

        let kitty_ownership_contract_address =
            self.get_kitty_ownership_contract_address_or_default();
        if kitty_ownership_contract_address != Address::zero() {
            OptionalResult::Some(
                self.kitty_ownership_proxy(kitty_ownership_contract_address)
                    .allow_auctioning(caller.clone(), kitty_id)
                    .async_call()
                    .with_callback(self.callbacks().allow_auctioning_callback(
                        auction_type,
                        kitty_id,
                        starting_price,
                        ending_price,
                        deadline,
                        caller,
                    )),
            )
        } else {
            OptionalResult::None
        }
    }

    fn start_gen_zero_kitty_auction(&self, kitty_id: u32) {
        let starting_price = self.gen_zero_kitty_starting_price().get();
        let ending_price = self.gen_zero_kitty_ending_price().get();
        let duration = self.gen_zero_kitty_auction_duration().get();
        let deadline = self.blockchain().get_block_timestamp() + duration;

        let auction = Auction::new(
            AuctionType::Selling,
            &starting_price,
            &ending_price,
            deadline,
            &self.blockchain().get_sc_address(),
        );

        self.auction(kitty_id).set(&auction);
    }

    fn transfer_to(
        &self,
        address: Address,
        kitty_id: u32,
    ) -> OptionalResult<AsyncCall<Self::SendApi>> {
        let kitty_ownership_contract_address =
            self.get_kitty_ownership_contract_address_or_default();
        if kitty_ownership_contract_address != Address::zero() {
            OptionalResult::Some(
                self.kitty_ownership_proxy(kitty_ownership_contract_address)
                    .transfer(address, kitty_id)
                    .async_call()
                    .with_callback(self.callbacks().transfer_callback(kitty_id)),
            )
        } else {
            OptionalResult::None
        }
    }

    fn approve_siring_and_return_kitty(
        &self,
        approved_address: Address,
        kitty_owner: Address,
        kitty_id: u32,
    ) -> OptionalResult<AsyncCall<Self::SendApi>> {
        let kitty_ownership_contract_address =
            self.get_kitty_ownership_contract_address_or_default();
        if kitty_ownership_contract_address != Address::zero() {
            OptionalResult::Some(
                self.kitty_ownership_proxy(kitty_ownership_contract_address)
                    .approve_siring_and_return_kitty(approved_address, kitty_owner, kitty_id)
                    // not a mistake, same callback for transfer and approveSiringAndReturnKitty
                    .async_call()
                    .with_callback(self.callbacks().transfer_callback(kitty_id)),
            )
        } else {
            OptionalResult::None
        }
    }

    fn get_kitty_ownership_contract_address_or_default(&self) -> Address {
        if self.kitty_ownership_contract_address().is_empty() {
            Address::zero()
        } else {
            self.kitty_ownership_contract_address().get()
        }
    }

    // callbacks

    #[callback]
    #[allow(clippy::too_many_arguments)]
    fn allow_auctioning_callback(
        &self,
        #[call_result] result: AsyncCallResult<()>,
        auction_type: AuctionType,
        cb_kitty_id: u32,
        starting_price: Self::BigUint,
        ending_price: Self::BigUint,
        deadline: u64,
        kitty_owner: Address,
    ) {
        match result {
            AsyncCallResult::Ok(()) => {
                let auction = Auction::new(
                    auction_type,
                    &starting_price,
                    &ending_price,
                    deadline,
                    &kitty_owner,
                );

                self.auction(cb_kitty_id).set(&auction);
            },
            AsyncCallResult::Err(_) => {
                // nothing to revert in case of error
            },
        }
    }

    #[callback]
    fn transfer_callback(&self, #[call_result] result: AsyncCallResult<()>, cb_kitty_id: u32) {
        match result {
            AsyncCallResult::Ok(()) => {
                let auction = self.auction(cb_kitty_id).get();
                self.auction(cb_kitty_id).clear();

                // send winning bid money to kitty owner
                // condition needed for gen zero kitties, since this sc is their owner
                // and for when no bid was made
                if auction.kitty_owner != self.blockchain().get_sc_address()
                    && auction.current_winner != Address::zero()
                {
                    self.send().direct_egld(
                        &auction.kitty_owner,
                        &auction.current_bid,
                        b"sold kitty",
                    );
                }
            },
            AsyncCallResult::Err(_) => {
                // this can only fail if the kitty_ownership contract address is invalid
                // nothing to revert in case of error
            },
        }
    }

    #[callback]
    fn approve_siring_callback(
        &self,
        #[call_result] result: AsyncCallResult<()>,
        cb_kitty_id: u32,
    ) -> OptionalResult<AsyncCall<Self::SendApi>> {
        match result {
            AsyncCallResult::Ok(()) => {
                let auction = self.auction(cb_kitty_id).get();

                // transfer kitty back to its owner
                self.transfer_to(auction.kitty_owner, cb_kitty_id)

                // auction data will be cleared in the transfer callback
                // winning bid money will be sent as well
            },
            AsyncCallResult::Err(_) => {
                // this can only fail if the kitty_ownership contract address is invalid
                // nothing to revert in case of error
                OptionalResult::None
            },
        }
    }

    #[callback]
    fn create_gen_zero_kitty_callback(&self, #[call_result] result: AsyncCallResult<u32>) {
        match result {
            AsyncCallResult::Ok(kitty_id) => {
                self.start_gen_zero_kitty_auction(kitty_id);
            },
            AsyncCallResult::Err(_) => {
                // this can only fail if the kitty_ownership contract address is invalid
                // nothing to revert in case of error
            },
        }
    }

    // proxy

    #[proxy]
    fn kitty_ownership_proxy(&self, to: Address) -> kitty_ownership::Proxy<Self::SendApi>;

    // storage

    // general

    #[storage_mapper("kittyOwnershipContractAddress")]
    fn kitty_ownership_contract_address(&self) -> SingleValueMapper<Self::Storage, Address>;

    // gen zero kitty

    #[storage_mapper("genZeroKittyStartingPrice")]
    fn gen_zero_kitty_starting_price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[storage_mapper("genZeroKittyEndingPrice")]
    fn gen_zero_kitty_ending_price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[storage_mapper("genZeroKittyAuctionDuration")]
    fn gen_zero_kitty_auction_duration(&self) -> SingleValueMapper<Self::Storage, u64>;

    // auction

    #[storage_mapper("auction")]
    fn auction(&self, kitty_id: u32) -> SingleValueMapper<Self::Storage, Auction<Self::BigUint>>;
}
