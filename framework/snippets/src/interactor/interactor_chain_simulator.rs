use std::collections::HashMap;

use anyhow::Error;
use multiversx_sc_scenario::imports::{Bech32Address, RustBigUint};
use multiversx_sdk::gateway::{
    ChainSimulatorAddKeysRequest, ChainSimulatorGenerateBlocksRequest,
    ChainSimulatorSendFundsRequest, ChainSimulatorSetStateOverwriteRequest,
    ChainSimulatorSetStateRequest, GatewayAsyncService, SetStateAccount,
};

use crate::InteractorBase;

pub const ALL_ACTIVATIONS_EPOCH: u64 = 2;

/// Storage key under which the protocol accumulates the validator rewards an account has
/// received. `core.ProtectedKeyPrefix + "reward"` on the protocol side.
///
/// It is a protected key: no contract can write it, and a contract reads its own through
/// the `getCumulatedValidatorRewards` VM hook. The chain simulator's `set-state` endpoint
/// writes account storage directly, which is what makes
/// [`InteractorBase::inject_validator_reward`] possible.
pub const VALIDATOR_REWARD_KEY: &str = "ELRONDreward";

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    /// Send funds to a user account on the chain simulator. The amount is fixed to 100 EGLD.
    pub async fn send_user_funds(&self, receiver: &Bech32Address) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy()
            .request(ChainSimulatorSendFundsRequest::to_address(receiver))
            .await
    }

    pub async fn generate_blocks(&self, num_blocks: u64) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy()
            .request(ChainSimulatorGenerateBlocksRequest::num_blocks(num_blocks))
            .await
    }

    pub async fn add_key(&self, key: Vec<u8>) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy()
            .request(ChainSimulatorAddKeysRequest::with_keys(vec![key]))
            .await
    }

    pub async fn generate_blocks_until_epoch(&self, epoch_number: u64) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy()
            .request(ChainSimulatorGenerateBlocksRequest::until_epoch(
                epoch_number,
            ))
            .await
    }

    /// Convenience method to wait enough blocks until all relevant activations are processed.
    pub async fn generate_blocks_until_all_activations(&self) {
        let _ = self
            .generate_blocks_until_epoch(ALL_ACTIVATIONS_EPOCH)
            .await
            .expect("Failed to generate blocks until all activations");
    }

    pub async fn generate_blocks_until_tx_processed(&self, tx_hash: &str) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy()
            .request(ChainSimulatorGenerateBlocksRequest::until_tx_processed(
                tx_hash,
            ))
            .await
    }

    pub async fn set_state(&self, accounts: Vec<SetStateAccount>) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy()
            .request(ChainSimulatorSetStateRequest::for_accounts(accounts))
            .await
    }

    pub async fn set_state_overwrite(
        &self,
        accounts: Vec<SetStateAccount>,
    ) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy()
            .request(ChainSimulatorSetStateOverwriteRequest::for_accounts(
                accounts,
            ))
            .await
    }

    pub async fn set_state_for_saved_accounts(&self) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        let accounts = self.get_accounts_from_file();
        self.proxy()
            .request(ChainSimulatorSetStateRequest::for_accounts(accounts))
            .await
    }

    /// The validator rewards an account has accumulated so far, as the protocol records
    /// them under [`VALIDATOR_REWARD_KEY`]. Zero if the key was never written.
    ///
    /// This is a plain storage read, so it also works against a real chain.
    pub async fn get_cumulated_validator_rewards(&self, address: &Bech32Address) -> RustBigUint {
        let storage = self.get_account_storage(address.as_address()).await;

        let Some(value_hex) = storage.get(&hex::encode(VALIDATOR_REWARD_KEY)) else {
            return RustBigUint::default();
        };

        let bytes = hex::decode(value_hex).unwrap_or_else(|_| {
            panic!("gateway returned a non-hex value for the validator reward key: '{value_hex}'")
        });
        RustBigUint::from_bytes_be(&bytes)
    }

    /// Simulates a validator reward arriving at `address`, on the chain simulator.
    ///
    /// Does what the protocol's reward transaction does: credits the balance, and adds the
    /// same amount to the account's cumulated reward counter. A contract reading
    /// `getCumulatedValidatorRewards` cannot tell the difference, which is what makes this
    /// usable for testing staking providers and other reward-splitting contracts without
    /// waiting for - or arranging - real block rewards.
    ///
    /// The account's nonce is read and written back unchanged, because `set-state` would
    /// otherwise reset it to zero. Code and the remaining fields are left empty, which the
    /// simulator treats as "leave as is".
    ///
    /// Does nothing if the interactor is not connected to a chain simulator.
    pub async fn inject_validator_reward(
        &self,
        address: &Bech32Address,
        amount: &RustBigUint,
    ) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        let account = self.get_account(address.as_address()).await;
        let new_balance = parse_balance(&account.balance) + amount;
        let new_cumulated = self.get_cumulated_validator_rewards(address).await + amount;

        let pairs = HashMap::from([(
            hex::encode(VALIDATOR_REWARD_KEY),
            hex::encode(new_cumulated.to_bytes_be()),
        )]);

        let mut set_state_account =
            SetStateAccount::from_address(address.to_bech32_string()).with_storage(pairs);
        set_state_account.nonce = account.nonce;
        set_state_account.balance = new_balance.to_string();

        self.set_state(vec![set_state_account]).await
    }
}

/// Parses a balance as the gateway reports it: a decimal string, possibly empty.
fn parse_balance(balance: &str) -> RustBigUint {
    if balance.is_empty() {
        return RustBigUint::default();
    }

    balance
        .parse()
        .unwrap_or_else(|_| panic!("unexpected balance value from gateway: '{balance}'"))
}
