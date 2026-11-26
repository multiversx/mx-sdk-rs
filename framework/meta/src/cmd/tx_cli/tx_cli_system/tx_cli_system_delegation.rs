use anyhow::{Result, anyhow};
use multiversx_sc_snippets::{
    InteractorEnvExec,
    imports::{
        Bech32Address, BigUint, DelegationManagerSCAddress, DelegationManagerSCProxy,
        DelegationManagerSCProxyMethods, DelegationSCProxy, DelegationSCProxyMethods, ExplicitGas,
        GatewayHttpProxy, Interactor, InteractorIntoSdkTransaction, InteractorRunAsync,
        ManagedAddress, MultiValueEncoded, ReturnsHandledOrError, ReturnsResult, RustBigUint,
        StaticApi,
    },
    sdk::data::transaction::Transaction,
};
use serde_json::json;

use super::system_common::{
    BlsKeysSignatures, bech32, biguint, resolve_bls_keys, system_proxy, system_tx,
};
use crate::cli::{
    cli_args_tx_system::{SystemQueryArgs, SystemTxCommonArgs},
    cli_args_tx_system_delegation::{
        DelegationAddNodesArgs, DelegationCliAction, DelegationCliArgs,
        DelegationDelegatorQueryArgs, DelegationManagerCliAction, DelegationManagerCliArgs,
        DelegationManagerCreateArgs, DelegationManagerMultiArgs, DelegationNodesArgs,
        DelegationRewardDataQueryArgs,
    },
};
use crate::cmd::tx_cli::tx_cli_common::{create_query_interactor, parse_address};

/// Default gas limit for creating a delegation contract (same as mxpy).
const CREATE_DELEGATION_CONTRACT_GAS: u64 = 60_000_000;

/// Default gas limit for converting a validator into a delegation contract (same as mxpy).
const NEW_CONTRACT_FROM_VALIDATOR_GAS: u64 = 51_000_000;

/// Default gas limit for merging a validator into a delegation contract (same as mxpy).
const MERGE_VALIDATOR_GAS: u64 = 50_000_000;

/// Default gas limit for regular delegation operations (same as mxpy).
const DELEGATION_OPS_GAS: u64 = 11_000_000;

/// Base gas limit for node operations (same as mxpy).
const NODE_OPS_BASE_GAS: u64 = 1_000_000;

/// Additional gas limit per node, for node operations (same as mxpy).
const NODE_OPS_GAS_PER_NODE: u64 = 6_000_000;

/// Additional gas limit for operations calling the validator SC: stake, unstake, unbond (same as mxpy).
const NODE_STAKING_EXTRA_GAS: u64 = 5_000_000;

/// The delegation manager system SC proxy, on top of an interactor transaction.
type DelegationManagerProxy<'a> = DelegationManagerSCProxyMethods<
    InteractorEnvExec<'a, GatewayHttpProxy>,
    Bech32Address,
    DelegationManagerSCAddress,
    ExplicitGas<u64>,
>;

/// The delegation contract proxy, on top of an interactor transaction.
type DelegationProxy<'a> = DelegationSCProxyMethods<
    InteractorEnvExec<'a, GatewayHttpProxy>,
    Bech32Address,
    Bech32Address,
    ExplicitGas<u64>,
>;

// ---------------------------------------------------------------------------
// Delegation manager
// ---------------------------------------------------------------------------

