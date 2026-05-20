use multiversx_sc_scenario::imports::Bech32Address;

/// Builds explorer URLs for a specific MultiversX network.
///
/// Constructed via [`ExplorerUrl::from_chain_id`].
pub struct ExplorerUrl {
    base_url: &'static str,
}

impl ExplorerUrl {
    pub const MAINNET_EXPLORER: &str = "https://explorer.multiversx.com";
    pub const TESTNET_EXPLORER: &str = "https://testnet-explorer.multiversx.com";
    pub const DEVNET_EXPLORER: &str = "https://devnet-explorer.multiversx.com";

    /// Returns an `ExplorerUrl` for the given chain ID, or `None` for unknown chains.
    pub fn from_chain_id(chain_id: &str) -> Option<Self> {
        explorer_base_url_from_chain_id(chain_id).map(|base_url| ExplorerUrl { base_url })
    }

    /// Returns the explorer URL for a transaction hash.
    pub fn tx_url(&self, tx_hash: &str) -> String {
        format!("{}/transactions/{tx_hash}", self.base_url)
    }

    /// Returns the explorer URL for an account address.
    pub fn address_url(&self, address: &Bech32Address) -> String {
        format!("{}/accounts/{}", self.base_url, address.bech32)
    }
}

fn explorer_base_url_from_chain_id(chain_id: &str) -> Option<&'static str> {
    match chain_id {
        "1" => Some(ExplorerUrl::MAINNET_EXPLORER),
        "T" => Some(ExplorerUrl::TESTNET_EXPLORER),
        "D" => Some(ExplorerUrl::DEVNET_EXPLORER),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_chain_id_known() {
        assert!(ExplorerUrl::from_chain_id("1").is_some());
        assert!(ExplorerUrl::from_chain_id("T").is_some());
        assert!(ExplorerUrl::from_chain_id("D").is_some());
    }

    #[test]
    fn test_from_chain_id_unknown() {
        assert!(ExplorerUrl::from_chain_id("").is_none());
        assert!(ExplorerUrl::from_chain_id("X").is_none());
    }

    #[test]
    fn test_tx_url() {
        let ex = ExplorerUrl::from_chain_id("1").unwrap();
        assert_eq!(
            ex.tx_url("abc123"),
            "https://explorer.multiversx.com/transactions/abc123"
        );

        let ex = ExplorerUrl::from_chain_id("T").unwrap();
        assert_eq!(
            ex.tx_url("abc123"),
            "https://testnet-explorer.multiversx.com/transactions/abc123"
        );

        let ex = ExplorerUrl::from_chain_id("D").unwrap();
        assert_eq!(
            ex.tx_url("abc123"),
            "https://devnet-explorer.multiversx.com/transactions/abc123"
        );
    }

    #[test]
    fn test_address_url() {
        let ex = ExplorerUrl::from_chain_id("1").unwrap();
        let addr = Bech32Address::from_bech32_string(
            "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th".to_string(),
        );
        assert_eq!(
            ex.address_url(&addr),
            "https://explorer.multiversx.com/accounts/erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
        );
    }
}
