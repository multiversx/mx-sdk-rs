#![no_std]

elrond_wasm::imports!();

mod events;
pub mod median;
pub mod price_aggregator_data;
pub mod staking;

use price_aggregator_data::{
    OracleStatus, PriceFeed, PriceFeedMultiValue, TimestampedPrice, TokenPair,
};

const SUBMISSION_LIST_MAX_LEN: usize = 50;
const FIRST_SUBMISSION_TIMESTAMP_MAX_DIFF_SECONDS: u64 = 30;
pub const MAX_ROUND_DURATION_SECONDS: u64 = 1_800; // 30 minutes
static PAUSED_ERROR_MSG: &[u8] = b"Contract is paused";
static PAIR_DECIMALS_NOT_CONFIGURED_ERROR: &[u8] = b"pair decimals not configured";
static WRONG_NUMBER_OF_DECIMALS_ERROR: &[u8] = b"wrong number of decimals";

#[elrond_wasm::contract]
pub trait PriceAggregator:
    elrond_wasm_modules::pause::PauseModule + staking::StakingModule + events::EventsModule
{
    #[init]
    fn init(
        &self,
        staking_token: EgldOrEsdtTokenIdentifier,
        staking_amount: BigUint,
        slash_amount: BigUint,
        slash_quorum: usize,
        submission_count: usize,
        oracles: MultiValueEncoded<ManagedAddress>,
    ) {
        self.init_staking_module(
            &staking_token,
            &staking_amount,
            &slash_amount,
            slash_quorum,
            &oracles.to_vec(),
        );

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
        decimals: u8,
    ) {
        self.require_not_paused();
        self.require_is_oracle();

        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            submission_timestamp <= current_timestamp,
            "Timestamp is from the future"
        );

        self.check_decimals(&from, &to, decimals);

        self.submit_unchecked(from, to, submission_timestamp, price, decimals);
    }

    fn submit_unchecked(
        &self,
        from: ManagedBuffer,
        to: ManagedBuffer,
        submission_timestamp: u64,
        price: BigUint,
        decimals: u8,
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

            self.create_new_round(token_pair, submissions, decimals);
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
        submissions: MultiValueEncoded<MultiValue5<ManagedBuffer, ManagedBuffer, u64, BigUint, u8>>,
    ) {
        self.require_not_paused();
        self.require_is_oracle();

        let current_timestamp = self.blockchain().get_block_timestamp();
        for (from, to, submission_timestamp, price, decimals) in submissions
            .into_iter()
            .map(|submission| submission.into_tuple())
        {
            require!(
                submission_timestamp <= current_timestamp,
                "Timestamp is from the future"
            );

            self.check_decimals(&from, &to, decimals);

            self.submit_unchecked(from, to, submission_timestamp, price, decimals);
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
        decimals: u8,
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
                decimals,
            };

            submissions.clear();
            self.first_submission_timestamp(&token_pair).clear();
            self.last_submission_timestamp(&token_pair).clear();

            self.rounds()
                .entry(token_pair.clone())
                .or_default()
                .get()
                .push(&price_feed);
            self.emit_new_round_event(&token_pair, &price_feed);
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

    fn get_price_feed(
        &self,
        from: ManagedBuffer,
        to: ManagedBuffer,
    ) -> SCResult<PriceFeed<Self::Api>> {
        require_old!(self.not_paused(), PAUSED_ERROR_MSG);

        let token_pair = TokenPair { from, to };
        let round_values = self
            .rounds()
            .get(&token_pair)
            .ok_or("token pair not found")?;

        Ok(self.make_price_feed(token_pair, round_values))
    }

    #[view(latestPriceFeed)]
    fn latest_price_feed(
        &self,
        from: ManagedBuffer,
        to: ManagedBuffer,
    ) -> SCResult<PriceFeedMultiValue<Self::Api>> {
        let result_price_feed = self.get_price_feed(from, to);
        match result_price_feed {
            SCResult::Ok(feed) => Ok(feed.into_multi_value()),
            SCResult::Err(err) => SCResult::Err(err),
        }
    }

    #[view(latestPriceFeedOptional)]
    fn latest_price_feed_optional(
        &self,
        from: ManagedBuffer,
        to: ManagedBuffer,
    ) -> OptionalValue<PriceFeedMultiValue<Self::Api>> {
        self.latest_price_feed(from, to).ok().into()
    }

    /// Receives pairs of (from, to) arguments,
    /// where `from` is the token ticker which we query the price for,
    /// and `to` is the token ticker in which we want the price to be denominated in.
    ///
    /// For example, if we want the USDC price of EGLD, we would pass "(EGLD, USDC)",
    /// even if the USDC full token ID would be something like USDC-abcdef. Only ticker is needed.
    ///
    /// Returns all the prices. Will fail if one of the prices of not available.
    /// If you want to do error handling yourself, use the `latestPriceFeedOptionalMulti` view.
    #[view(latestPriceFeedMulti)]
    fn latest_price_feed_multi(
        &self,
        from_to_pairs: MultiValueEncoded<MultiValue2<ManagedBuffer, ManagedBuffer>>,
    ) -> SCResult<MultiValueEncoded<PriceFeedMultiValue<Self::Api>>> {
        let mut results = MultiValueEncoded::new();
        for pair in from_to_pairs {
            let (from, to) = pair.into_tuple();
            let price_feed = self.latest_price_feed(from, to)?;
            results.push(price_feed);
        }

        Ok(results)
    }

    /// Same as the `latestPriceFeedMulti` view, but instead,
    /// returns Option<Price> instead of signalling an error if any prices are missing.
    #[view(latestPriceFeedOptionalMulti)]
    fn latest_price_feed_optional_multi(
        &self,
        from_to_pairs: MultiValueEncoded<MultiValue2<ManagedBuffer, ManagedBuffer>>,
    ) -> MultiValueEncoded<Option<PriceFeed<Self::Api>>> {
        let mut results = MultiValueEncoded::new();
        for pair in from_to_pairs {
            let (from, to) = pair.into_tuple();
            let result_price_feed = self.get_price_feed(from, to);
            match result_price_feed {
                SCResult::Ok(price_feed) => results.push(Some(price_feed)),
                SCResult::Err(_) => results.push(None),
            };
        }

        results
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
            decimals: last_price.decimals,
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

    fn clear_submissions(&self, token_pair: &TokenPair<Self::Api>) {
        if let Some(mut pair_submission_mapper) = self.submissions().get(token_pair) {
            pair_submission_mapper.clear();
        }
        self.first_submission_timestamp(token_pair).clear();
        self.last_submission_timestamp(token_pair).clear();
    }

    #[only_owner]
    #[endpoint(setPairDecimals)]
    fn set_pair_decimals(&self, from: ManagedBuffer, to: ManagedBuffer, decimals: u8) {
        self.require_paused();

        self.pair_decimals(&from, &to).set(Some(decimals));
        let pair = TokenPair { from, to };
        self.clear_submissions(&pair);
    }

    fn check_decimals(&self, from: &ManagedBuffer, to: &ManagedBuffer, decimals: u8) {
        let configured_decimals = self.get_pair_decimals(from, to);
        require!(
            decimals == configured_decimals,
            WRONG_NUMBER_OF_DECIMALS_ERROR
        )
    }

    #[view(getPairDecimals)]
    fn get_pair_decimals(&self, from: &ManagedBuffer, to: &ManagedBuffer) -> u8 {
        self.pair_decimals(from, to)
            .get()
            .unwrap_or_else(|| sc_panic!(PAIR_DECIMALS_NOT_CONFIGURED_ERROR))
    }

    #[storage_mapper("pair_decimals")]
    fn pair_decimals(
        &self,
        from: &ManagedBuffer,
        to: &ManagedBuffer,
    ) -> SingleValueMapper<Option<u8>>;

    #[view]
    #[storage_mapper("submission_count")]
    fn submission_count(&self) -> SingleValueMapper<usize>;

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