pub(super) async fn tx_system_delegation_manager(args: &DelegationManagerCliArgs) -> Result<()> {
    match &args.command {
        DelegationManagerCliAction::CreateNewDelegationContract(a) => {
            create_new_delegation_contract(a).await
        }
        DelegationManagerCliAction::MakeNewContractFromValidatorData(a) => {
            let max_delegation_cap = biguint(&a.max_delegation_cap);
            delegation_manager_tx(&a.common, NEW_CONTRACT_FROM_VALIDATOR_GAS, |manager| {
                manager
                    .make_new_contract_from_validator_data(max_delegation_cap, a.service_fee)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationManagerCliAction::MergeValidatorToDelegationSameOwner(a) => {
            let contract = parse_address("--delegation-contract", &a.delegation_contract)?;
            delegation_manager_tx(&a.common, MERGE_VALIDATOR_GAS, |manager| {
                manager
                    .merge_validator_to_delegation_same_owner(&contract)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationManagerCliAction::MergeValidatorToDelegationWithWhitelist(a) => {
            let contract = parse_address("--delegation-contract", &a.delegation_contract)?;
            delegation_manager_tx(&a.common, MERGE_VALIDATOR_GAS, |manager| {
                manager
                    .merge_validator_to_delegation_with_whitelist(&contract)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationManagerCliAction::ClaimMulti(a) => {
            let (contracts, gas) = parse_multi_args(a)?;
            delegation_manager_tx(&a.common, gas, |manager| {
                manager.claim_multi(contracts).into_sdk_transaction()
            })
            .await
        }
        DelegationManagerCliAction::RedelegateMulti(a) => {
            let (contracts, gas) = parse_multi_args(a)?;
            delegation_manager_tx(&a.common, gas, |manager| {
                manager.redelegate_multi(contracts).into_sdk_transaction()
            })
            .await
        }
        DelegationManagerCliAction::ContractConfig(a) => query_manager_contract_config(a).await,
        DelegationManagerCliAction::ContractAddresses(a) => query_contract_addresses(a).await,
    }
}

/// Starts a transaction from `sender` to the delegation manager system SC, through the typed proxy.
fn delegation_manager_proxy<'a>(
    interactor: &'a mut Interactor,
    sender: &Bech32Address,
    relayer: Option<Bech32Address>,
    gas_limit: u64,
) -> DelegationManagerProxy<'a> {
    system_proxy(
        interactor,
        sender,
        DelegationManagerSCAddress,
        relayer,
        gas_limit,
        DelegationManagerSCProxy,
    )
}

/// Builds a transaction with the delegation manager proxy, then signs and dispatches it.
async fn delegation_manager_tx<F>(
    common: &SystemTxCommonArgs,
    default_gas_limit: u64,
    build: F,
) -> Result<()>
where
    F: FnOnce(DelegationManagerProxy<'_>) -> Transaction,
{
    system_tx(
        common,
        default_gas_limit,
        async move |interactor, sender, relayer, gas_limit| {
            Ok(build(delegation_manager_proxy(
                interactor, &sender, relayer, gas_limit,
            )))
        },
    )
    .await
}

async fn create_new_delegation_contract(args: &DelegationManagerCreateArgs) -> Result<()> {
    let total_delegation_cap = biguint(&args.total_delegation_cap);
    system_tx(
        &args.common,
        CREATE_DELEGATION_CONTRACT_GAS,
        async move |interactor, sender, relayer, gas_limit| {
            let value = resolve_min_deposit(interactor, &args.value).await?;
            Ok(
                delegation_manager_proxy(interactor, &sender, relayer, gas_limit)
                    .create_new_delegation_contract(total_delegation_cap, args.service_fee, value)
                    .into_sdk_transaction(),
            )
        },
    )
    .await
}

/// Resolves the initial deposit: either explicitly given, or the minimum deposit from the delegation manager configuration.
async fn resolve_min_deposit(
    interactor: &mut Interactor,
    value: &Option<RustBigUint>,
) -> Result<BigUint<StaticApi>> {
    match value {
        Some(value) => Ok(biguint(value)),
        None => {
            let config = fetch_manager_contract_config(interactor).await?;
            Ok(config.min_deposit)
        }
    }
}

fn parse_multi_args(
    args: &DelegationManagerMultiArgs,
) -> Result<(MultiValueEncoded<StaticApi, ManagedAddress<StaticApi>>, u64)> {
    let contracts = args
        .delegation_contracts
        .iter()
        .map(|contract| parse_address("--delegation-contracts", contract))
        .collect::<Result<Vec<_>>>()?;
    let gas = DELEGATION_OPS_GAS * contracts.len() as u64;
    let contracts = contracts
        .iter()
        .map(|contract| ManagedAddress::from_address(contract.as_address()))
        .collect();
    Ok((contracts, gas))
}

struct ManagerContractConfig {
    num_contracts: u64,
    last_contract: ManagedAddress<StaticApi>,
    min_service_fee: u64,
    max_service_fee: u64,
    min_deposit: BigUint<StaticApi>,
    min_delegation_amount: BigUint<StaticApi>,
}

async fn fetch_manager_contract_config(
    interactor: &mut Interactor,
) -> Result<ManagerContractConfig> {
    let (
        num_contracts,
        last_contract,
        min_service_fee,
        max_service_fee,
        min_deposit,
        min_delegation_amount,
    ) = interactor
        .query()
        .to(DelegationManagerSCAddress)
        .typed(DelegationManagerSCProxy)
        .get_contract_config()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("delegation manager config query failed: {}", err.message))?
        .into_tuple();

    Ok(ManagerContractConfig {
        num_contracts,
        last_contract,
        min_service_fee,
        max_service_fee,
        min_deposit,
        min_delegation_amount,
    })
}

async fn query_manager_contract_config(args: &SystemQueryArgs) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let config = fetch_manager_contract_config(&mut interactor).await?;

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "numContracts": config.num_contracts,
            "lastContract": bech32(&config.last_contract.to_address(), &interactor),
            "minServiceFee": config.min_service_fee,
            "maxServiceFee": config.max_service_fee,
            "minDeposit": config.min_deposit.to_display().to_string(),
            "minDelegationAmount": config.min_delegation_amount.to_display().to_string(),
        }))?
    );
    Ok(())
}

async fn query_contract_addresses(args: &SystemQueryArgs) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let addresses = interactor
        .query()
        .to(DelegationManagerSCAddress)
        .typed(DelegationManagerSCProxy)
        .get_all_contract_addresses()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| {
            anyhow!(
                "delegation contract addresses query failed: {}",
                err.message
            )
        })?;

    let addresses: Vec<String> = addresses
        .into_iter()
        .map(|address| bech32(&address.to_address(), &interactor))
        .collect();
    println!("{}", Interactor::to_json_pretty(&addresses)?);
    Ok(())
}

