#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();
use elrond_wasm_modules::ongoing_operation::{
    CONTINUE_OP, DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, STOP_OP,
};

type Epoch = u64;

pub const EPOCHS_IN_WEEK: Epoch = 7;
pub const MAX_PERCENTAGE: u64 = 100_000; // 100%

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TypeAbi)]
pub struct Bracket {
    pub index_percent: u64,
    pub reward_percent: u64,
}

#[derive(ManagedVecItem, NestedEncode, NestedDecode, TypeAbi)]
pub struct ComputedBracket<M: ManagedTypeApi> {
    pub end_index: u64,
    pub reward: BigUint<M>,
}

#[derive(NestedEncode, NestedDecode)]
pub struct RaffleProgress<M: ManagedTypeApi> {
    pub ticket_position: u64,
    pub ticket_count: u64,
    pub computed_brackets: ManagedVec<M, ComputedBracket<M>>,
}

#[elrond_wasm::contract]
pub trait RewardsDistribution:
    elrond_wasm_modules::ongoing_operation::OngoingOperationModule
{
    #[init]
    fn init(&self, seed_nft_minter_address: ManagedAddress, brackets: ManagedVec<Bracket>) {
        self.seed_nft_minter_address().set(&seed_nft_minter_address);

        let nft_token_identifier: TokenIdentifier = self
            .seed_nft_minter_proxy(seed_nft_minter_address)
            .get_nft_token_id()
            .execute_on_dest_context();
        self.nft_token_identifier().set(nft_token_identifier);

        self.validate_brackets(&brackets);
        self.brackets().set(brackets);
    }

    #[payable("EGLD")]
    #[endpoint(depositRoyalties)]
    fn deposit_royalties(&self) {
        let value = self.call_value().egld_value();
        self.royalties().update(|total| *total += value);
    }

    #[endpoint(raffle)]
    fn raffle(&self) -> OperationCompletionStatus {
        let mut raffle = self
            .raffle_progress()
            .get()
            .unwrap_or_else(|| self.new_raffle());
        let mut rng = RandomnessSource::default();

        let mut bracket = raffle.computed_brackets.get(0);

        let mut total_distributed = BigUint::zero();
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

            self.rewards(ticket).update(|balance| {
                require!(balance.to_u64().unwrap() == 0u64, "OOF");
                *balance += &bracket.reward
            });
            total_distributed += &bracket.reward;

            if raffle.ticket_position == raffle.ticket_count {
                return STOP_OP;
            }

            raffle.ticket_position += 1;

            CONTINUE_OP
        });
        self.royalties()
            .update(|royalties| *royalties -= total_distributed);

        let raffle_progress = match run_result {
            OperationCompletionStatus::InterruptedBeforeOutOfGas => Some(raffle),
            OperationCompletionStatus::Completed => None,
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

        let reward_total: u64 = brackets.iter().map(|bracket| bracket.reward_percent).sum();
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

        let seed_nft_minter_address = self.seed_nft_minter_address().get();
        let ticket_count: u64 = self
            .seed_nft_minter_proxy(seed_nft_minter_address)
            .get_nft_count()
            .execute_on_dest_context();
        let brackets = self.brackets().get();

        let reward_amount = self.royalties().get();
        let computed_brackets = self.compute_brackets(brackets, ticket_count, reward_amount);

        let ticket_position = 1;

        RaffleProgress {
            ticket_position,
            ticket_count,
            computed_brackets,
        }
    }

    fn compute_brackets(
        &self,
        brackets: ManagedVec<Bracket>,
        ticket_count: u64,
        reward_amount: BigUint,
    ) -> ManagedVec<ComputedBracket<Self::Api>> {
        require!(ticket_count > 0, "No tickets");

        let mut computed_brackets = ManagedVec::new();
        let mut index_cutoff_percent = 0;

        let mut start_index = 0;
        for bracket in &brackets {
            index_cutoff_percent += bracket.index_percent;
            let end_index = ticket_count * index_cutoff_percent / MAX_PERCENTAGE;
            let total_reward_for_bracket =
                reward_amount.clone() * bracket.reward_percent / MAX_PERCENTAGE;
            let count = end_index - start_index;
            start_index = end_index;
            require!(count > 0, "Invalid bracket");
            let reward = total_reward_for_bracket / count;

            computed_brackets.push(ComputedBracket { end_index, reward });
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
    fn claim_rewards(&self) {
        let nfts = self.call_value().all_esdt_transfers();
        let nft_token_identifier = self.nft_token_identifier().get();
        require!(!nfts.is_empty(), "Missing payment");
        let mut total = BigUint::zero();
        for nft in &nfts {
            require!(
                nft.token_identifier == nft_token_identifier,
                "Invalid payment"
            );
            total += self.rewards(nft.token_nonce).take();
        }
        let caller = self.blockchain().get_caller();
        if total > 0 {
            self.send().direct_egld(&caller, &total);
        }
        self.send().direct_multi(&caller, &nfts);
    }

    #[view(getRoyalties)]
    #[storage_mapper("royalties")]
    fn royalties(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewards)]
    #[storage_mapper("rewards")]
    fn rewards(&self, nft_nonce: u64) -> SingleValueMapper<BigUint>;

    #[view(getSeedNftMinterAddress)]
    #[storage_mapper("seedNftMinterAddress")]
    fn seed_nft_minter_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getBrackets)]
    #[storage_mapper("brackets")]
    fn brackets(&self) -> SingleValueMapper<ManagedVec<Bracket>>;

    #[view(getLastRaffleEpoch)]
    #[storage_mapper("lastRaffleEpoch")]
    fn last_raffle_epoch(&self) -> SingleValueMapper<Epoch>;

    #[view(getNftTokenIdentifier)]
    #[storage_mapper("nftTokenIdentifier")]
    fn nft_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

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
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait SeedNftMinter {
        #[endpoint(getNftCount)]
        fn get_nft_count(&self) -> u64;

        #[endpoint(getNftTokenId)]
        fn get_nft_token_id(&self) -> TokenIdentifier;
    }
}
