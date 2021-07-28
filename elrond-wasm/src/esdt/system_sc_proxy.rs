use super::properties::*;
use hex_literal::hex;

use crate::{
    api::{BigUintApi, SendApi},
    types::{Address, BoxedBytes, ContractCall, EsdtLocalRole, EsdtTokenType, TokenIdentifier},
};

/// Address of the system smart contract that manages ESDT.
/// Bech32: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");

pub fn esdt_system_sc_address() -> Address {
    Address::from(ESDT_SYSTEM_SC_ADDRESS_ARRAY)
}

const ISSUE_FUNGIBLE_ENDPOINT_NAME: &[u8] = b"issue";
const ISSUE_NON_FUNGIBLE_ENDPOINT_NAME: &[u8] = b"issueNonFungible";
const ISSUE_SEMI_FUNGIBLE_ENDPOINT_NAME: &[u8] = b"issueSemiFungible";

/// Proxy for the ESDT system smart contract.
/// Unlike other contract proxies, this one has a fixed address,
/// so the proxy object doesn't really contain any data, it is more of a placeholder.
pub struct ESDTSystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    pub api: SA,
}

impl<SA> ESDTSystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    /// Constructor.
    /// TODO: consider moving this to a new Proxy contructor trait (bonus: better proxy constructor syntax).
    pub fn new_proxy_obj(api: SA) -> Self {
        ESDTSystemSmartContractProxy { api }
    }
}

