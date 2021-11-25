#![no_std]

elrond_wasm::imports!();

mod user_status;

use user_status::UserStatus;

/// Derived empirically.
const PONG_ALL_LOW_GAS_LIMIT: u64 = 3_000_000;

/// A contract that allows anyone to send a fixed sum, locks it for a while and then allows users to take it back.
/// Sending funds to the contract is called "ping".
/// Taking the same funds back is called "pong".
///
/// Restrictions:
/// - `ping` can be called only after the contract is activated. By default the contract is activated on deploy.
/// - Users can only `ping` once, ever.
/// - Only the set amount can be `ping`-ed, no more, no less.
/// - The contract can optionally have a maximum cap. No more users can `ping` after the cap has been reached.
/// - The `ping` endpoint optionally accepts
/// - `pong` can only be called after the contract expired (a certain duration has passed since activation).
/// - `pongAll` can be used to send to all users to `ping`-ed. If it runs low on gas, it will interrupt itself.
/// It can be continued anytime.
#[elrond_wasm::contract]
pub trait PingPong {
    /// Necessary configuration when deploying:
    /// `ping_amount` - the exact EGLD amounf that needs to be sent when `ping`-ing.
    /// `duration_in_seconds` - how much time (in seconds) until contract expires.
    /// `opt_activation_timestamp` - optionally specify the contract to only actvivate at a later date.
    /// `max_funds` - optional funding cap, no more funds than this can be added to the contract.
    #[init]
    fn init(
        &self,
        ping_amount: &BigUint,
        duration_in_seconds: u64,
        opt_activation_timestamp: Option<u64>,
        #[var_args] max_funds: OptionalArg<BigUint>,
    ) {
        self.ping_amount().set(ping_amount);
        let activation_timestamp =
            opt_activation_timestamp.unwrap_or_else(|| self.blockchain().get_block_timestamp());
        let deadline = activation_timestamp + duration_in_seconds;
        self.deadline().set(&deadline);
        self.activation_timestamp().set(&activation_timestamp);
        self.max_funds().set(&max_funds.into_option());
    }

    /// User sends some EGLD to be locked in the contract for a period of time.
    /// Optional `_data` argument is ignored.
    #[payable("EGLD")]
    #[endpoint]
    fn ping(
        &self,
        #[payment] payment: BigUint,
        #[var_args] _data: OptionalArg<BoxedBytes>,
    ) -> SCResult<()> {
        require!(
            payment == self.ping_amount().get(),
            "the payment must match the fixed sum"
        );

        let block_timestamp = self.blockchain().get_block_timestamp();
        require!(
            self.activation_timestamp().get() <= block_timestamp,
            "smart contract not active yet"
        );

        require!(
            block_timestamp < self.deadline().get(),
            "deadline has passed"
        );

        if let Some(max_funds) = self.max_funds().get() {
            require!(
                &self
                    .blockchain()
                    .get_sc_balance(&TokenIdentifier::egld(), 0)
                    + &payment
                    <= max_funds,
                "smart contract full"
            );
        }

        let caller = self.blockchain().get_caller();
        let user_id = self.user_mapper().get_or_create_user(&caller);
        let user_status = self.user_status(user_id).get();
        match user_status {
            UserStatus::New => {
                self.user_status(user_id).set(&UserStatus::Registered);
                Ok(())
            },
            UserStatus::Registered => {
                sc_error!("can only ping once")
            },
            UserStatus::Withdrawn => {
                sc_error!("already withdrawn")
            },
        }
    }

    fn pong_by_user_id(&self, user_id: usize) -> SCResult<()> {
        let user_status = self.user_status(user_id).get();
        match user_status {
            UserStatus::New => {
                sc_error!("can't pong, never pinged")
            },
            UserStatus::Registered => {
                self.user_status(user_id).set(&UserStatus::Withdrawn);
                if let Some(user_address) = self.user_mapper().get_user_address(user_id) {
                    self.send()
                        .direct_egld(&user_address, &self.ping_amount().get(), b"pong");
                    Ok(())
                } else {
                    sc_error!("unknown user")
                }
            },
            UserStatus::Withdrawn => {
                sc_error!("already withdrawn")
            },
        }
    }

    /// User can take back funds from the contract.
    /// Can only be called after expiration.
    #[endpoint]
    fn pong(&self) -> SCResult<()> {
        require!(
            self.blockchain().get_block_timestamp() >= self.deadline().get(),
            "can't withdraw before deadline"
        );

        let caller = self.blockchain().get_caller();
        let user_id = self.user_mapper().get_user_id(&caller);
        self.pong_by_user_id(user_id)
    }

    /// Send back funds to all users who pinged.
    /// Returns
    /// - `completed` if everything finished
    /// - `interrupted` if run out of gas midway.
    /// Can only be called after expiration.
    #[endpoint(pongAll)]
    fn pong_all(&self) -> SCResult<OperationCompletionStatus> {
        require!(
            self.blockchain().get_block_timestamp() >= self.deadline().get(),
            "can't withdraw before deadline"
        );

        let num_users = self.user_mapper().get_user_count();
        let mut pong_all_last_user = self.pong_all_last_user().get();
        loop {
            if pong_all_last_user >= num_users {
                // clear field and reset to 0
                pong_all_last_user = 0;
                self.pong_all_last_user().set(&pong_all_last_user);
                return Ok(OperationCompletionStatus::Completed);
            }

            if self.blockchain().get_gas_left() < PONG_ALL_LOW_GAS_LIMIT {
                self.pong_all_last_user().set(&pong_all_last_user);
                return Ok(OperationCompletionStatus::InterruptedBeforeOutOfGas);
            }

            pong_all_last_user += 1;
            let _ = self.pong_by_user_id(pong_all_last_user);
        }
    }

    /// Lists the addresses of all users that have `ping`-ed,
    /// in the order they have `ping`-ed
    #[view(getUserAddresses)]
    fn get_user_addresses(&self) -> ManagedMultiResultVec<ManagedAddress> {
        self.user_mapper().get_all_addresses().into()
    }

    // storage

    #[view(getPingAmount)]
    #[storage_mapper("pingAmount")]
    fn ping_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    /// Block timestamp of the block where the contract got activated.
    /// If not specified in the constructor it is the the deploy block timestamp.
    #[view(getActivationTimestamp)]
    #[storage_mapper("activationTimestamp")]
    fn activation_timestamp(&self) -> SingleValueMapper<u64>;

    /// Optional funding cap.
    #[view(getMaxFunds)]
    #[storage_mapper("maxFunds")]
    fn max_funds(&self) -> SingleValueMapper<Option<BigUint>>;

    #[storage_mapper("user")]
    fn user_mapper(&self) -> UserMapper;

    /// State of user funds.
    /// 0 - user unknown, never `ping`-ed
    /// 1 - `ping`-ed
    /// 2 - `pong`-ed
    #[view(getUserStatus)]
    #[storage_mapper("userStatus")]
    fn user_status(&self, user_id: usize) -> SingleValueMapper<UserStatus>;

    /// Part of the `pongAll` status, the last user to be processed.
    /// 0 if never called `pongAll` or `pongAll` completed..
    #[view(pongAllLastUser)]
    #[storage_mapper("pongAllLastUser")]
    fn pong_all_last_user(&self) -> SingleValueMapper<usize>;
}
