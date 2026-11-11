use std::path::Path;

use anyhow::{Result, anyhow};
use multiversx_chain_core::std::base64_decode;
use multiversx_sc_snippets::{
    InteractorEnvExec,
    imports::{
        Address, BLSKey, BLSSignature, Bech32Address, BigUint, ExplicitGas, GatewayHttpProxy,
        Interactor, MultiValue2, MultiValueVec, RustBigUint, StaticApi, TxProxyTrait, TxTo,
    },
    sdk::{data::transaction::Transaction, validator::Validator},
};

use crate::cli::cli_args_tx_system::{BlsKeysArgs, BlsKeysSignaturesArgs, SystemTxCommonArgs};
use crate::cmd::tx_cli::tx_cli_common::{
    create_interactor, load_relayer_for_interactor, load_wallet,
};

/// Starts a transaction from `sender` to a system smart contract, through its typed proxy.
pub(super) fn system_proxy<'a, To, P>(
    interactor: &'a mut Interactor,
    sender: &Bech32Address,
    to: To,
    relayer: Option<Bech32Address>,
    gas_limit: u64,
    proxy: P,
) -> P::TxProxyMethods
where
    To: TxTo<InteractorEnvExec<'a, GatewayHttpProxy>>,
    P: TxProxyTrait<InteractorEnvExec<'a, GatewayHttpProxy>, Bech32Address, To, ExplicitGas<u64>>,
{
    interactor
        .tx()
        .from(sender.clone())
        .to(to)
        .gas(gas_limit)
        .opt_relayer(relayer)
        .typed(proxy)
}

/// Runs a system SC transaction from CLI arguments.
///
/// Connects to the gateway, registers the sender and the relayer, then lets `build` produce the transaction
/// (with the interactor, sender, relayer and gas limit). Finally sets the sender nonce, signs, and dispatches.
///
/// When `--gas-limit` is not given, the gas limit is computed like in mxpy:
/// `default_gas_limit` (the cost of the operation) + the network minimum gas limit + the cost of the data field.
pub(super) async fn system_tx<B>(
    common: &SystemTxCommonArgs,
    default_gas_limit: u64,
    build: B,
) -> Result<()>
where
    B: AsyncFnOnce(
        &mut Interactor,
        Bech32Address,
        Option<Bech32Address>,
        u64,
    ) -> Result<Transaction>,
{
    let tx_args = common.resolve_tx_args(default_gas_limit);
    let sender_wallet = load_wallet(&common.sender)?;

    let mut interactor = create_interactor(&common.gateway, &tx_args).await?;
    let sender_address = interactor.register_wallet_bech32(sender_wallet).await;
    let relayer_address_opt = load_relayer_for_interactor(&mut interactor, &common.relayer).await?;

    let mut tx = build(
        &mut interactor,
        sender_address,
        relayer_address_opt,
        tx_args.gas_limit,
    )
    .await?;
    if common.tx.gas_limit.is_none() {
        tx.gas_limit += data_field_gas(&interactor, &tx)?;
    }
    interactor.set_tx_nonce_update_sender(&mut tx).await;

    interactor
        .sign_and_dispatch(
            tx,
            tx_args.send,
            tx_args.wait_result,
            tx_args.outfile.as_deref(),
        )
        .await
}

/// The base cost of a transaction: the network minimum gas limit, plus the cost of the data field.
fn data_field_gas(interactor: &Interactor, tx: &Transaction) -> Result<u64> {
    let data_len = match &tx.data {
        Some(data) => base64_decode(data)
            .map_err(|e| anyhow!("invalid transaction data: {e}"))?
            .len() as u64,
        None => 0,
    };
    let network_config = interactor.network_config();
    Ok(network_config.min_gas_limit + network_config.gas_per_data_byte * data_len)
}

/// Converts an amount given as a CLI argument.
pub(super) fn biguint(value: &RustBigUint) -> BigUint<StaticApi> {
    BigUint::from_bytes_be(&value.to_bytes_be())
}

/// Formats an address with the network HRP.
pub(super) fn bech32(address: &Address, interactor: &Interactor) -> String {
    address.to_bech32(interactor.get_hrp()).to_bech32_string()
}

