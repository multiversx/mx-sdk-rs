use anyhow::{Result, anyhow};
use multiversx_sc_snippets::{
    InteractorEnvExec,
    imports::{
        BLSKey, Bech32Address, ExplicitGas, GatewayHttpProxy, Interactor,
        InteractorIntoSdkTransaction, InteractorRunAsync, MultiValueVec, ReturnsHandledOrError,
        ReturnsResult, ValidatorSCProxy, ValidatorSCProxyMethods, ValidatorSystemSCAddress,
    },
    sdk::data::transaction::Transaction,
};
use serde_json::json;

use super::system_common::{BlsKeysSignatures, biguint, resolve_bls_keys, system_proxy, system_tx};
use crate::cli::{
    cli_args_tx_system::SystemTxCommonArgs,
    cli_args_tx_system_validator::{
        ValidatorAddressQueryArgs, ValidatorCliAction, ValidatorCliArgs, ValidatorNodesArgs,
        ValidatorStakeArgs,
    },
};
use crate::cmd::tx_cli::tx_cli_common::{create_query_interactor, parse_address};

/// Default gas limit for validator operations, and per node for node operations (same as mxpy).
const VALIDATOR_GAS: u64 = 5_000_000;

/// The validator system SC proxy, on top of an interactor transaction.
type ValidatorProxy<'a> = ValidatorSCProxyMethods<
    InteractorEnvExec<'a, GatewayHttpProxy>,
    Bech32Address,
    ValidatorSystemSCAddress,
    ExplicitGas<u64>,
>;

