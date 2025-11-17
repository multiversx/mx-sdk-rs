use multiversx_sc::imports::*;

use crate::{
    basics::{
        constants::{MAX_TICKETS, PERCENTAGE_TOTAL, THIRTY_DAYS_IN_MILLISECONDS},
        storage, utils, views,
    },
    specific::{lottery_info::LotteryInfo, status::Status},
};

#[multiversx_sc::module]
pub trait SetupModule: storage::StorageModule + views::ViewsModule + utils::UtilsModule {
    #[allow_multiple_var_args]
    #[allow(clippy::too_many_arguments)]
    #[endpoint(startLottery)]
    fn start_lottery(
        &self,
        lottery_name: ManagedBuffer,
        token_identifier: TokenIdentifier,
        ticket_price: BigUint,
        opt_total_tickets: Option<usize>,
        opt_deadline: Option<TimestampMillis>,
        opt_max_entries_per_user: Option<usize>,
        opt_prize_distribution: ManagedOption<ManagedVec<u8>>,
        opt_whitelist: ManagedOption<ManagedVec<ManagedAddress>>,
        opt_burn_percentage: OptionalValue<BigUint>,
    ) {
        require!(!lottery_name.is_empty(), "Name can't be empty!");

        let timestamp = self.blockchain().get_block_timestamp_millis();
        let total_tickets = opt_total_tickets.unwrap_or(MAX_TICKETS);
        let deadline = opt_deadline.unwrap_or(timestamp + THIRTY_DAYS_IN_MILLISECONDS);
        let max_entries_per_user = opt_max_entries_per_user.unwrap_or(MAX_TICKETS);
        let prize_distribution = opt_prize_distribution
            .unwrap_or_else(|| ManagedVec::from_single_item(PERCENTAGE_TOTAL as u8));

        require!(
            total_tickets > prize_distribution.len(),
            "Number of winners should be smaller than the number of available tickets"
        );
        require!(
            self.status(&lottery_name) == Status::Inactive,
            "Lottery is already active!"
        );
        require!(
            token_identifier.is_valid_esdt_identifier(), // this also returns `false` if token is EGLD
            "Invalid token name provided!"
        );
        require!(ticket_price > 0, "Ticket price must be higher than 0!");
        require!(
            total_tickets > 0,
            "Must have more than 0 tickets available!"
        );
        require!(
            total_tickets <= MAX_TICKETS,
            "Only 800 or less total tickets per lottery are allowed!"
        );
        require!(deadline > timestamp, "Deadline can't be in the past!");
        require!(
            deadline <= timestamp + THIRTY_DAYS_IN_MILLISECONDS,
            "Deadline can't be later than 30 days from now!"
        );
        require!(
            max_entries_per_user > 0,
            "Must have more than 0 max entries per user!"
        );
        require!(
            self.sum_array(&prize_distribution) == PERCENTAGE_TOTAL,
            "Prize distribution must add up to exactly 100(%)!"
        );

        match opt_burn_percentage {
            OptionalValue::Some(burn_percentage) => {
                let roles = self
                    .blockchain()
                    .get_esdt_local_roles(&token_identifier.clone());
                require!(
                    roles.has_role(&EsdtLocalRole::Burn),
                    "The contract can't burn the selected token!"
                );

                require!(
                    burn_percentage < PERCENTAGE_TOTAL,
                    "Invalid burn percentage!"
                );
                self.burn_percentage_for_lottery(&lottery_name)
                    .set(burn_percentage);
            }
            OptionalValue::None => {}
        }

        if let Some(whitelist) = opt_whitelist.as_option() {
            let mut mapper = self.lottery_whitelist(&lottery_name);
            for addr in &*whitelist {
                let addr_id = self.address_to_id_mapper().get_id_or_insert(&addr);
                mapper.insert(addr_id);
            }
        }

        let info = LotteryInfo {
            token_identifier,
            ticket_price,
            tickets_left: total_tickets,
            deadline,
            max_entries_per_user,
            prize_distribution,
            prize_pool: BigUint::zero(),
            unawarded_amount: BigUint::zero(),
        };

        self.lottery_info(&lottery_name).set(&info);
    }
}