fn parse_bls_key(key: &str) -> Result<BLSKey> {
    BLSKey::parse_hex(key.trim()).ok_or_else(|| anyhow!("invalid BLS public key: {key}"))
}

fn parse_bls_signature(signature: &str) -> Result<BLSSignature> {
    BLSSignature::parse_hex(signature.trim())
        .ok_or_else(|| anyhow!("invalid BLS signature: {signature}"))
}

fn load_validators(path: &Path) -> Result<Vec<Validator>> {
    let path_str = path
        .to_str()
        .ok_or_else(|| anyhow!("invalid path {}", path.display()))?;
    Validator::from_pem_file_all(path_str)
        .map_err(|e| anyhow!("failed to load validators PEM {}: {e}", path.display()))
}

/// Resolves the BLS public keys, either from `--bls-keys`, or from `--validators-pem`.
pub(super) fn resolve_bls_keys(args: &BlsKeysArgs) -> Result<MultiValueVec<BLSKey>> {
    let keys = if let Some(path) = &args.validators_pem {
        load_validators(path)?
            .into_iter()
            .map(|validator| validator.public_key)
            .collect()
    } else {
        args.bls_keys
            .iter()
            .map(|key| parse_bls_key(key))
            .collect::<Result<Vec<_>>>()?
    };

    if keys.is_empty() {
        return Err(anyhow!("at least one BLS key is required"));
    }
    Ok(MultiValueVec::from(keys))
}

/// BLS keys, together with their signatures, or with the means to produce them.
pub(super) enum BlsKeysSignatures {
    /// Keys and signatures given explicitly.
    Signed(Vec<(BLSKey, BLSSignature)>),
    /// Keys loaded from a validators PEM file, to be signed.
    Validators(Vec<Validator>),
}

impl BlsKeysSignatures {
    /// Resolves the arguments: either `--bls-keys` with `--signatures`, or `--validators-pem`.
    pub fn from_args(args: &BlsKeysSignaturesArgs) -> Result<Self> {
        let result = if let Some(path) = &args.validators_pem {
            BlsKeysSignatures::Validators(load_validators(path)?)
        } else {
            if args.bls_keys.len() != args.signatures.len() {
                return Err(anyhow!(
                    "--bls-keys and --signatures must have the same number of values"
                ));
            }
            let pairs = args
                .bls_keys
                .iter()
                .zip(args.signatures.iter())
                .map(|(key, signature)| Ok((parse_bls_key(key)?, parse_bls_signature(signature)?)))
                .collect::<Result<Vec<_>>>()?;
            BlsKeysSignatures::Signed(pairs)
        };

        if result.len() == 0 {
            return Err(anyhow!(
                "at least one BLS key is required: use --validators-pem, or --bls-keys with --signatures"
            ));
        }
        Ok(result)
    }

    pub fn len(&self) -> usize {
        match self {
            BlsKeysSignatures::Signed(pairs) => pairs.len(),
            BlsKeysSignatures::Validators(validators) => validators.len(),
        }
    }

    /// Produces the key-signature pairs. When signing, each BLS key signs `signed_address`.
    pub fn into_pairs(
        self,
        signed_address: &Address,
    ) -> Result<MultiValueVec<MultiValue2<BLSKey, BLSSignature>>> {
        let pairs = match self {
            BlsKeysSignatures::Signed(pairs) => pairs,
            BlsKeysSignatures::Validators(validators) => validators
                .iter()
                .map(|validator| {
                    Ok((
                        validator.public_key,
                        sign_with_validator(validator, signed_address)?,
                    ))
                })
                .collect::<Result<Vec<_>>>()?,
        };
        Ok(MultiValueVec::from(
            pairs.into_iter().map(MultiValue2::from).collect::<Vec<_>>(),
        ))
    }
}

#[cfg(feature = "bls")]
fn sign_with_validator(validator: &Validator, signed_address: &Address) -> Result<BLSSignature> {
    validator.sign(signed_address.as_bytes())
}

#[cfg(not(feature = "bls"))]
fn sign_with_validator(_validator: &Validator, _signed_address: &Address) -> Result<BLSSignature> {
    Err(anyhow!(
        "signing with --validators-pem requires sc-meta built with the `bls` feature \
        (cargo install multiversx-sc-meta --features bls); \
        alternatively, provide --bls-keys and --signatures"
    ))
}
