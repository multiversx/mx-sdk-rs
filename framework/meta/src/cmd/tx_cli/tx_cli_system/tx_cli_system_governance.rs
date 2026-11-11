use anyhow::{Result, anyhow};
use multiversx_sc_snippets::{
    InteractorEnvExec,
    imports::{
        Bech32Address, BigUint, ExplicitGas, GatewayHttpProxy, GovernanceConfigResult,
        GovernanceSCProxy, GovernanceSCProxyMethods, GovernanceSystemSCAddress, Interactor,
        InteractorIntoSdkTransaction, InteractorRunAsync, ManagedAddress, MultiValueEncoded,
        ReturnsHandledOrError, ReturnsResult, RustBigUint, StaticApi,
    },
    sdk::data::transaction::Transaction,
};
use serde_json::json;

use super::system_common::{bech32, biguint, system_proxy, system_tx};
use crate::cli::{
    cli_args_tx_system::{SystemQueryArgs, SystemTxCommonArgs},
    cli_args_tx_system_governance::{
        GovernanceCliAction, GovernanceCliArgs, GovernanceDelegatedVoteInfoArgs,
        GovernanceProposalQueryArgs, GovernanceProposeArgs, GovernanceVotingPowerArgs,
    },
};
use crate::cmd::tx_cli::tx_cli_common::{create_query_interactor, parse_address};

/// Default gas limit for most governance operations (same as mxpy).
const GOVERNANCE_GAS: u64 = 50_000_000;

/// Default gas limit for voting (same as mxpy).
const GOVERNANCE_VOTE_GAS: u64 = 5_100_000;

/// Default gas limit for claiming the accumulated fees (same as mxpy).
const GOVERNANCE_CLAIM_FEES_GAS: u64 = 1_000_000;

/// A git commit hash, as expected in proposals.
const COMMIT_HASH_LENGTH: usize = 40;

/// The governance system SC proxy, on top of an interactor transaction.
type GovernanceProxy<'a> = GovernanceSCProxyMethods<
    InteractorEnvExec<'a, GatewayHttpProxy>,
    Bech32Address,
    GovernanceSystemSCAddress,
    ExplicitGas<u64>,
>;

pub(super) async fn tx_system_governance(args: &GovernanceCliArgs) -> Result<()> {
    match &args.command {
        GovernanceCliAction::Propose(a) => propose(a).await,
        GovernanceCliAction::Vote(a) => {
            governance_tx(&a.common, GOVERNANCE_VOTE_GAS, |governance| {
                governance
                    .vote(a.proposal_nonce, a.vote.as_str())
                    .into_sdk_transaction()
            })
            .await
        }
        GovernanceCliAction::CloseProposal(a) => {
            governance_tx(&a.common, GOVERNANCE_GAS, |governance| {
                governance
                    .close_proposal(a.proposal_nonce)
                    .into_sdk_transaction()
            })
            .await
        }
        GovernanceCliAction::ClearEndedProposals(a) => {
            let proposers = a
                .proposers
                .iter()
                .map(|proposer| parse_address("--proposers", proposer))
                .collect::<Result<Vec<_>>>()?;
            let num_proposers = proposers.len() as u64;
            let proposers: MultiValueEncoded<StaticApi, ManagedAddress<StaticApi>> = proposers
                .iter()
                .map(|proposer| ManagedAddress::from_address(proposer.as_address()))
                .collect();
            governance_tx(
                &a.common,
                GOVERNANCE_GAS * (1 + num_proposers),
                |governance| {
                    governance
                        .clear_ended_proposals(proposers)
                        .into_sdk_transaction()
                },
            )
            .await
        }
        GovernanceCliAction::ClaimAccumulatedFees(a) => {
            governance_tx(&a.common, GOVERNANCE_CLAIM_FEES_GAS, |governance| {
                governance.claim_accumulated_fees().into_sdk_transaction()
            })
            .await
        }
        GovernanceCliAction::ChangeConfig(a) => {
            let proposal_fee = biguint(&a.proposal_fee);
            let lost_proposal_fee = biguint(&a.lost_proposal_fee);
            governance_tx(&a.common, GOVERNANCE_GAS, |governance| {
                governance
                    .change_config(
                        &proposal_fee,
                        &lost_proposal_fee,
                        a.min_quorum,
                        a.min_veto_threshold,
                        a.min_pass_threshold,
                    )
                    .into_sdk_transaction()
            })
            .await
        }

        // --- queries ---
        GovernanceCliAction::Config(a) => query_config(a).await,
        GovernanceCliAction::Proposal(a) => query_proposal(a).await,
        GovernanceCliAction::VotingPower(a) => query_voting_power(a).await,
        GovernanceCliAction::DelegatedVoteInfo(a) => query_delegated_vote_info(a).await,
    }
}

// ---------------------------------------------------------------------------
// Transactions
// ---------------------------------------------------------------------------

/// Starts a transaction from `sender` to the governance system SC, through the typed proxy.
fn governance_proxy<'a>(
    interactor: &'a mut Interactor,
    sender: &Bech32Address,
    relayer: Option<Bech32Address>,
    gas_limit: u64,
) -> GovernanceProxy<'a> {
    system_proxy(
        interactor,
        sender,
        GovernanceSystemSCAddress,
        relayer,
        gas_limit,
        GovernanceSCProxy,
    )
}