pub(super) async fn tx_system_validator(args: &ValidatorCliArgs) -> Result<()> {
    match &args.command {
        ValidatorCliAction::Stake(a) => stake(a).await,
        ValidatorCliAction::StakeTopUp(a) => {
            let value = biguint(&a.value);
            validator_tx(&a.common, VALIDATOR_GAS, |validator| {
                validator.stake_top_up(value).into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::Unstake(a) => {
            nodes_tx(a, |validator, keys| {
                validator.unstake(keys).into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::UnstakeNodes(a) => {
            nodes_tx(a, |validator, keys| {
                validator.unstake_nodes(keys).into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::UnstakeTokens(a) => {
            let amount = biguint(&a.amount);
            validator_tx(&a.common, VALIDATOR_GAS, |validator| {
                validator.unstake_tokens(amount).into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::Unbond(a) => {
            nodes_tx(a, |validator, keys| {
                validator.unbond(keys).into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::UnbondNodes(a) => {
            nodes_tx(a, |validator, keys| {
                validator.unbond_nodes(keys).into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::UnbondTokens(a) => {
            let amount = biguint(&a.amount);
            validator_tx(&a.common, VALIDATOR_GAS, |validator| {
                validator.unbond_tokens(amount).into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::RestakeUnstakedNodes(a) => {
            nodes_tx(a, |validator, keys| {
                validator
                    .restake_unstaked_nodes(keys)
                    .into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::Unjail(a) => {
            let keys = resolve_bls_keys(&a.keys)?;
            let gas = VALIDATOR_GAS * keys.len() as u64;
            let unjail_price_per_node = biguint(&a.unjail_price_per_node);
            validator_tx(&a.common, gas, |validator| {
                validator
                    .unjail(keys, unjail_price_per_node)
                    .into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::ChangeRewardAddress(a) => {
            let reward_address = parse_address("--reward-address", &a.reward_address)?;
            validator_tx(&a.common, VALIDATOR_GAS, |validator| {
                validator
                    .change_reward_address(&reward_address)
                    .into_sdk_transaction()
            })
            .await
        }
        ValidatorCliAction::CleanRegisteredData(a) => {
            validator_tx(&a.common, VALIDATOR_GAS, |validator| {
                validator.clean_registered_data().into_sdk_transaction()
            })
            .await
        }

        // --- queries ---
        ValidatorCliAction::TotalStaked(a) => query_total_staked(a).await,
        ValidatorCliAction::TotalStakedTopUp(a) => query_total_staked_top_up(a).await,
        ValidatorCliAction::UnstakedTokens(a) => query_unstaked_tokens(a).await,
        ValidatorCliAction::BlsKeysStatus(a) => query_bls_keys_status(a).await,
    }
}

// ---------------------------------------------------------------------------
// Transactions
// ---------------------------------------------------------------------------

/// Starts a transaction from `sender` to the validator system SC, through the typed proxy.
fn validator_proxy<'a>(
    interactor: &'a mut Interactor,
    sender: &Bech32Address,
    relayer: Option<Bech32Address>,
    gas_limit: u64,
) -> ValidatorProxy<'a> {
    system_proxy(
        interactor,
        sender,
        ValidatorSystemSCAddress,
        relayer,
        gas_limit,
        ValidatorSCProxy,
    )
}

/// Builds a transaction with the validator system SC proxy, then signs and dispatches it.
async fn validator_tx<F>(
    common: &SystemTxCommonArgs,
    default_gas_limit: u64,
    build: F,
) -> Result<()>
where
    F: FnOnce(ValidatorProxy<'_>) -> Transaction,
{
    system_tx(
        common,
        default_gas_limit,
        async move |interactor, sender, relayer, gas_limit| {
            Ok(build(validator_proxy(
                interactor, &sender, relayer, gas_limit,
            )))
        },
    )
    .await
}

/// Node operation: the default gas limit depends on the number of nodes.
async fn nodes_tx<F>(args: &ValidatorNodesArgs, build: F) -> Result<()>
where
    F: FnOnce(ValidatorProxy<'_>, MultiValueVec<BLSKey>) -> Transaction,
{
    let keys = resolve_bls_keys(&args.keys)?;
    let gas = VALIDATOR_GAS * keys.len() as u64;
    validator_tx(&args.common, gas, |validator| build(validator, keys)).await
}

/// Stakes nodes: each BLS key signs the sender address.
async fn stake(args: &ValidatorStakeArgs) -> Result<()> {
    let keys = BlsKeysSignatures::from_args(&args.keys)?;
    let reward_address = args
        .reward_address
        .as_ref()
        .map(|address| parse_address("--reward-address", address))
        .transpose()?;
    let value = biguint(&args.value);
    let gas = VALIDATOR_GAS * keys.len() as u64;

    system_tx(
        &args.common,
        gas,
        async move |interactor, sender, relayer, gas_limit| {
            // the signatures need the sender address, known only once the wallet is loaded
            let pairs = keys.into_pairs(sender.as_address())?;
            let validator = validator_proxy(interactor, &sender, relayer, gas_limit);
            let tx = match &reward_address {
                Some(reward_address) => validator
                    .stake_with_reward_address(pairs, reward_address, value)
                    .into_sdk_transaction(),
                None => validator.stake(pairs, value).into_sdk_transaction(),
            };
            Ok(tx)
        },
    )
    .await
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

async fn query_total_staked(args: &ValidatorAddressQueryArgs) -> Result<()> {
    let address = parse_address("address", &args.address)?;
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let total_staked = interactor
        .query()
        .to(ValidatorSystemSCAddress)
        .typed(ValidatorSCProxy)
        .get_total_staked(&address)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("total staked query failed: {}", err.message))?;

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            // already a base 10 string
            "totalStaked": total_staked.to_string(),
        }))?
    );
    Ok(())
}

async fn query_total_staked_top_up(args: &ValidatorAddressQueryArgs) -> Result<()> {
    let address = parse_address("address", &args.address)?;
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let (top_up, total_staked, num_active_nodes, bls_keys) = interactor
        .query()
        .to(ValidatorSystemSCAddress)
        .typed(ValidatorSCProxy)
        .get_total_staked_top_up_staked_bls_keys(&address)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("total staked top-up query failed: {}", err.message))?
        .into_tuple();

    let bls_keys: Vec<String> = bls_keys
        .into_iter()
        .map(|key| hex::encode(key.to_vec()))
        .collect();
    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "topUp": top_up.to_display().to_string(),
            "totalStaked": total_staked.to_display().to_string(),
            "numActiveNodes": num_active_nodes,
            "blsKeys": bls_keys,
        }))?
    );
    Ok(())
}

async fn query_unstaked_tokens(args: &ValidatorAddressQueryArgs) -> Result<()> {
    let address = parse_address("address", &args.address)?;
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let unstaked = interactor
        .query()
        .to(ValidatorSystemSCAddress)
        .typed(ValidatorSCProxy)
        .get_unstaked_tokens_list(&address)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("unstaked tokens query failed: {}", err.message))?;

    let unstaked: Vec<serde_json::Value> = unstaked
        .into_iter()
        .map(|entry| {
            let (amount, remaining_epochs) = entry.into_tuple();
            json!({
                "amount": amount.to_display().to_string(),
                "remainingEpochs": remaining_epochs,
            })
        })
        .collect();
    println!("{}", Interactor::to_json_pretty(&unstaked)?);
    Ok(())
}

async fn query_bls_keys_status(args: &ValidatorAddressQueryArgs) -> Result<()> {
    let address = parse_address("address", &args.address)?;
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let statuses = interactor
        .query()
        .to(ValidatorSystemSCAddress)
        .typed(ValidatorSCProxy)
        .get_bls_keys_status(&address)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("BLS keys status query failed: {}", err.message))?;

    let statuses: Vec<serde_json::Value> = statuses
        .into_iter()
        .map(|entry| {
            let (key, status) = entry.into_tuple();
            json!({
                "blsKey": hex::encode(key.to_vec()),
                "status": status.to_string(),
            })
        })
        .collect();
    println!("{}", Interactor::to_json_pretty(&statuses)?);
    Ok(())
}
