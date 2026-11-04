use anyhow::{Result, anyhow};
use multiversx_sc_snippets::{
    InteractorEnvExec,
    imports::{
        Bech32Address, BigUint, ESDTSystemSCAddress, ESDTSystemSCProxy, ESDTSystemSCProxyMethods,
        EsdtLocalRole, EsdtTokenIdentifier, EsdtTokenType, ExplicitGas, FungibleTokenProperties,
        GatewayHttpProxy, Interactor, InteractorIntoSdkTransaction, InteractorRunAsync,
        ManagedBuffer, MetaTokenProperties, NonFungibleTokenProperties, ReturnsHandledOrError,
        ReturnsResult, RustBigUint, SemiFungibleTokenProperties, StaticApi, TokenPropertyArguments,
    },
    sdk::data::transaction::Transaction,
};
use serde_json::json;

use super::system_tx_session::{
    SystemTxSession, create_query_interactor, parse_address, print_json,
};
use crate::cli::{
    cli_args_tx_system::SystemTxCommonArgs,
    cli_args_tx_system_esdt::{
        EsdtCliAction, EsdtCliArgs, EsdtControlChangesArgs, EsdtDynamicTokenTypeArg,
        EsdtIssueCostArgs, EsdtIssuePropertiesArgs, EsdtNftAddressArgs, EsdtQueryArgs,
        EsdtRolesArgs, EsdtTokenAddressArgs, EsdtTokenQueryArgs, EsdtTokenTxArgs, EsdtTokenTypeArg,
    },
};

/// Default gas limit for ESDT system SC operations (same as mxpy).
const ESDT_SYSTEM_SC_GAS: u64 = 60_000_000;

/// The ESDT system SC proxy, on top of an interactor transaction.
type EsdtProxy<'a> = ESDTSystemSCProxyMethods<
    InteractorEnvExec<'a, GatewayHttpProxy>,
    Bech32Address,
    ESDTSystemSCAddress,
    ExplicitGas<u64>,
>;

