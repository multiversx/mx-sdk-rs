use mandos::{
    AddressKey, BytesValue, CheckEsdt, CheckEsdtData, CheckEsdtValues, CheckStorage, CheckValue,
    Checkable,
};

use crate::{
    account_esdt::{AccountEsdt, EsdtData},
    esdt_instance::{EsdtInstance, EsdtInstances},
    verbose_hex, BlockchainMock,
};

pub fn execute(accounts: &mandos::CheckAccounts, state: &mut BlockchainMock) {
    for (expected_address, expected_account) in accounts.accounts.iter() {
        if let Some(account) = state.accounts.get(&expected_address.value.into()) {
            assert!(
                expected_account.nonce.check(account.nonce),
                "bad account nonce. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.nonce,
                account.nonce
            );

            assert!(
                expected_account.balance.check(&account.balance),
                "bad account balance. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.balance,
                account.balance
            );

            assert!(
                expected_account.username.check(&account.username),
                "bad account username. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.username,
                std::str::from_utf8(account.username.as_slice()).unwrap()
            );
            let default_value = &Vec::new();
            let actual_code = account.contract_path.as_ref().unwrap_or(default_value);
            assert!(
                expected_account.code.check(actual_code),
                "bad account code. Address: {}. Want: {}. Have: {}",
                expected_address,
                expected_account.code,
                std::str::from_utf8(actual_code.as_slice()).unwrap()
            );

            if let CheckStorage::Equal(eq) = &expected_account.storage {
                let default_value = &Vec::new();
                for (expected_key, expected_value) in eq.storages.iter() {
                    let actual_value = account
                        .storage
                        .get(&expected_key.value)
                        .unwrap_or(default_value);
                    assert!(
                        expected_value.check(actual_value),
                        "bad storage value. Address: {}. Key: {}. Want: {}. Have: {}",
                        expected_address,
                        expected_key,
                        expected_value,
                        verbose_hex(actual_value)
                    );
                }

                let default_check_value = CheckValue::Equal(BytesValue::empty());
                for (actual_key, actual_value) in account.storage.iter() {
                    let expected_value = eq
                        .storages
                        .get(&actual_key.clone().into())
                        .unwrap_or(&default_check_value);
                    if expected_value.to_string() == default_check_value.to_string()
                        && !eq.other_storages_allowed
                    {
                        assert!(
                            expected_value.check(actual_value),
                            "bad storage value. Address: {}. Key: {}. Want: {}. Have: {}",
                            expected_address,
                            verbose_hex(actual_key),
                            expected_value,
                            verbose_hex(actual_value)
                        );
                    }
                }
                check_account_esdt(&expected_address, &expected_account.esdt, &account.esdt);
            }
        } else {
            assert!(
                accounts.other_accounts_allowed,
                "Expected account not found"
            );
        }
    }
}

pub fn check_account_esdt(address: &AddressKey, expected: &CheckEsdt, actual: &AccountEsdt) {
    match expected {
        CheckEsdt::Equal(eq) => {
            let default_value = &EsdtData::default();
            for expected_value in eq.iter() {
                let actual_value = actual
                    .get_by_identifier(expected_value.token_identifier.value)
                    .unwrap_or(default_value);
                check_esdt_data(
                    address,
                    verbose_hex(&expected_value.token_identifier.value),
                    expected_value,
                    actual_value,
                )
            }

            let default_check_value = CheckEsdtData::default();
            for (actual_key, actual_value) in actual
                .iter()
                .filter(|&(key, val)| !expected.contains_identifier(key))
            {
                check_esdt_data(
                    address,
                    verbose_hex(&actual_value.token_identifier),
                    &default_check_value,
                    actual_value,
                );
            }
        },
        CheckEsdt::Star => {
            // nothing to be done for *
        },
    }
}

pub fn check_esdt_data(
    address: &AddressKey,
    token: String,
    expected: &CheckEsdtData,
    actual: &EsdtData,
) {
    check_token_instances(address, token, &expected.instances, &actual.instances);
    assert!(
        expected.last_nonce.check(actual.last_nonce),
        "bad last nonce. Address: {}. Token Name: {}. Want: {}. Have: {}",
        address,
        token,
        expected.last_nonce,
        &actual.last_nonce
    );
}

pub fn check_token_instances(
    address: &AddressKey,
    token: String,
    expected: &CheckEsdtValues,
    actual: &EsdtInstances,
) {
    match expected {
        CheckEsdtValues::Equal(eq) => {
            let default_value = EsdtInstance::default();
            for expected_value in eq.iter() {
                let actual_value = actual
                    .find_instance_with_nonce(expected_value.nonce.value)
                    .unwrap_or(default_value);
                assert!(
                    expected_value.balance.check(&actual_value.value),
                    "bad esdt value. Address: {}. Token Name: {}. Want: {}. Have: {}",
                    address,
                    token,
                    expected_value.balance,
                    &actual_value.value
                );
            }
        },
        CheckEsdtValues::Star => {
            // nothing to be done for *
        },
    }
}