impl<SA> ESDTSystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    /// Produces a contract call to the ESDT system SC,
    /// which causes it to issue a new fungible ESDT token.
    pub fn issue_fungible(
        self,
        issue_cost: SA::AmountType,
        token_display_name: &BoxedBytes,
        token_ticker: &BoxedBytes,
        initial_supply: &SA::AmountType,
        properties: FungibleTokenProperties,
    ) -> ContractCall<SA, ()> {
        self.issue(
            issue_cost,
            EsdtTokenType::Fungible,
            token_display_name,
            token_ticker,
            initial_supply,
            properties,
        )
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to issue a new non-fungible ESDT token.
    pub fn issue_non_fungible(
        self,
        issue_cost: SA::AmountType,
        token_display_name: &BoxedBytes,
        token_ticker: &BoxedBytes,
        properties: NonFungibleTokenProperties,
    ) -> ContractCall<SA, ()> {
        self.issue(
            issue_cost,
            EsdtTokenType::NonFungible,
            token_display_name,
            token_ticker,
            &SA::AmountType::zero(),
            TokenProperties {
                num_decimals: 0,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_mint: false,
                can_burn: false,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to issue a new semi-fungible ESDT token.
    pub fn issue_semi_fungible(
        self,
        issue_cost: SA::AmountType,
        token_display_name: &BoxedBytes,
        token_ticker: &BoxedBytes,
        properties: SemiFungibleTokenProperties,
    ) -> ContractCall<SA, ()> {
        self.issue(
            issue_cost,
            EsdtTokenType::SemiFungible,
            token_display_name,
            token_ticker,
            &SA::AmountType::zero(),
            TokenProperties {
                num_decimals: 0,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_mint: false,
                can_burn: false,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    /// Deduplicates code from all the possible issue functions
    fn issue(
        self,
        issue_cost: SA::AmountType,
        token_type: EsdtTokenType,
        token_display_name: &BoxedBytes,
        token_ticker: &BoxedBytes,
        initial_supply: &SA::AmountType,
        properties: TokenProperties,
    ) -> ContractCall<SA, ()> {
        let endpoint_name = match token_type {
            EsdtTokenType::Fungible => ISSUE_FUNGIBLE_ENDPOINT_NAME,
            EsdtTokenType::NonFungible => ISSUE_NON_FUNGIBLE_ENDPOINT_NAME,
            EsdtTokenType::SemiFungible => ISSUE_SEMI_FUNGIBLE_ENDPOINT_NAME,
            EsdtTokenType::Invalid => &[],
        };

        let mut contract_call = ContractCall::new(
            self.api,
            esdt_system_sc_address(),
            BoxedBytes::from(endpoint_name),
        )
        .with_token_transfer(TokenIdentifier::egld(), issue_cost);

        contract_call.push_argument_raw_bytes(token_display_name.as_slice());
        contract_call.push_argument_raw_bytes(token_ticker.as_slice());

        if token_type == EsdtTokenType::Fungible {
            contract_call.push_argument_raw_bytes(&initial_supply.to_bytes_be());
            contract_call.push_argument_raw_bytes(&properties.num_decimals.to_be_bytes());
        }

        set_token_property(&mut contract_call, &b"canFreeze"[..], properties.can_freeze);
        set_token_property(&mut contract_call, &b"canWipe"[..], properties.can_wipe);
        set_token_property(&mut contract_call, &b"canPause"[..], properties.can_pause);

        if token_type == EsdtTokenType::Fungible {
            set_token_property(&mut contract_call, &b"canMint"[..], properties.can_mint);
            set_token_property(&mut contract_call, &b"canBurn"[..], properties.can_burn);
        }

        set_token_property(
            &mut contract_call,
            &b"canChangeOwner"[..],
            properties.can_change_owner,
        );
        set_token_property(
            &mut contract_call,
            &b"canUpgrade"[..],
            properties.can_upgrade,
        );
        set_token_property(
            &mut contract_call,
            &b"canAddSpecialRoles"[..],
            properties.can_add_special_roles,
        );

        contract_call
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to mint more fungible ESDT tokens.
    /// It will fail if the SC is not the owner of the token.
    pub fn mint(
        self,
        token_identifier: &TokenIdentifier,
        amount: &SA::AmountType,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"mint");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(&amount.to_bytes_be());

        contract_call
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to burn fungible ESDT tokens owned by the SC.
    pub fn burn(
        self,
        token_identifier: &TokenIdentifier,
        amount: &SA::AmountType,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"ESDTBurn");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(&amount.to_bytes_be());

        contract_call
    }

    /// The manager of an ESDT token may choose to suspend all transactions of the token,
    /// except minting, freezing/unfreezing and wiping.
    pub fn pause(self, token_identifier: &TokenIdentifier) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"pause");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());

        contract_call
    }

    /// The reverse operation of `pause`.
    pub fn unpause(self, token_identifier: &TokenIdentifier) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"unPause");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());

        contract_call
    }

    /// The manager of an ESDT token may freeze the tokens held by a specific account.
    /// As a consequence, no tokens may be transferred to or from the frozen account.
    /// Freezing and unfreezing the tokens of an account are operations designed to help token managers to comply with regulations.
    pub fn freeze(
        self,
        token_identifier: &TokenIdentifier,
        address: &Address,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"freeze");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(address.as_bytes());

        contract_call
    }

    /// The reverse operation of `freeze`, unfreezing, will allow further transfers to and from the account.
    pub fn unfreeze(
        self,
        token_identifier: &TokenIdentifier,
        address: &Address,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"unFreeze");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(address.as_bytes());

        contract_call
    }

    /// The manager of an ESDT token may wipe out all the tokens held by a frozen account.
    /// This operation is similar to burning the tokens, but the account must have been frozen beforehand,
    /// and it must be done by the token manager.
    /// Wiping the tokens of an account is an operation designed to help token managers to comply with regulations.
    pub fn wipe(
        self,
        token_identifier: &TokenIdentifier,
        address: &Address,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"wipe");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(address.as_bytes());

        contract_call
    }

    /// This function can be called only if canSetSpecialRoles was set to true.
    /// The metachain system SC will evaluate the arguments and call “ESDTSetRole@tokenId@listOfRoles” for the given address.
    /// This will be actually a cross shard call.
    /// This function as almost all in case of ESDT can be called only by the owner.
    pub fn set_special_roles(
        self,
        address: &Address,
        token_identifier: &TokenIdentifier,
        roles: &[EsdtLocalRole],
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"setSpecialRole");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(address.as_bytes());
        for role in roles {
            if role != &EsdtLocalRole::None {
                contract_call.push_argument_raw_bytes(role.as_role_name());
            }
        }

        contract_call
    }

    /// This function can be called only if canSetSpecialRoles was set to true.
    /// The metachain system SC will evaluate the arguments and call “ESDTUnsetRole@tokenId@listOfRoles” for the given address.
    /// This will be actually a cross shard call.
    /// This function as almost all in case of ESDT can be called only by the owner.
    pub fn unset_special_roles(
        self,
        address: &Address,
        token_identifier: &TokenIdentifier,
        roles: &[EsdtLocalRole],
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"unSetSpecialRole");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(address.as_bytes());
        for role in roles {
            if role != &EsdtLocalRole::None {
                contract_call.push_argument_raw_bytes(role.as_role_name());
            }
        }

        contract_call
    }

    pub fn transfer_ownership(
        self,
        token_identifier: &TokenIdentifier,
        new_owner: &Address,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"transferOwnership");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(new_owner.as_bytes());

        contract_call
    }

    pub fn transfer_nft_create_role(
        self,
        token_identifier: &TokenIdentifier,
        old_creator: &Address,
        new_creator: &Address,
    ) -> ContractCall<SA, ()> {
        let mut contract_call = self.esdt_system_sc_call_no_args(b"transferNFTCreateRole");

        contract_call.push_argument_raw_bytes(token_identifier.as_esdt_identifier());
        contract_call.push_argument_raw_bytes(old_creator.as_bytes());
        contract_call.push_argument_raw_bytes(new_creator.as_bytes());

        contract_call
    }

    fn esdt_system_sc_call_no_args(self, endpoint_name: &[u8]) -> ContractCall<SA, ()> {
        ContractCall::new(self.api, esdt_system_sc_address(), endpoint_name.into())
    }
}

const TRUE_BYTES: &[u8] = b"true";
const FALSE_BYTES: &[u8] = b"false";

fn bool_name_bytes(b: bool) -> &'static [u8] {
    if b {
        TRUE_BYTES
    } else {
        FALSE_BYTES
    }
}

fn set_token_property<SA, R>(contract_call: &mut ContractCall<SA, R>, name: &[u8], value: bool)
where
    SA: SendApi + 'static,
{
    contract_call.push_argument_raw_bytes(name);
    contract_call.push_argument_raw_bytes(bool_name_bytes(value));
}