pub(super) async fn tx_system_esdt(args: &EsdtCliArgs) -> Result<()> {
    match &args.command {
        // --- issue ---
        EsdtCliAction::IssueFungible(a) => {
            let properties = FungibleTokenProperties {
                num_decimals: a.num_decimals,
                can_mint: false,
                can_burn: false,
                ..fungible_properties(&a.properties)
            };
            esdt_issue_tx(&a.common, &a.cost, |esdt, cost| {
                esdt.issue_fungible(
                    cost,
                    a.token.name.as_str(),
                    a.token.ticker.as_str(),
                    biguint(&a.initial_supply),
                    properties,
                )
                .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::IssueNonFungible(a) => {
            let p = &a.properties;
            let properties = NonFungibleTokenProperties {
                can_freeze: p.can_freeze,
                can_wipe: p.can_wipe,
                can_pause: p.can_pause,
                can_transfer_create_role: a.can_transfer_nft_create_role,
                can_change_owner: p.can_change_owner,
                can_upgrade: p.can_upgrade,
                can_add_special_roles: p.can_add_special_roles,
            };
            esdt_issue_tx(&a.common, &a.cost, |esdt, cost| {
                esdt.issue_non_fungible(
                    cost,
                    a.token.name.as_str(),
                    a.token.ticker.as_str(),
                    properties,
                )
                .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::IssueSemiFungible(a) => {
            let p = &a.properties;
            let properties = SemiFungibleTokenProperties {
                can_freeze: p.can_freeze,
                can_wipe: p.can_wipe,
                can_pause: p.can_pause,
                can_transfer_create_role: a.can_transfer_nft_create_role,
                can_change_owner: p.can_change_owner,
                can_upgrade: p.can_upgrade,
                can_add_special_roles: p.can_add_special_roles,
            };
            esdt_issue_tx(&a.common, &a.cost, |esdt, cost| {
                esdt.issue_semi_fungible(
                    cost,
                    a.token.name.as_str(),
                    a.token.ticker.as_str(),
                    properties,
                )
                .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::RegisterMetaEsdt(a) => {
            let p = &a.properties;
            let properties = MetaTokenProperties {
                num_decimals: a.num_decimals,
                can_freeze: p.can_freeze,
                can_wipe: p.can_wipe,
                can_pause: p.can_pause,
                can_transfer_create_role: a.can_transfer_nft_create_role,
                can_change_owner: p.can_change_owner,
                can_upgrade: p.can_upgrade,
                can_add_special_roles: p.can_add_special_roles,
            };
            esdt_issue_tx(&a.common, &a.cost, |esdt, cost| {
                esdt.register_meta_esdt(
                    cost,
                    a.token.name.as_str(),
                    a.token.ticker.as_str(),
                    properties,
                )
                .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::RegisterAndSetAllRoles(a) => {
            let token_type = match a.token_type {
                EsdtTokenTypeArg::Fungible => EsdtTokenType::Fungible,
                EsdtTokenTypeArg::NonFungible => EsdtTokenType::NonFungible,
                EsdtTokenTypeArg::SemiFungible => EsdtTokenType::SemiFungible,
                EsdtTokenTypeArg::Meta => EsdtTokenType::MetaFungible,
            };
            esdt_issue_tx(&a.common, &a.cost, |esdt, cost| {
                esdt.issue_and_set_all_roles(
                    cost,
                    a.token.name.as_str(),
                    a.token.ticker.as_str(),
                    token_type,
                    a.num_decimals,
                )
                .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::RegisterDynamic(a) => {
            let token_type = dynamic_token_type(a.token_type);
            esdt_issue_tx(&a.common, &a.cost, |esdt, cost| {
                esdt.issue_dynamic(
                    cost,
                    a.token.name.as_str(),
                    a.token.ticker.as_str(),
                    token_type,
                    a.num_decimals,
                )
                .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::RegisterAndSetAllRolesDynamic(a) => {
            // `issue_and_set_all_roles` routes dynamic types to `registerAndSetAllRolesDynamic`
            let token_type = dynamic_token_type(a.token_type);
            esdt_issue_tx(&a.common, &a.cost, |esdt, cost| {
                esdt.issue_and_set_all_roles(
                    cost,
                    a.token.name.as_str(),
                    a.token.ticker.as_str(),
                    token_type,
                    a.num_decimals,
                )
                .into_sdk_transaction()
            })
            .await
        }

        // --- roles ---
        EsdtCliAction::SetSpecialRole(a) => {
            let (address, roles) = parse_roles_args(a)?;
            esdt_tx(&a.common, |esdt| {
                esdt.set_special_roles(&address, token_id(&a.token), roles.into_iter())
                    .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::UnsetSpecialRole(a) => {
            let (address, roles) = parse_roles_args(a)?;
            esdt_tx(&a.common, |esdt| {
                esdt.unset_special_roles(&address, token_id(&a.token), roles.into_iter())
                    .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::TransferNftCreateRole(a) => {
            let from_address = parse_address("--from-address", &a.from_address)?;
            let to_address = parse_address("--to-address", &a.to_address)?;
            esdt_tx(&a.common, |esdt| {
                esdt.transfer_nft_create_role(token_id(&a.token), &from_address, &to_address)
                    .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::SetBurnRoleGlobally(a) => {
            token_tx(a, |esdt, token| {
                esdt.set_burn_role_globally(token).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::UnsetBurnRoleGlobally(a) => {
            token_tx(a, |esdt, token| {
                esdt.unset_burn_role_globally(token).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::SendAllTransferRoleAddresses(a) => {
            token_tx(a, |esdt, token| {
                esdt.send_all_transfer_role_addresses(token)
                    .into_sdk_transaction()
            })
            .await
        }

        // --- management ---
        EsdtCliAction::Pause(a) => {
            token_tx(a, |esdt, token| esdt.pause(token).into_sdk_transaction()).await
        }
        EsdtCliAction::Unpause(a) => {
            token_tx(a, |esdt, token| esdt.unpause(token).into_sdk_transaction()).await
        }
        EsdtCliAction::Freeze(a) => {
            token_address_tx(a, |esdt, token, address| {
                esdt.freeze(token, address).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::Unfreeze(a) => {
            token_address_tx(a, |esdt, token, address| {
                esdt.unfreeze(token, address).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::Wipe(a) => {
            token_address_tx(a, |esdt, token, address| {
                esdt.wipe(token, address).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::FreezeNft(a) => {
            nft_address_tx(a, |esdt, token, nonce, address| {
                esdt.freeze_nft(token, nonce, address)
                    .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::UnfreezeNft(a) => {
            nft_address_tx(a, |esdt, token, nonce, address| {
                esdt.unfreeze_nft(token, nonce, address)
                    .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::WipeNft(a) => {
            nft_address_tx(a, |esdt, token, nonce, address| {
                esdt.wipe_nft(token, nonce, address).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::TransferOwnership(a) => {
            let new_owner = parse_address("--new-owner", &a.new_owner)?;
            esdt_tx(&a.common, |esdt| {
                esdt.transfer_ownership(token_id(&a.token), &new_owner)
                    .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::ControlChanges(a) => {
            let property_arguments = control_changes_properties(a)?;
            esdt_tx(&a.common, |esdt| {
                esdt.control_changes(token_id(&a.token), &property_arguments)
                    .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::StopNftCreate(a) => {
            token_tx(a, |esdt, token| {
                esdt.stop_nft_create(token).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::ChangeSftToMetaEsdt(a) => {
            esdt_tx(&a.common, |esdt| {
                esdt.change_sft_to_meta_esdt(token_id(&a.token), a.num_decimals)
                    .into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::ChangeToDynamic(a) => {
            token_tx(a, |esdt, token| {
                esdt.change_to_dynamic(token).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::UpdateTokenId(a) => {
            token_tx(a, |esdt, token| {
                esdt.update_token(token).into_sdk_transaction()
            })
            .await
        }
        EsdtCliAction::ChangeToMultiShardCreate(a) => {
            token_tx(a, |esdt, token| {
                esdt.change_to_multi_shard_create(token)
                    .into_sdk_transaction()
            })
            .await
        }

        // --- queries ---
        EsdtCliAction::TokenProperties(a) => query_token_properties(a).await,
        EsdtCliAction::SpecialRoles(a) => query_special_roles(a).await,
        EsdtCliAction::ContractConfig(a) => query_contract_config(a).await,
    }
}

// ---------------------------------------------------------------------------
// Transactions
// ---------------------------------------------------------------------------

impl SystemTxSession {
    /// Starts a transaction from the sender to the ESDT system SC, through the typed proxy.
    fn esdt(&mut self) -> EsdtProxy<'_> {
        let sender = self.sender.clone();
        let relayer = self.relayer.clone();
        let gas_limit = self.gas_limit;
        self.interactor
            .tx()
            .from(sender)
            .to(ESDTSystemSCAddress)
            .gas(gas_limit)
            .opt_relayer(relayer)
            .typed(ESDTSystemSCProxy)
    }
}

/// Builds a transaction with the ESDT system SC proxy, then signs and dispatches it.
async fn esdt_tx<F>(common: &SystemTxCommonArgs, build: F) -> Result<()>
where
    F: FnOnce(EsdtProxy<'_>) -> Transaction,
{
    let mut session = SystemTxSession::new(common, ESDT_SYSTEM_SC_GAS).await?;
    let tx = build(session.esdt());
    session.dispatch(tx).await
}

/// Same as [`esdt_tx`], but also resolves the issue cost,
/// either from the arguments or from the ESDT system SC configuration.
async fn esdt_issue_tx<F>(
    common: &SystemTxCommonArgs,
    cost_args: &EsdtIssueCostArgs,
    build: F,
) -> Result<()>
where
    F: FnOnce(EsdtProxy<'_>, BigUint<StaticApi>) -> Transaction,
{
    let mut session = SystemTxSession::new(common, ESDT_SYSTEM_SC_GAS).await?;
    let issue_cost = match &cost_args.issue_cost {
        Some(issue_cost) => biguint(issue_cost),
        None => {
            let (_, issue_cost, _, _) = fetch_contract_config(&mut session.interactor).await?;
            issue_cost
        }
    };
    let tx = build(session.esdt(), issue_cost);
    session.dispatch(tx).await
}

/// Transaction with only a token identifier argument.
async fn token_tx<F>(args: &EsdtTokenTxArgs, build: F) -> Result<()>
where
    F: FnOnce(EsdtProxy<'_>, EsdtTokenIdentifier<StaticApi>) -> Transaction,
{
    esdt_tx(&args.common, |esdt| build(esdt, token_id(&args.token))).await
}

/// Transaction with a token identifier and an address.
async fn token_address_tx<F>(args: &EsdtTokenAddressArgs, build: F) -> Result<()>
where
    F: FnOnce(EsdtProxy<'_>, EsdtTokenIdentifier<StaticApi>, &Bech32Address) -> Transaction,
{
    let address = parse_address("--address", &args.address)?;
    esdt_tx(&args.common, |esdt| {
        build(esdt, token_id(&args.token), &address)
    })
    .await
}

/// Transaction with a token identifier, a token nonce and an address.
async fn nft_address_tx<F>(args: &EsdtNftAddressArgs, build: F) -> Result<()>
where
    F: FnOnce(EsdtProxy<'_>, EsdtTokenIdentifier<StaticApi>, u64, &Bech32Address) -> Transaction,
{
    let address = parse_address("--address", &args.address)?;
    esdt_tx(&args.common, |esdt| {
        build(esdt, token_id(&args.token), args.token_nonce, &address)
    })
    .await
}

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

async fn fetch_contract_config(
    interactor: &mut Interactor,
) -> Result<(
    multiversx_sc_snippets::imports::ManagedAddress<StaticApi>,
    BigUint<StaticApi>,
    u32,
    u32,
)> {
    let config = interactor
        .query()
        .to(ESDTSystemSCAddress)
        .typed(ESDTSystemSCProxy)
        .get_contract_config()
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("ESDT system SC config query failed: {}", err.message))?;
    Ok(config.into_tuple())
}

async fn query_contract_config(args: &EsdtQueryArgs) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let hrp = interactor.get_hrp();
    let (owner, issue_cost, min_name_length, max_name_length) =
        fetch_contract_config(&mut interactor).await?;

    print_json(&json!({
        "owner": owner.to_address().to_bech32(hrp).to_bech32_string(),
        "issueCost": issue_cost.to_display().to_string(),
        "minTokenNameLength": min_name_length,
        "maxTokenNameLength": max_name_length,
    }))
}

async fn query_token_properties(args: &EsdtTokenQueryArgs) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let hrp = interactor.get_hrp();
    let p = interactor
        .query()
        .to(ESDTSystemSCAddress)
        .typed(ESDTSystemSCProxy)
        .get_token_properties(token_id(&args.token))
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("token properties query failed: {}", err.message))?;

    print_json(&json!({
        "name": p.token_name,
        "type": format!("{:?}", p.token_type),
        "owner": p.owner_address.to_bech32(hrp).to_bech32_string(),
        "mintedValue": p.minted_value,
        "burntValue": p.burnt_value,
        "numDecimals": p.num_decimals,
        "isPaused": p.is_paused,
        "canUpgrade": p.can_upgrade,
        "canMint": p.can_mint,
        "canBurn": p.can_burn,
        "canChangeOwner": p.can_change_owner,
        "canPause": p.can_pause,
        "canFreeze": p.can_freeze,
        "canWipe": p.can_wipe,
        "canAddSpecialRoles": p.can_add_special_roles,
        "canTransferNFTCreateRole": p.can_transfer_nft_create_role,
        "nftCreateStopped": p.nft_create_stopped,
        "numWiped": p.num_wiped,
    }))
}

async fn query_special_roles(args: &EsdtTokenQueryArgs) -> Result<()> {
    let mut interactor = create_query_interactor(&args.gateway).await?;
    let entries = interactor
        .query()
        .to(ESDTSystemSCAddress)
        .typed(ESDTSystemSCProxy)
        .get_special_roles(token_id(&args.token))
        .returns(ReturnsHandledOrError::new().returns(ReturnsResult))
        .run()
        .await
        .map_err(|err| anyhow!("special roles query failed: {}", err.message))?;

    // each entry is formatted as `<bech32 address>:<role1>,<role2>,...`;
    // global roles (e.g. ESDTRoleBurnForAll) have an empty address, shown as null
    let result: Vec<serde_json::Value> = entries
        .into_iter()
        .map(|entry: ManagedBuffer<StaticApi>| {
            let entry = String::from_utf8_lossy(&entry.to_vec()).into_owned();
            let (address, roles) = entry.split_once(':').unwrap_or((entry.as_str(), ""));
            let roles: Vec<&str> = roles.split(',').filter(|r| !r.is_empty()).collect();
            let address = (!address.is_empty()).then_some(address);
            json!({ "address": address, "roles": roles })
        })
        .collect();

    print_json(&result)
}

// ---------------------------------------------------------------------------
// Argument conversion
// ---------------------------------------------------------------------------

fn token_id(token: &str) -> EsdtTokenIdentifier<StaticApi> {
    EsdtTokenIdentifier::from(token)
}

fn biguint(value: &RustBigUint) -> BigUint<StaticApi> {
    BigUint::from_bytes_be(&value.to_bytes_be())
}

fn fungible_properties(p: &EsdtIssuePropertiesArgs) -> FungibleTokenProperties {
    FungibleTokenProperties {
        num_decimals: 0,
        can_freeze: p.can_freeze,
        can_wipe: p.can_wipe,
        can_pause: p.can_pause,
        can_mint: false,
        can_burn: false,
        can_change_owner: p.can_change_owner,
        can_upgrade: p.can_upgrade,
        can_add_special_roles: p.can_add_special_roles,
    }
}

fn dynamic_token_type(token_type: EsdtDynamicTokenTypeArg) -> EsdtTokenType {
    match token_type {
        EsdtDynamicTokenTypeArg::NonFungible => EsdtTokenType::DynamicNFT,
        EsdtDynamicTokenTypeArg::SemiFungible => EsdtTokenType::DynamicSFT,
        EsdtDynamicTokenTypeArg::Meta => EsdtTokenType::DynamicMeta,
    }
}

fn parse_roles_args(args: &EsdtRolesArgs) -> Result<(Bech32Address, Vec<EsdtLocalRole>)> {
    let address = parse_address("--address", &args.address)?;
    let roles = args
        .roles
        .iter()
        .map(|role| parse_role(role.trim()))
        .collect::<Result<Vec<_>>>()?;
    Ok((address, roles))
}

/// Accepts the protocol role names (`ESDTRoleLocalMint`), without the `ESDTRole` prefix (`LocalMint`),
/// or the Rust enum names (`Mint`). Case-insensitive.
fn parse_role(name: &str) -> Result<EsdtLocalRole> {
    EsdtLocalRole::iter_all()
        .copied()
        .find(|role| {
            let protocol_name = role.name();
            protocol_name.eq_ignore_ascii_case(name)
                || protocol_name
                    .strip_prefix("ESDTRole")
                    .is_some_and(|short| short.eq_ignore_ascii_case(name))
                || format!("{role:?}").eq_ignore_ascii_case(name)
        })
        .ok_or_else(|| anyhow!("unknown ESDT role: {name}"))
}

fn control_changes_properties(args: &EsdtControlChangesArgs) -> Result<TokenPropertyArguments> {
    let properties = TokenPropertyArguments {
        can_freeze: args.can_freeze,
        can_wipe: args.can_wipe,
        can_pause: args.can_pause,
        can_transfer_create_role: args.can_transfer_nft_create_role,
        can_mint: args.can_mint,
        can_burn: args.can_burn,
        can_change_owner: args.can_change_owner,
        can_upgrade: args.can_upgrade,
        can_add_special_roles: args.can_add_special_roles,
    };

    let any_set = [
        properties.can_freeze,
        properties.can_wipe,
        properties.can_pause,
        properties.can_transfer_create_role,
        properties.can_mint,
        properties.can_burn,
        properties.can_change_owner,
        properties.can_upgrade,
        properties.can_add_special_roles,
    ]
    .iter()
    .any(Option::is_some);
    if !any_set {
        return Err(anyhow!("control-changes requires at least one property"));
    }

    Ok(properties)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_role_accepts_all_name_forms() {
        assert_eq!(
            parse_role("ESDTRoleLocalMint").unwrap(),
            EsdtLocalRole::Mint
        );
        assert_eq!(parse_role("LocalMint").unwrap(), EsdtLocalRole::Mint);
        assert_eq!(parse_role("mint").unwrap(), EsdtLocalRole::Mint);
        assert_eq!(parse_role("NftCreate").unwrap(), EsdtLocalRole::NftCreate);
        assert_eq!(
            parse_role("ESDTRoleNFTCreate").unwrap(),
            EsdtLocalRole::NftCreate
        );
        assert_eq!(
            parse_role("ESDTTransferRole").unwrap(),
            EsdtLocalRole::Transfer
        );
        assert_eq!(parse_role("transfer").unwrap(), EsdtLocalRole::Transfer);
        assert_eq!(
            parse_role("ESDTRoleNFTUpdate").unwrap(),
            EsdtLocalRole::NftUpdate
        );
        assert_eq!(parse_role("NftUpdate").unwrap(), EsdtLocalRole::NftUpdate);
        assert!(parse_role("NotARole").is_err());
        assert!(parse_role("").is_err());
    }
}
