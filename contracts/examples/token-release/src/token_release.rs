#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod contract_data;

use contract_data::{Schedule, UnlockType};

const PERCENTAGE_TOTAL: u64 = 100;

#[multiversx_sc::contract]
pub trait TokenRelease {
    // The SC initializes with the setup period started. After the initial setup, the SC offers a function that ends the setup period.
    // There is no function to start the setup period back on, so once the setup period is ended, it cannot be changed.
    #[init]
    fn init(&self, token_identifier: TokenIdentifier) {
        require!(
            token_identifier.is_valid_esdt_identifier(),
            "Invalid token provided"
        );
        self.token_identifier().set(&token_identifier);
        self.setup_period_status().set(true);
    }

    // endpoints

    // Workflow
    // First, all groups are defined. After that, an address can be assigned as many groups as needed
    #[only_owner]
    #[endpoint(addFixedAmountGroup)]
    fn add_fixed_amount_group(
        &self,
        group_identifier: ManagedBuffer,
        group_total_amount: BigUint,
        period_unlock_amount: BigUint,
        release_period: u64,
        release_ticks: u64,
    ) {
        self.require_setup_period_live();
        require!(
            self.group_schedule(&group_identifier).is_empty(),
            "The group already exists"
        );
        require!(
            release_ticks > 0u64,
            "The schedule must have at least 1 unlock period"
        );
        require!(
            group_total_amount > BigUint::zero(),
            "The schedule must have a positive number of total tokens released"
        );
        require!(
            &period_unlock_amount * &BigUint::from(release_ticks) == group_total_amount,
            "The total number of tokens is invalid"
        );

        self.token_total_supply()
            .update(|total| *total += &group_total_amount);
        let unlock_type = UnlockType::FixedAmount {
            period_unlock_amount,
            release_period,
            release_ticks,
        };
        let new_schedule = Schedule {
            group_total_amount,
            unlock_type,
        };
        self.group_schedule(&group_identifier).set(&new_schedule);
    }

    #[only_owner]
    #[endpoint(addPercentageBasedGroup)]
    fn add_percentage_based_group(
        &self,
        group_identifier: ManagedBuffer,
        group_total_amount: BigUint,
        period_unlock_percentage: u8,
        release_period: u64,
        release_ticks: u64,
    ) {
        self.require_setup_period_live();
        require!(
            self.group_schedule(&group_identifier).is_empty(),
            "The group already exists"
        );
        require!(
            release_ticks > 0_u64,
            "The schedule must have at least 1 unlock period"
        );
        require!(
            group_total_amount > BigUint::zero(),
            "The schedule must have a positive number of total tokens released"
        );
        require!(
            (period_unlock_percentage as u64) * release_ticks == PERCENTAGE_TOTAL,
            "The final percentage is invalid"
        );

        self.token_total_supply()
            .update(|total| *total += &group_total_amount);
        let unlock_type = UnlockType::Percentage {
            period_unlock_percentage,
            release_period,
            release_ticks,
        };
        let new_schedule = Schedule {
            group_total_amount,
            unlock_type,
        };
        self.group_schedule(&group_identifier).set(&new_schedule);
    }

    #[only_owner]
    #[endpoint(removeGroup)]
    fn remove_group(&self, group_identifier: ManagedBuffer) {
        self.require_setup_period_live();
        require!(
            !self.group_schedule(&group_identifier).is_empty(),
            "The group does not exist"
        );

        let schedule = self.group_schedule(&group_identifier).get();
        self.token_total_supply()
            .update(|total| *total -= &schedule.group_total_amount);
        self.group_schedule(&group_identifier).clear();
        self.users_in_group(&group_identifier).clear();
    }

    #[only_owner]
    #[endpoint(addUserGroup)]
    fn add_user_group(&self, address: ManagedAddress, group_identifier: ManagedBuffer) {
        self.require_setup_period_live();
        require!(
            !self.group_schedule(&group_identifier).is_empty(),
            "The group does not exist"
        );

        self.user_groups(&address).update(|groups| {
            let mut group_exists = false;
            for group in groups.iter() {
                if group == group_identifier.as_ref() {
                    group_exists = true;
                    break;
                }
            }
            if !group_exists {
                self.users_in_group(&group_identifier)
                    .update(|users_in_group_no| *users_in_group_no += 1);
                groups.push(group_identifier);
            }
        });
    }

    #[only_owner]
    #[endpoint(removeUser)]
    fn remove_user(&self, address: ManagedAddress) {
        self.require_setup_period_live();
        require!(
            !self.user_groups(&address).is_empty(),
            "The address is not defined"
        );
        let address_groups = self.user_groups(&address).get();
        for group_identifier in address_groups.iter() {
            self.users_in_group(&group_identifier)
                .update(|users_in_group_no| *users_in_group_no -= 1);
        }
        self.user_groups(&address).clear();
        self.claimed_balance(&address).clear();
    }

    //To change a receiving address, the user registers a request, which is afterwards accepted or not by the owner
    #[endpoint(requestAddressChange)]
    fn request_address_change(&self, new_address: ManagedAddress) {
        self.require_setup_period_ended();
        let user_address = self.blockchain().get_caller();
        self.address_change_request(&user_address).set(&new_address);
    }

