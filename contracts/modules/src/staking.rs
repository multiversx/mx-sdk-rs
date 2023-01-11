multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode)]
pub struct TokenAmountPair<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
}

static NOT_ENOUGH_STAKE_ERR_MSG: &[u8] = b"Not enough stake";

#[multiversx_sc::module]
pub trait StakingModule {
    fn init_staking_module(
        &self,
        staking_token: &EgldOrEsdtTokenIdentifier,
        staking_amount: &BigUint,
        slash_amount: &BigUint,
        slash_quorum: usize,
        user_whitelist: &ManagedVec<ManagedAddress>,
    ) {
        let nr_board_members = user_whitelist.len();
        require!(nr_board_members > 0, "No board members");
        require!(
            slash_quorum <= nr_board_members,
            "Quorum higher than total possible board members"
        );
        require!(
            staking_amount > &0 && slash_amount > &0,
            "Staking and slash amount cannot be 0"
        );
        require!(
            slash_amount <= staking_amount,
            "Slash amount cannot be higher than required stake"
        );

        self.staking_token().set(staking_token);
        self.required_stake_amount().set(staking_amount);
        self.slash_amount().set(slash_amount);
        self.slash_quorum().set(slash_quorum);

        for user in user_whitelist {
            let _ = self.user_whitelist().insert(user);
        }
    }

    #[payable("*")]
    #[endpoint]
    fn stake(&self) {
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        let staking_token = self.staking_token().get();
        require!(payment_token == staking_token, "Invalid payment token");

        let caller = self.blockchain().get_caller();
        require!(
            self.user_whitelist().contains(&caller),
            "Only whitelisted members can stake"
        );

        self.staked_amount(&caller)
            .update(|amt| *amt += payment_amount);
    }

    #[endpoint]
    fn unstake(&self, unstake_amount: BigUint) {
        let caller = self.blockchain().get_caller();
        let staked_amount_mapper = self.staked_amount(&caller);
        let staked_amount = staked_amount_mapper.get();
        require!(unstake_amount <= staked_amount, NOT_ENOUGH_STAKE_ERR_MSG);

        let leftover_amount = &staked_amount - &unstake_amount;
        let required_stake_amount = self.required_stake_amount().get();
        if self.user_whitelist().contains(&caller) {
            require!(
                leftover_amount >= required_stake_amount,
                NOT_ENOUGH_STAKE_ERR_MSG
            );
        }

        staked_amount_mapper.set(&leftover_amount);

        let staking_token = self.staking_token().get();
        self.send()
            .direct(&caller, &staking_token, 0, &unstake_amount);
    }

    #[endpoint(voteSlashMember)]
    fn vote_slash_member(&self, member_to_slash: ManagedAddress) {
        require!(
            self.is_staked_board_member(&member_to_slash),
            "Voted user is not a staked board member"
        );

        let caller = self.blockchain().get_caller();
        require!(
            self.is_staked_board_member(&caller),
            NOT_ENOUGH_STAKE_ERR_MSG
        );

        let _ = self
            .slashing_proposal_voters(&member_to_slash)
            .insert(caller);
    }

    #[endpoint(slashMember)]
    fn slash_member(&self, member_to_slash: ManagedAddress) {
        let quorum = self.slash_quorum().get();
        let mut slashing_voters_mapper = self.slashing_proposal_voters(&member_to_slash);
        require!(slashing_voters_mapper.len() >= quorum, "Quorum not reached");

        let slash_amount = self.slash_amount().get();
        self.staked_amount(&member_to_slash)
            .update(|amt| *amt -= &slash_amount);
        self.total_slashed_amount()
            .update(|total| *total += slash_amount);

        slashing_voters_mapper.clear();
    }

    fn is_staked_board_member(&self, user: &ManagedAddress) -> bool {
        let required_stake = self.required_stake_amount().get();
        let user_stake = self.staked_amount(user).get();

        self.user_whitelist().contains(user) && user_stake >= required_stake
    }

    #[inline]
    fn add_board_member(&self, user: ManagedAddress) {
        let _ = self.user_whitelist().insert(user);
    }

    fn remove_board_member(&self, user: &ManagedAddress) {
        let mut whitelist_mapper = self.user_whitelist();
        let was_whitelisted = whitelist_mapper.swap_remove(user);
        if !was_whitelisted {
            return;
        }

        // remove user's votes as well
        for board_member in whitelist_mapper.iter() {
            let _ = self
                .slashing_proposal_voters(&board_member)
                .swap_remove(user);
        }
        self.slashing_proposal_voters(user).clear();
    }

    #[storage_mapper("staking_module:stakingToken")]
    fn staking_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[storage_mapper("staking_module:requiredStakeAmount")]
    fn required_stake_amount(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("staking_module:userWhitelist")]
    fn user_whitelist(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("staking_module:stakedAmount")]
    fn staked_amount(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("staking_module:slashingProposalVoters")]
    fn slashing_proposal_voters(
        &self,
        slash_address: &ManagedAddress,
    ) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("staking_module:slashQuorum")]
    fn slash_quorum(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("staking_module:slashAmount")]
    fn slash_amount(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("staking_module:totalSlashedAmount")]
    fn total_slashed_amount(&self) -> SingleValueMapper<BigUint>;
}