// ---------------------------------------------------------------------------
// Delegation
// ---------------------------------------------------------------------------

pub(super) async fn tx_system_delegation(args: &DelegationCliArgs) -> Result<()> {
    let contract = parse_address("contract", &args.contract)?;

    match &args.command {
        // --- owner ---
        DelegationCliAction::SetMetadata(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation
                    .set_metadata(a.name.as_str(), a.website.as_str(), a.identifier.as_str())
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::ChangeServiceFee(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation
                    .change_service_fee(a.service_fee)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::ModifyTotalDelegationCap(a) => {
            let cap = biguint(&a.total_delegation_cap);
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation
                    .modify_total_delegation_cap(cap)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::SetAutomaticActivation(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation
                    .set_automatic_activation(a.enabled)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::SetCheckCapOnRedelegateRewards(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation
                    .set_check_cap_on_redelegate_rewards(a.enabled)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::AddNodes(a) => add_nodes(a, contract).await,
        DelegationCliAction::RemoveNodes(a) => {
            nodes_tx(a, &contract, 0, |delegation, keys| {
                delegation.remove_nodes(keys).into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::StakeNodes(a) => {
            nodes_tx(a, &contract, NODE_STAKING_EXTRA_GAS, |delegation, keys| {
                delegation.stake_nodes(keys).into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::UnstakeNodes(a) => {
            nodes_tx(a, &contract, NODE_STAKING_EXTRA_GAS, |delegation, keys| {
                delegation.unstake_nodes(keys).into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::UnbondNodes(a) => {
            nodes_tx(a, &contract, NODE_STAKING_EXTRA_GAS, |delegation, keys| {
                delegation.unbond_nodes(keys).into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::RestakeUnstakedNodes(a) => {
            nodes_tx(a, &contract, NODE_STAKING_EXTRA_GAS, |delegation, keys| {
                delegation
                    .restake_unstaked_nodes(keys)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::UnjailNodes(a) => {
            let keys = resolve_bls_keys(&a.keys)?;
            let gas = NODE_OPS_BASE_GAS + NODE_OPS_GAS_PER_NODE * keys.len() as u64;
            let unjail_price_per_node = biguint(&a.unjail_price_per_node);
            delegation_tx(&a.common, &contract, gas, |delegation| {
                delegation
                    .unjail_nodes(keys, unjail_price_per_node)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::WhitelistForMerge(a) => {
            let address = parse_address("--address", &a.address)?;
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation
                    .whitelist_for_merge(&address)
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::DeleteWhitelistForMerge(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation
                    .delete_whitelist_for_merge()
                    .into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::ChangeOwner(a) => {
            let new_owner = parse_address("--address", &a.address)?;
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation.change_owner(&new_owner).into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::SynchronizeOwner(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation.synchronize_owner().into_sdk_transaction()
            })
            .await
        }

        // --- delegator ---
        DelegationCliAction::Delegate(a) => {
            let value = biguint(&a.value);
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation.delegate(value).into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::ClaimRewards(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation.claim_rewards().into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::RedelegateRewards(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation.redelegate_rewards().into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::Undelegate(a) => {
            let amount = biguint(&a.amount);
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation.undelegate(amount).into_sdk_transaction()
            })
            .await
        }
        DelegationCliAction::Withdraw(a) => {
            delegation_tx(&a.common, &contract, DELEGATION_OPS_GAS, |delegation| {
                delegation.withdraw().into_sdk_transaction()
            })
            .await
        }

        // --- queries ---
        DelegationCliAction::ContractConfig(a) => query_contract_config(a, &contract).await,
        DelegationCliAction::Metadata(a) => query_metadata(a, &contract).await,
        DelegationCliAction::NodeStates(a) => query_node_states(a, &contract).await,
        DelegationCliAction::Totals(a) => query_totals(a, &contract).await,
        DelegationCliAction::Delegator(a) => query_delegator(a, &contract).await,
        DelegationCliAction::RewardData(a) => query_reward_data(a, &contract).await,
        DelegationCliAction::WhitelistedForMerge(a) => {
            query_whitelisted_for_merge(a, &contract).await
        }
    }
}

/// Starts a transaction from `sender` to a delegation contract, through the typed proxy.
fn delegation_proxy<'a>(
    interactor: &'a mut Interactor,
    sender: &Bech32Address,
    contract: &Bech32Address,
    relayer: Option<Bech32Address>,
    gas_limit: u64,
) -> DelegationProxy<'a> {
    system_proxy(
        interactor,
        sender,
        contract.clone(),
        relayer,
        gas_limit,
        DelegationSCProxy,
    )
}

/// Builds a transaction with the delegation contract proxy, then signs and dispatches it.
async fn delegation_tx<F>(
    common: &SystemTxCommonArgs,
    contract: &Bech32Address,
    default_gas_limit: u64,
    build: F,
) -> Result<()>
where
    F: FnOnce(DelegationProxy<'_>) -> Transaction,
{
    system_tx(
        common,
        default_gas_limit,
        async move |interactor, sender, relayer, gas_limit| {
            Ok(build(delegation_proxy(
                interactor, &sender, contract, relayer, gas_limit,
            )))
        },
    )
    .await
}

/// Node operation: the default gas limit depends on the number of nodes.
async fn nodes_tx<F>(
    args: &DelegationNodesArgs,
    contract: &Bech32Address,
    extra_gas: u64,
    build: F,
) -> Result<()>
where
    F: FnOnce(
        DelegationProxy<'_>,
        multiversx_sc_snippets::imports::MultiValueVec<multiversx_sc_snippets::imports::BLSKey>,
    ) -> Transaction,
{
    let keys = resolve_bls_keys(&args.keys)?;
    let gas = NODE_OPS_BASE_GAS + extra_gas + NODE_OPS_GAS_PER_NODE * keys.len() as u64;
    delegation_tx(&args.common, contract, gas, |delegation| {
        build(delegation, keys)
    })
    .await
}

/// Adds nodes: each BLS key signs the delegation contract address.
async fn add_nodes(args: &DelegationAddNodesArgs, contract: Bech32Address) -> Result<()> {
    let keys = BlsKeysSignatures::from_args(&args.keys)?;
    let gas = NODE_OPS_BASE_GAS + NODE_OPS_GAS_PER_NODE * keys.len() as u64;
    let pairs = keys.into_pairs(contract.as_address())?;
    delegation_tx(&args.common, &contract, gas, |delegation| {
        delegation.add_nodes(pairs).into_sdk_transaction()
    })
    .await
}

// ---------------------------------------------------------------------------
// Delegation queries
// ---------------------------------------------------------------------------

async fn query_contract_config(args: &SystemQueryArgs, contract: &Bech32Address) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let config = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_contract_config()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("delegation contract config query failed: {}", err.message))?;

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "owner": bech32(&config.owner.to_address(), &interactor),
            "serviceFee": config.service_fee,
            "maxDelegationCap": config.max_delegation_cap.to_display().to_string(),
            "initialOwnerFunds": config.initial_owner_funds.to_display().to_string(),
            "automaticActivation": config.automatic_activation,
            "withDelegationCap": config.with_delegation_cap,
            "changeableServiceFee": config.changeable_service_fee,
            "checkCapOnRedelegate": config.check_cap_on_redelegate,
            "createdNonce": config.created_nonce,
            "unbondPeriodInEpochs": config.unbond_period_in_epochs,
        }))?
    );
    Ok(())
}

async fn query_metadata(args: &SystemQueryArgs, contract: &Bech32Address) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let (name, website, identifier) = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_meta_data()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("metadata query failed: {}", err.message))?
        .into_tuple();

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "name": name.to_string(),
            "website": website.to_string(),
            "identifier": identifier.to_string(),
        }))?
    );
    Ok(())
}

async fn query_node_states(args: &SystemQueryArgs, contract: &Bech32Address) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let node_states = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_all_node_states()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("node states query failed: {}", err.message))?;

    let hex_keys = |keys: &[multiversx_sc_snippets::imports::BLSKey]| -> Vec<String> {
        keys.iter().map(|key| hex::encode(key.to_vec())).collect()
    };
    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "staked": hex_keys(&node_states.staked),
            "notStaked": hex_keys(&node_states.not_staked),
            "unStaked": hex_keys(&node_states.un_staked),
        }))?
    );
    Ok(())
}

async fn query_totals(args: &SystemQueryArgs, contract: &Bech32Address) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;

    let total_active_stake = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_total_active_stake()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("total active stake query failed: {}", err.message))?;
    let total_unstaked = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_total_unstaked()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("total unstaked query failed: {}", err.message))?;
    let num_users = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_num_users()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("number of users query failed: {}", err.message))?;
    let num_nodes = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_num_nodes()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("number of nodes query failed: {}", err.message))?;

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "totalActiveStake": total_active_stake.to_display().to_string(),
            "totalUnstaked": total_unstaked.to_display().to_string(),
            "numUsers": num_users,
            "numNodes": num_nodes,
        }))?
    );
    Ok(())
}