/// Builds a transaction with the governance system SC proxy, then signs and dispatches it.
async fn governance_tx<F>(
    common: &SystemTxCommonArgs,
    default_gas_limit: u64,
    build: F,
) -> Result<()>
where
    F: FnOnce(GovernanceProxy<'_>) -> Transaction,
{
    system_tx(
        common,
        default_gas_limit,
        async move |interactor, sender, relayer, gas_limit| {
            Ok(build(governance_proxy(
                interactor, &sender, relayer, gas_limit,
            )))
        },
    )
    .await
}

async fn propose(args: &GovernanceProposeArgs) -> Result<()> {
    if args.commit_hash.len() != COMMIT_HASH_LENGTH {
        return Err(anyhow!(
            "--commit-hash must be exactly {COMMIT_HASH_LENGTH} characters long"
        ));
    }

    system_tx(
        &args.common,
        GOVERNANCE_GAS,
        async move |interactor, sender, relayer, gas_limit| {
            let proposal_fee = resolve_proposal_fee(interactor, &args.proposal_fee).await?;
            Ok(governance_proxy(interactor, &sender, relayer, gas_limit)
                .proposal(
                    args.commit_hash.as_str(),
                    args.start_vote_epoch,
                    args.end_vote_epoch,
                    proposal_fee,
                )
                .into_sdk_transaction())
        },
    )
    .await
}

/// Resolves the proposal fee: either explicitly given, or read from the governance configuration.
async fn resolve_proposal_fee(
    interactor: &mut Interactor,
    proposal_fee: &Option<RustBigUint>,
) -> Result<BigUint<StaticApi>> {
    let proposal_fee = match proposal_fee {
        Some(proposal_fee) => proposal_fee.clone(),
        None => fetch_config(interactor)
            .await?
            .proposal_fee
            .parse()
            .map_err(|_| anyhow!("invalid proposal fee in governance config"))?,
    };
    Ok(biguint(&proposal_fee))
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

async fn fetch_config(interactor: &mut Interactor) -> Result<GovernanceConfigResult> {
    interactor
        .query()
        .to(GovernanceSystemSCAddress)
        .typed(GovernanceSCProxy)
        .view_config()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("governance config query failed: {}", err.message))
}

async fn query_config(args: &SystemQueryArgs) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let config = fetch_config(&mut interactor).await?;

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "proposalFee": config.proposal_fee,
            "lostProposalFee": config.lost_proposal_fee,
            "minQuorum": config.min_quorum,
            "minVetoThreshold": config.min_veto_threshold,
            "minPassThreshold": config.min_pass_threshold,
            "lastProposalNonce": config.last_proposal_nonce,
        }))?
    );
    Ok(())
}

async fn query_proposal(args: &GovernanceProposalQueryArgs) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let proposal = interactor
        .query()
        .to(GovernanceSystemSCAddress)
        .typed(GovernanceSCProxy)
        .view_proposal(args.proposal_nonce)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("proposal query failed: {}", err.message))?;

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "proposalCost": proposal.proposal_cost.to_display().to_string(),
            "commitHash": proposal.commit_hash,
            "nonce": proposal.proposal_nonce,
            "issuer": bech32(&proposal.issuer_address, &interactor),
            "startVoteEpoch": proposal.start_vote_epoch,
            "endVoteEpoch": proposal.end_vote_epoch,
            "quorumStake": proposal.quorum_stake.to_display().to_string(),
            "yes": proposal.yes.to_display().to_string(),
            "no": proposal.no.to_display().to_string(),
            "veto": proposal.veto.to_display().to_string(),
            "abstain": proposal.abstain.to_display().to_string(),
            "closed": proposal.closed,
            "passed": proposal.passed,
        }))?
    );
    Ok(())
}

async fn query_voting_power(args: &GovernanceVotingPowerArgs) -> Result<()> {
    let address = parse_address("address", &args.address)?;
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let voting_power = interactor
        .query()
        .to(GovernanceSystemSCAddress)
        .typed(GovernanceSCProxy)
        .view_voting_power(&address)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("voting power query failed: {}", err.message))?;

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "votingPower": voting_power.to_display().to_string(),
        }))?
    );
    Ok(())
}

async fn query_delegated_vote_info(args: &GovernanceDelegatedVoteInfoArgs) -> Result<()> {
    let contract = parse_address("--contract", &args.contract)?;
    let user = parse_address("--user", &args.user)?;
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let (used_stake, used_power, total_stake, total_power) = interactor
        .query()
        .to(GovernanceSystemSCAddress)
        .typed(GovernanceSCProxy)
        .view_delegated_vote_info(&contract, &user)
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("delegated vote info query failed: {}", err.message))?
        .into_tuple();

    println!(
        "{}",
        Interactor::to_json_pretty(&json!({
            "usedStake": used_stake.to_display().to_string(),
            "usedPower": used_power.to_display().to_string(),
            "totalStake": total_stake.to_display().to_string(),
            "totalPower": total_power.to_display().to_string(),
        }))?
    );
    Ok(())
}
