#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();
use multiversx_sc_modules::ongoing_operation::{
    CONTINUE_OP, DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, STOP_OP,
};

type Epoch = u64;

pub const EPOCHS_IN_WEEK: Epoch = 7;
pub const MAX_PERCENTAGE: u64 = 100_000; // 100%
pub const DIVISION_SAFETY_CONSTANT: u64 = 1_000_000_000_000;

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TypeAbi)]
pub struct Bracket {
    pub index_percent: u64,
    pub bracket_reward_percent: u64,
}

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TypeAbi)]
pub struct ComputedBracket<M: ManagedTypeApi> {
    pub end_index: u64,
    pub nft_reward_percent: BigUint<M>,
}

#[derive(NestedEncode, NestedDecode)]
pub struct RaffleProgress<M: ManagedTypeApi> {
    pub raffle_id: u64,
    pub ticket_position: u64,
    pub ticket_count: u64,
    pub computed_brackets: ManagedVec<M, ComputedBracket<M>>,
}

#[multiversx_sc::contract]
pub trait RewardsDistribution:
    multiversx_sc_modules::ongoing_operation::OngoingOperationModule
{
    #[init]
    fn init(&self, seed_nft_minter_address: ManagedAddress, brackets: ManagedVec<Bracket>) {
        self.seed_nft_minter_address().set(&seed_nft_minter_address);

        let nft_token_id: TokenIdentifier = self
            .seed_nft_minter_proxy(seed_nft_minter_address)
            .get_nft_token_id()
            .execute_on_dest_context();
        self.nft_token_id().set(nft_token_id);

        self.validate_brackets(&brackets);
        self.brackets().set(brackets);
    }

    #[payable("*")]
    #[endpoint(depositRoyalties)]
    fn deposit_royalties(&self) {
        let payment = self.call_value().egld_or_single_esdt();
        let raffle_id = self.raffle_id().get();
        self.royalties(raffle_id, &payment.token_identifier, payment.token_nonce)
            .update(|total| *total += payment.amount);
    }

    #[endpoint(raffle)]
    fn raffle(&self) -> OperationCompletionStatus {
        let mut raffle = self
            .raffle_progress()
            .get()
            .unwrap_or_else(|| self.new_raffle());
        let mut rng = RandomnessSource::default();

        let mut bracket = raffle.computed_brackets.get(0);

        let run_result = self.run_while_it_has_gas(DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, || {
            let ticket = self.shuffle_and_pick_single_ticket(
                &mut rng,
                raffle.ticket_position,
                raffle.ticket_count,
            );
            self.try_advance_bracket(
                &mut bracket,
                &mut raffle.computed_brackets,
                raffle.ticket_position,
            );

            self.nft_reward_percent(raffle.raffle_id, ticket)
                .update(|nft_reward_percent| *nft_reward_percent += &bracket.nft_reward_percent);

            if raffle.ticket_position == raffle.ticket_count {
                return STOP_OP;
            }

            raffle.ticket_position += 1;

            CONTINUE_OP
        });

        let raffle_progress = match run_result {
            OperationCompletionStatus::InterruptedBeforeOutOfGas => Some(raffle),
            OperationCompletionStatus::Completed => {
                self.completed_raffle_id_count().set(raffle.raffle_id + 1);
                None
            },
        };

        self.raffle_progress().set(raffle_progress);

        run_result
    }

    fn validate_brackets(&self, brackets: &ManagedVec<Bracket>) {
        let index_total: u64 = brackets.iter().map(|bracket| bracket.index_percent).sum();
        require!(
            index_total == MAX_PERCENTAGE,
            "Index percent total must be 100%"
        );

        let reward_total: u64 = brackets
            .iter()
            .map(|bracket| bracket.bracket_reward_percent)
            .sum();
        require!(
            reward_total == MAX_PERCENTAGE,
            "Reward percent total must be 100%"
        );
    }

    fn try_advance_bracket(
        &self,
        bracket: &mut ComputedBracket<Self::Api>,
        computed_brackets: &mut ManagedVec<ComputedBracket<Self::Api>>,
        ticket: u64,
    ) {
        while ticket > bracket.end_index {
            computed_brackets.remove(0);
            *bracket = computed_brackets.get(0);
        }
    }

    /// Fisher-Yates algorithm,
    /// each position i is swapped with a random one in range [i, n]
    ///
    /// After shuffling, the ticket at the current position is taken and returned
    fn shuffle_and_pick_single_ticket(
        &self,
        rng: &mut RandomnessSource,
        current_ticket_position: u64,
        ticket_count: u64,
    ) -> u64 {
        let rand_pos = rng.next_u64_in_range(current_ticket_position, ticket_count + 1);

        let current_ticket_id = self.take_ticket(current_ticket_position);
        if rand_pos == current_ticket_position {
            current_ticket_id
        } else {
            self.replace_ticket(rand_pos, current_ticket_id)
        }
    }

    fn take_ticket(&self, position: u64) -> u64 {
        let id = self.tickets(position).take();
        ticket_from_storage(position, id)
    }

    fn replace_ticket(&self, position: u64, new_ticket_id: u64) -> u64 {
        let id_to_save = ticket_to_storage(position, new_ticket_id);
        let loaded_id = self.tickets(position).replace(id_to_save);
        ticket_from_storage(position, loaded_id)
    }

    fn new_raffle(&self) -> RaffleProgress<Self::Api> {
        self.require_new_raffle_period();

        let raffle_id = self.raffle_id().update(|raffle_id| {
            let last_id = *raffle_id;
            *raffle_id += 1;
            last_id
        });

        let seed_nft_minter_address = self.seed_nft_minter_address().get();
        let ticket_count: u64 = self
            .seed_nft_minter_proxy(seed_nft_minter_address)
            .get_nft_count()
            .execute_on_dest_context();
        let brackets = self.brackets().get();

        let computed_brackets = self.compute_brackets(brackets, ticket_count);

        let ticket_position = 1;

        RaffleProgress {
            raffle_id,
            ticket_position,
            ticket_count,
            computed_brackets,
        }
    }

    fn compute_brackets(
        &self,
        brackets: ManagedVec<Bracket>,
        ticket_count: u64,
    ) -> ManagedVec<ComputedBracket<Self::Api>> {
        require!(ticket_count > 0, "No tickets");

        let mut computed_brackets = ManagedVec::new();
        let mut index_cutoff_percent = 0;

        let mut start_index = 0;
        for bracket in &brackets {
            index_cutoff_percent += bracket.index_percent;
            let end_index = ticket_count * index_cutoff_percent / MAX_PERCENTAGE;
            let count = end_index - start_index;
            start_index = end_index;
            require!(count > 0, "Invalid bracket");
            let nft_reward_percent =
                BigUint::from(bracket.bracket_reward_percent) * DIVISION_SAFETY_CONSTANT / count;

            computed_brackets.push(ComputedBracket {
                end_index,
                nft_reward_percent,
            });
        }

        computed_brackets
    }

    fn require_new_raffle_period(&self) {
        let current_epoch = self.blockchain().get_block_epoch();
        let last_raffle_epoch = self.last_raffle_epoch().replace(current_epoch);
        if last_raffle_epoch == 0 {
            return;
        }
        require!(
            last_raffle_epoch + EPOCHS_IN_WEEK <= current_epoch,
            "Last raffle was less than one week ago"
        );
    }

    #[payable("*")]
    #[endpoint(claimRewards)]
    fn claim_rewards(
        &self,
        raffle_id_start: u64,
        raffle_id_end: u64,
        reward_tokens: MultiValueEncoded<MultiValue2<EgldOrEsdtTokenIdentifier, u64>>,
    ) {
        let nfts = self.call_value().all_esdt_transfers();
        self.validate_nft_payments(&nfts);
        self.validate_raffle_id_range(raffle_id_start, raffle_id_end);

        let caller = self.blockchain().get_caller();
        let mut rewards = ManagedVec::new();
        let mut total_egld_reward = BigUint::zero();

        for reward_token_pair in reward_tokens.into_iter() {
            let (reward_token_id, reward_token_nonce) = reward_token_pair.into_tuple();
            let (egld_reward, reward_payment_opt) = self.claim_reward_token(
                raffle_id_start,
                raffle_id_end,
                &reward_token_id,
                reward_token_nonce,
                &nfts,
            );

            total_egld_reward += egld_reward;
            if let Some(reward_payment) = reward_payment_opt {
                rewards.push(reward_payment);
            }
        }

        self.send()
            .direct_non_zero_egld(&caller, &total_egld_reward);
        self.send().direct_multi(&caller, &rewards);
        self.send().direct_multi(&caller, &nfts);
    }

    fn claim_reward_token(
        &self,
        raffle_id_start: u64,
        raffle_id_end: u64,
        reward_token_id: &EgldOrEsdtTokenIdentifier,
        reward_token_nonce: u64,
        nfts: &ManagedVec<EsdtTokenPayment>,
    ) -> (BigUint, Option<EsdtTokenPayment>) {
        let mut total = BigUint::zero();

        for raffle_id in raffle_id_start..=raffle_id_end {
            for nft in nfts {
                let claim_result =
                    self.try_claim(raffle_id, reward_token_id, reward_token_nonce, &nft);
                if claim_result.is_err() {
                    continue;
                }

                total += self.compute_claimable_amount(
                    raffle_id,
                    reward_token_id,
                    reward_token_nonce,
                    nft.token_nonce,
                );
            }
        }

        if total == 0 || reward_token_id.is_egld() {
            return (total, None);
        }
        let reward_payment = EsdtTokenPayment::new(
            reward_token_id.clone().unwrap_esdt(),
            reward_token_nonce,
            total,
        );
        (BigUint::zero(), Some(reward_payment))
    }

    fn try_claim(
        &self,
        raffle_id: u64,
        reward_token_id: &EgldOrEsdtTokenIdentifier,
        reward_token_nonce: u64,
        nft: &EsdtTokenPayment,
    ) -> Result<(), ()> {
        let was_claimed_mapper = self.was_claimed(
            raffle_id,
            reward_token_id,
            reward_token_nonce,
            nft.token_nonce,
        );
        let available = !was_claimed_mapper.get();
        if available {
            was_claimed_mapper.set(true);
            Result::Ok(())
        } else {
            Result::Err(())
        }
    }

    #[view(computeClaimableAmount)]
    fn compute_claimable_amount(
        &self,
        raffle_id: u64,
        reward_token_id: &EgldOrEsdtTokenIdentifier,
        reward_token_nonce: u64,
        nft_nonce: u64,
    ) -> BigUint {
        let nft_reward_percent = self.nft_reward_percent(raffle_id, nft_nonce).get();
        let royalties = self
            .royalties(raffle_id, reward_token_id, reward_token_nonce)
            .get();
        royalties * nft_reward_percent / MAX_PERCENTAGE / DIVISION_SAFETY_CONSTANT
    }

    fn validate_nft_payments(&self, nfts: &ManagedVec<EsdtTokenPayment>) {
        let nft_token_id = self.nft_token_id().get();
        require!(!nfts.is_empty(), "Missing payment");
        for nft in nfts {
            require!(nft.token_identifier == nft_token_id, "Invalid payment");
        }
    }

    fn validate_raffle_id_range(&self, start: u64, end: u64) {
        require!(start <= end, "Invalid range");

        let completed_raffle_id_count = self.completed_raffle_id_count().get();
        require!(end < completed_raffle_id_count, "Invalid raffle id end");
    }

    #[view(getRaffleId)]
    #[storage_mapper("raffleId")]
    fn raffle_id(&self) -> SingleValueMapper<u64>;

    #[view(getCompletedRaffleIdCount)]
    #[storage_mapper("completedRaffleIdCount")]
    fn completed_raffle_id_count(&self) -> SingleValueMapper<u64>;

    #[view(getRoyalties)]
    #[storage_mapper("royalties")]
    fn royalties(
        &self,
        raffle_id: u64,
        reward_token_id: &EgldOrEsdtTokenIdentifier,
        reward_token_nonce: u64,
    ) -> SingleValueMapper<BigUint>;

    #[view(getNftRewardPercent)]
    #[storage_mapper("nftRewardPercent")]
    fn nft_reward_percent(&self, raffle_id: u64, nft_nonce: u64) -> SingleValueMapper<BigUint>;

    #[view(getWasClaimed)]
    #[storage_mapper("wasClaimed")]
    fn was_claimed(
        &self,
        raffle_id: u64,
        reward_token_id: &EgldOrEsdtTokenIdentifier,
        reward_token_nonce: u64,
        nft_nonce: u64,
    ) -> SingleValueMapper<bool>;

    #[view(getSeedNftMinterAddress)]
    #[storage_mapper("seedNftMinterAddress")]
    fn seed_nft_minter_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getBrackets)]
    #[storage_mapper("brackets")]
    fn brackets(&self) -> SingleValueMapper<ManagedVec<Bracket>>;

    #[view(getLastRaffleEpoch)]
    #[storage_mapper("lastRaffleEpoch")]
    fn last_raffle_epoch(&self) -> SingleValueMapper<Epoch>;

    #[view(getNftTokenId)]
    #[storage_mapper("nftTokenIdentifier")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("tickets")]
    fn tickets(&self, position: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("currentTicketId")]
    fn current_ticket_id(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("raffleProgress")]
    fn raffle_progress(&self) -> SingleValueMapper<Option<RaffleProgress<Self::Api>>>;

    #[proxy]
    fn seed_nft_minter_proxy(&self, address: ManagedAddress) -> seed_nft_minter::Proxy<Self::Api>;
}

fn ticket_to_storage(position: u64, ticket_id: u64) -> u64 {
    if position == ticket_id {
        0
    } else {
        ticket_id
    }
}

fn ticket_from_storage(position: u64, ticket_id: u64) -> u64 {
    if ticket_id == 0 {
        position
    } else {
        ticket_id
    }
}

mod seed_nft_minter {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait SeedNftMinter {
        #[endpoint(getNftCount)]
        fn get_nft_count(&self) -> u64;

        #[endpoint(getNftTokenId)]
        fn get_nft_token_id(&self) -> TokenIdentifier;
    }
}