    #[only_owner]
    #[endpoint(approveAddressChange)]
    fn approve_address_change(&self, user_address: ManagedAddress) {
        self.require_setup_period_ended();
        require!(
            !self.address_change_request(&user_address).is_empty(),
            "The address does not have a change request"
        );

        // Get old address values
        let new_address = self.address_change_request(&user_address).get();
        let user_current_groups = self.user_groups(&user_address).get();
        let user_claimed_balance = self.claimed_balance(&user_address).get();

        // Save the new address with the old address values
        self.user_groups(&new_address).set(&user_current_groups);
        self.claimed_balance(&new_address)
            .set(&user_claimed_balance);

        // Delete the old address
        self.user_groups(&user_address).clear();
        self.claimed_balance(&user_address).clear();

        // Delete the change request
        self.address_change_request(&user_address).clear();
    }

    #[only_owner]
    #[endpoint(endSetupPeriod)]
    fn end_setup_period(&self) {
        self.require_setup_period_live();
        let token_identifier = self.token_identifier().get();
        let total_mint_tokens = self.token_total_supply().get();
        self.mint_all_tokens(&token_identifier, &total_mint_tokens);
        let activation_timestamp = self.blockchain().get_block_timestamp();
        self.activation_timestamp().set(activation_timestamp);
        self.setup_period_status().set(false);
    }

    #[endpoint(claimTokens)]
    fn claim_tokens(&self) -> BigUint {
        self.require_setup_period_ended();
        let token_identifier = self.token_identifier().get();
        let caller = self.blockchain().get_caller();
        let current_claimable_amount = self.get_claimable_tokens(&caller);

        require!(
            current_claimable_amount > BigUint::zero(),
            "This address cannot currently claim any more tokens"
        );
        self.send_tokens(&token_identifier, &caller, &current_claimable_amount);
        self.claimed_balance(&caller)
            .update(|current_balance| *current_balance += &current_claimable_amount);

        current_claimable_amount
    }

    // views

    #[view]
    fn verify_address_change(&self, address: &ManagedAddress) -> ManagedAddress {
        self.address_change_request(address).get()
    }

    #[view]
    fn get_claimable_tokens(&self, address: &ManagedAddress) -> BigUint {
        let total_claimable_amount = self.calculate_claimable_tokens(address);
        let current_balance = self.claimed_balance(address).get();
        if total_claimable_amount > current_balance {
            total_claimable_amount - current_balance
        } else {
            BigUint::zero()
        }
    }

    // private functions

    fn calculate_claimable_tokens(&self, address: &ManagedAddress) -> BigUint {
        let starting_timestamp = self.activation_timestamp().get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let address_groups = self.user_groups(address).get();

        let mut claimable_amount = BigUint::zero();

        // Compute the total claimable amount at the time of the request, for all of the user groups
        for group_identifier in address_groups.iter() {
            let schedule = self.group_schedule(&group_identifier).get();
            let users_in_group_no = self.users_in_group(&group_identifier).get();
            let time_passed = current_timestamp - starting_timestamp;

            match schedule.unlock_type {
                UnlockType::FixedAmount {
                    period_unlock_amount,
                    release_period,
                    release_ticks,
                } => {
                    let mut periods_passed = time_passed / release_period;
                    if periods_passed == 0 {
                        continue;
                    }
                    if periods_passed > release_ticks {
                        periods_passed = release_ticks;
                    }
                    claimable_amount += BigUint::from(periods_passed) * period_unlock_amount
                        / BigUint::from(users_in_group_no);
                },
                UnlockType::Percentage {
                    period_unlock_percentage,
                    release_period,
                    release_ticks,
                } => {
                    let mut periods_passed = time_passed / release_period;
                    if periods_passed == 0 {
                        continue;
                    }
                    if periods_passed > release_ticks {
                        periods_passed = release_ticks;
                    }
                    claimable_amount += BigUint::from(periods_passed)
                        * &schedule.group_total_amount
                        * (period_unlock_percentage as u64)
                        / PERCENTAGE_TOTAL
                        / BigUint::from(users_in_group_no);
                },
            }
        }

        claimable_amount
    }

    fn send_tokens(
        &self,
        token_identifier: &TokenIdentifier,
        address: &ManagedAddress,
        amount: &BigUint,
    ) {
        self.send()
            .direct_esdt(address, token_identifier, 0, amount);
    }

    fn mint_all_tokens(&self, token_identifier: &TokenIdentifier, amount: &BigUint) {
        self.send().esdt_local_mint(token_identifier, 0, amount);
    }

    fn require_setup_period_live(&self) {
        require!(self.setup_period_status().get(), "Setup period has ended");
    }

    fn require_setup_period_ended(&self) {
        require!(
            !(self.setup_period_status().get()),
            "Setup period is still active"
        );
    }

    // storage
    #[storage_mapper("activationTimestamp")]
    fn activation_timestamp(&self) -> SingleValueMapper<u64>;

    #[view(getTokenIdentifier)]
    #[storage_mapper("tokenIdentifier")]
    fn token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTokenTotalSupply)]
    #[storage_mapper("tokenTotalSupply")]
    fn token_total_supply(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("setupPeriodStatus")]
    fn setup_period_status(&self) -> SingleValueMapper<bool>;

    #[storage_mapper("addressChangeRequest")]
    fn address_change_request(&self, address: &ManagedAddress)
        -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("groupSchedule")]
    fn group_schedule(
        &self,
        group_identifier: &ManagedBuffer,
    ) -> SingleValueMapper<Schedule<Self::Api>>;

    #[storage_mapper("userGroups")]
    fn user_groups(&self, address: &ManagedAddress)
        -> SingleValueMapper<ManagedVec<ManagedBuffer>>;

    #[storage_mapper("usersInGroup")]
    fn users_in_group(&self, group_identifier: &ManagedBuffer) -> SingleValueMapper<u64>;

    #[storage_mapper("claimedBalance")]
    fn claimed_balance(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
