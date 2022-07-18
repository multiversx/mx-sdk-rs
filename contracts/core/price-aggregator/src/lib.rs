#![no_std]

elrond_wasm::imports!();

pub mod median;
pub mod price_aggregator_data;
pub mod staking;

use price_aggregator_data::{OracleStatus, PriceFeed, TimestampedPrice, TokenPair};

const SUBMISSION_LIST_MAX_LEN: usize = 50;
const FIRST_SUBMISSION_TIMESTAMP_MAX_DIFF_SECONDS: u64 = 6;
pub const MAX_ROUND_DURATION_SECONDS: u64 = 1_800; // 30 minutes
static PAUSED_ERROR_MSG: &[u8] = b"Contract is paused";

#[elrond_wasm::contract]
pub trait PriceAggregator:
    elrond_wasm_modules::pause::PauseModule + staking::StakingModule
{
    #[init]
    fn init(
        &self,
        staking_token: EgldOrEsdtTokenIdentifier,
        staking_amount: BigUint,
        slash_amount: BigUint,
        slash_quorum: usize,
        submission_count: usize,
        decimals: u8,
        oracles: MultiValueEncoded<ManagedAddress>,
    ) {
        self.init_staking_module(
            &staking_token,
            &staking_amount,
            &slash_amount,
            slash_quorum,
            &oracles.to_vec(),
        );

        let is_deploy_call = !self.was_contract_deployed().get();
        if is_deploy_call {
            self.decimals().set(decimals);
            self.was_contract_deployed().set(true);
        }

        self.add_oracles(oracles);

        self.require_valid_submission_count(submission_count);
        self.submission_count().set(submission_count);

        self.set_paused(true);
    }

    #[only_owner]
    #[endpoint(addOracles)]
    fn add_oracles(&self, oracles: MultiValueEncoded<ManagedAddress>) {
        let mut oracle_mapper = self.oracle_status();
        for oracle in oracles {
            if !oracle_mapper.contains_key(&oracle) {
                let _ = oracle_mapper.insert(
                    oracle.clone(),
                    OracleStatus {
                        total_submissions: 0,
                        accepted_submissions: 0,
                    },
                );
                self.add_board_member(oracle);
            }
        }
    }

    /// Also receives submission count,
    /// so the owner does not have to update it manually with setSubmissionCount before this call
    #[only_owner]
    #[endpoint(removeOracles)]
    fn remove_oracles(&self, submission_count: usize, oracles: MultiValueEncoded<ManagedAddress>) {
        let mut oracle_mapper = self.oracle_status();
        for oracle in oracles {
            let _ = oracle_mapper.remove(&oracle);
            self.remove_board_member(&oracle);
        }

        self.require_valid_submission_count(submission_count);
        self.submission_count().set(submission_count);
    }

    #[endpoint]
    fn submit(
        &self,
        from: ManagedBuffer,
        to: ManagedBuffer,
        submission_timestamp: u64,
        price: BigUint,
    ) {
        self.require_not_paused();
        self.require_is_oracle();

        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            submission_timestamp <= current_timestamp,
            "Timestamp is from the future"
        );

        self.submit_unchecked(from, to, submission_timestamp, price);
    }

    fn submit_unchecked(
        &self,
        from: ManagedBuffer,
        to: ManagedBuffer,
        submission_timestamp: u64,
        price: BigUint,
    ) {
        let token_pair = TokenPair { from, to };
        let mut submissions = self
            .submissions()
            .entry(token_pair.clone())
            .or_default()
            .get();

        let first_sub_time_mapper = self.first_submission_timestamp(&token_pair);
        let last_sub_time_mapper = self.last_submission_timestamp(&token_pair);

        let current_timestamp = self.blockchain().get_block_timestamp();
        let mut is_first_submission = false;
        let mut first_submission_timestamp = if submissions.is_empty() {
            self.require_valid_first_submission(submission_timestamp, current_timestamp);

            first_sub_time_mapper.set(current_timestamp);
            is_first_submission = true;

            current_timestamp
        } else {
            first_sub_time_mapper.get()
        };

        // round was not completed in time, so it's discarded
        if current_timestamp > first_submission_timestamp + MAX_ROUND_DURATION_SECONDS {
            self.require_valid_first_submission(submission_timestamp, current_timestamp);

            submissions.clear();
            first_sub_time_mapper.set(current_timestamp);
            last_sub_time_mapper.set(current_timestamp);

            first_submission_timestamp = current_timestamp;
            is_first_submission = true;
        }

        let caller = self.blockchain().get_caller();
        let accepted = !submissions.contains_key(&caller)
            && (is_first_submission || submission_timestamp >= first_submission_timestamp);
        if accepted {
            submissions.insert(caller, price);
            last_sub_time_mapper.set(current_timestamp);

            self.create_new_round(token_pair, submissions);
        }

        self.oracle_status()
            .entry(self.blockchain().get_caller())
            .and_modify(|oracle_status| {
                oracle_status.accepted_submissions += accepted as u64;
                oracle_status.total_submissions += 1;
            });
    }

    fn require_valid_first_submission(&self, submission_timestamp: u64, current_timestamp: u64) {
        require!(
            current_timestamp - submission_timestamp <= FIRST_SUBMISSION_TIMESTAMP_MAX_DIFF_SECONDS,
            "First submission too old"
        );
    }

    #[endpoint(submitBatch)]
    fn submit_batch(
        &self,
        submissions: MultiValueEncoded<MultiValue4<ManagedBuffer, ManagedBuffer, u64, BigUint>>,
    ) {
        self.require_not_paused();
        self.require_is_oracle();

        let current_timestamp = self.blockchain().get_block_timestamp();
        for (from, to, submission_timestamp, price) in submissions
            .into_iter()
            .map(|submission| submission.into_tuple())
        {
            require!(
                submission_timestamp <= current_timestamp,
                "Timestamp is from the future"
            );

            self.submit_unchecked(from, to, submission_timestamp, price);
        }
    }

    fn require_is_oracle(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            self.oracle_status().contains_key(&caller) && self.is_staked_board_member(&caller),
            "only oracles allowed"
        );
    }

    fn require_valid_submission_count(&self, submission_count: usize) {
        require!(
            submission_count >= 1
                && submission_count <= self.oracle_status().len()
                && submission_count <= SUBMISSION_LIST_MAX_LEN,
            "Invalid submission count"
        )
    }

    fn create_new_round(
        &self,
        token_pair: TokenPair<Self::Api>,
        mut submissions: MapMapper<ManagedAddress, BigUint>,
    ) {
        let submissions_len = submissions.len();
        if submissions_len >= self.submission_count().get() {
            require!(
                submissions_len <= SUBMISSION_LIST_MAX_LEN,
                "submission list capacity exceeded"
            );

            let mut submissions_vec = ArrayVec::<BigUint, SUBMISSION_LIST_MAX_LEN>::new();
            for submission_value in submissions.values() {
                submissions_vec.push(submission_value);
            }

            let price_result = median::calculate(submissions_vec.as_mut_slice());
            let price_opt = price_result.unwrap_or_else(|err| sc_panic!(err.as_bytes()));
            let price = price_opt.unwrap_or_else(|| sc_panic!("no submissions"));
            let price_feed = TimestampedPrice {
                price,
                timestamp: self.blockchain().get_block_timestamp(),
            };

            submissions.clear();
            self.first_submission_timestamp(&token_pair).clear();
            self.last_submission_timestamp(&token_pair).clear();

            self.rounds()
                .entry(token_pair)
                .or_default()
                .get()
                .push(&price_feed);
        }
    }

    #[view(latestRoundData)]
    fn latest_round_data(&self) -> MultiValueEncoded<PriceFeed<Self::Api>> {
        self.require_not_paused();
        require!(!self.rounds().is_empty(), "no completed rounds");

        let mut result = MultiValueEncoded::new();
        for (token_pair, round_values) in self.rounds().iter() {
            result.push(self.make_price_feed(token_pair, round_values));
        }

        result
    }

    #[view(latestPriceFeed)]
    fn latest_price_feed(
        &self,
        from: ManagedBuffer,
        to: ManagedBuffer,
    ) -> SCResult<MultiValue6<u32, ManagedBuffer, ManagedBuffer, u64, BigUint, u8>> {
        require_old!(self.not_paused(), PAUSED_ERROR_MSG);

        let token_pair = TokenPair { from, to };
        let round_values = self
            .rounds()
            .get(&token_pair)
            .ok_or("token pair not found")?;
        let feed = self.make_price_feed(token_pair, round_values);
        Ok((
            feed.round_id,
            feed.from,
            feed.to,
            feed.timestamp,
            feed.price,
            feed.decimals,
        )
            .into())
    }

    #[view(latestPriceFeedOptional)]
    fn latest_price_feed_optional(
        &self,
        from: ManagedBuffer,
        to: ManagedBuffer,
    ) -> OptionalValue<MultiValue6<u32, ManagedBuffer, ManagedBuffer, u64, BigUint, u8>> {
        self.latest_price_feed(from, to).ok().into()
    }

    #[only_owner]
    #[endpoint(setSubmissionCount)]
    fn set_submission_count(&self, submission_count: usize) {
        self.require_valid_submission_count(submission_count);
        self.submission_count().set(submission_count);
    }

    fn make_price_feed(
        &self,
        token_pair: TokenPair<Self::Api>,
        round_values: VecMapper<TimestampedPrice<Self::Api>>,
    ) -> PriceFeed<Self::Api> {
        let round_id = round_values.len();
        let last_price = round_values.get(round_id);

        PriceFeed {
            round_id: round_id as u32,
            from: token_pair.from,
            to: token_pair.to,
            timestamp: last_price.timestamp,
            price: last_price.price,
            decimals: self.decimals().get(),
        }
    }

    #[view(getOracles)]
    fn get_oracles(&self) -> MultiValueEncoded<ManagedAddress> {
        let mut result = MultiValueEncoded::new();
        for key in self.oracle_status().keys() {
            result.push(key);
        }
        result
    }

    fn require_not_paused(&self) {
        require!(self.not_paused(), PAUSED_ERROR_MSG);
    }

    #[storage_mapper("was_contract_deployed")]
    fn was_contract_deployed(&self) -> SingleValueMapper<bool>;

    #[view]
    #[storage_mapper("submission_count")]
    fn submission_count(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("decimals")]
    fn decimals(&self) -> SingleValueMapper<u8>;

    #[storage_mapper("oracle_status")]
    fn oracle_status(&self) -> MapMapper<ManagedAddress, OracleStatus>;

    #[storage_mapper("rounds")]
    fn rounds(
        &self,
    ) -> MapStorageMapper<TokenPair<Self::Api>, VecMapper<TimestampedPrice<Self::Api>>>;

    #[storage_mapper("first_submission_timestamp")]
    fn first_submission_timestamp(
        &self,
        token_pair: &TokenPair<Self::Api>,
    ) -> SingleValueMapper<u64>;

    #[storage_mapper("last_submission_timestamp")]
    fn last_submission_timestamp(
        &self,
        token_pair: &TokenPair<Self::Api>,
    ) -> SingleValueMapper<u64>;

    #[storage_mapper("submissions")]
    fn submissions(
        &self,
    ) -> MapStorageMapper<TokenPair<Self::Api>, MapMapper<ManagedAddress, BigUint>>;
}