async fn query_delegator(
    args: &DelegationDelegatorQueryArgs,
    contract: &Bech32Address,
) -> Result<()> {
    let delegator = parse_address("address", &args.address)?;
    let mut interactor = create_query_interactor(&args.gateway).await?;

    let (active_stake, claimable_rewards, unstaked_value, unbondable_value) = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_delegator_funds_data(&delegator)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("delegator funds query failed: {}", err.message))?
        .into_tuple();
    let total_cumulated_rewards = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_total_cumulated_rewards_for_user(&delegator)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("cumulated rewards query failed: {}", err.message))?;
    let undelegated = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_user_undelegated_list(&delegator)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("undelegated list query failed: {}", err.message))?;

    let undelegated: Vec<serde_json::Value> = undelegated
        .into_iter()
        .map(|entry| {
            let (amount, remaining_epochs) = entry.into_tuple();
            json!({
                "amount": amount.to_display().to_string(),
                "remainingEpochs": remaining_epochs,
            })
        })
        .collect();

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "activeStake": active_stake.to_display().to_string(),
            "claimableRewards": claimable_rewards.to_display().to_string(),
            "totalCumulatedRewards": total_cumulated_rewards.to_display().to_string(),
            "unstakedValue": unstaked_value.to_display().to_string(),
            "unbondableValue": unbondable_value.to_display().to_string(),
            "undelegated": undelegated,
        }))?
    );
    Ok(())
}

async fn query_reward_data(
    args: &DelegationRewardDataQueryArgs,
    contract: &Bech32Address,
) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let (rewards_to_distribute, total_active, service_fee) = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_reward_data(args.epoch)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("reward data query failed: {}", err.message))?
        .into_tuple();

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "rewardsToDistribute": rewards_to_distribute.to_display().to_string(),
            "totalActive": total_active.to_display().to_string(),
            "serviceFee": service_fee,
        }))?
    );
    Ok(())
}

async fn query_whitelisted_for_merge(
    args: &SystemQueryArgs,
    contract: &Bech32Address,
) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let address = interactor
        .query()
        .to(contract)
        .typed(DelegationSCProxy)
        .get_whitelist_for_merge()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("whitelist for merge query failed: {}", err.message))?;

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "address": bech32(&address.to_address(), &interactor),
        }))?
    );
    Ok(())
}
