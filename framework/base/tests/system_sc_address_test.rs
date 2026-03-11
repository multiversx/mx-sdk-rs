use multiversx_sc::types::{
    DelegationManagerSCAddress, ESDTSystemSCAddress, GovernanceSystemSCAddress, SystemSCAddress,
    ValidatorSystemSCAddress,
};

#[test]
fn esdt_system_sc_address_is_metachain_sc() {
    assert!(
        ESDTSystemSCAddress
            .to_address()
            .is_smart_contract_on_metachain()
    );
}

#[test]
fn governance_sc_address_is_metachain_sc() {
    assert!(
        GovernanceSystemSCAddress
            .to_address()
            .is_smart_contract_on_metachain()
    );
}

#[test]
fn validator_sc_address_is_metachain_sc() {
    assert!(
        ValidatorSystemSCAddress
            .to_address()
            .is_smart_contract_on_metachain()
    );
}

#[test]
fn delegation_manager_sc_address_is_metachain_sc() {
    assert!(
        DelegationManagerSCAddress
            .to_address()
            .is_smart_contract_on_metachain()
    );
}

/// SystemSCAddress is the all-0xFF protocol address — it is not a regular smart
/// contract address and therefore is not on the metachain either.
#[test]
fn system_sc_address_is_not_metachain_sc() {
    assert!(
        !SystemSCAddress
            .to_address()
            .is_smart_contract_on_metachain()
    );
}

#[cfg(feature = "std")]
mod bech32_tests {
    use super::*;

    #[test]
    fn esdt_system_sc_address_bech32_matches() {
        let address = ESDTSystemSCAddress.to_address();
        assert_eq!(
            address.to_bech32_default().bech32,
            ESDTSystemSCAddress.to_bech32_str(),
        );
    }

    #[test]
    fn governance_sc_address_bech32_matches() {
        let address = GovernanceSystemSCAddress.to_address();
        assert_eq!(
            address.to_bech32_default().bech32,
            GovernanceSystemSCAddress.to_bech32_str(),
        );
    }

    #[test]
    fn validator_sc_address_bech32_matches() {
        let address = ValidatorSystemSCAddress.to_address();
        assert_eq!(
            address.to_bech32_default().bech32,
            ValidatorSystemSCAddress.to_bech32_str(),
        );
    }

    #[test]
    fn delegation_manager_sc_address_bech32_matches() {
        let address = DelegationManagerSCAddress.to_address();
        assert_eq!(
            address.to_bech32_default().bech32,
            DelegationManagerSCAddress.to_bech32_str(),
        );
    }

    #[test]
    fn system_sc_address_bech32_matches() {
        let address = SystemSCAddress.to_address();
        assert_eq!(
            address.to_bech32_default().bech32,
            SystemSCAddress.to_bech32_str(),
        );
    }
}
