use crate::data::api::{ApiAccount, ApiUpgrade};
use multiversx_chain_core::std::Bech32Address;

use super::{API_ACCOUNTS_ENDPOINT, GatewayRequest, GatewayRequestType};

/// Retrieves an account from the MultiversX API: code hash, owner, deploy transaction,
/// and, optionally, the contract code.
pub struct ApiGetAccountRequest<'a> {
    pub address: &'a Bech32Address,
    pub with_code: bool,
}

impl<'a> ApiGetAccountRequest<'a> {
    /// Requests the account, including the contract code.
    pub fn new(address: &'a Bech32Address) -> Self {
        Self {
            address,
            with_code: true,
        }
    }

    /// Leaves out the contract code, which can be large, when only the metadata is needed.
    pub fn without_code(self) -> Self {
        Self {
            with_code: false,
            ..self
        }
    }
}

impl GatewayRequest for ApiGetAccountRequest<'_> {
    type Payload = ();
    type DecodedJson = ApiAccount;
    type Result = ApiAccount;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        let code_field = if self.with_code { "code," } else { "" };
        format!(
            "{API_ACCOUNTS_ENDPOINT}/{}?fields=address,{code_field}codeHash,deployTxHash,deployedAt,ownerAddress",
            self.address.bech32
        )
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        Ok(decoded)
    }
}

/// Retrieves the upgrade history of a contract from the MultiversX API, newest first.
pub struct ApiGetUpgradesRequest<'a> {
    pub address: &'a Bech32Address,
}

impl<'a> ApiGetUpgradesRequest<'a> {
    pub fn new(address: &'a Bech32Address) -> Self {
        Self { address }
    }
}

impl GatewayRequest for ApiGetUpgradesRequest<'_> {
    type Payload = ();
    type DecodedJson = Vec<ApiUpgrade>;
    type Result = Vec<ApiUpgrade>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!(
            "{API_ACCOUNTS_ENDPOINT}/{}/upgrades?size=10000",
            self.address.bech32
        )
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        Ok(decoded)
    }
}
