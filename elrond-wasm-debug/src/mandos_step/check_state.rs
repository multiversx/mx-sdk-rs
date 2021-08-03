use mandos::{BigUintValue, BytesValue, CheckEsdt, CheckStorage, CheckValue, Checkable};
use num_bigint::BigUint;

use crate::{verbose_hex, BlockchainMock};

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

            if let CheckStorage::Equal(eq) = &expected_account.storage {
                let default_value = &Vec::new();
                for (expected_key, expected_value) in eq.iter() {
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
                        .get(&actual_key.clone().into())
                        .unwrap_or(&default_check_value);
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

            match &expected_account.esdt {
                CheckEsdt::Equal(eq) => {
                    let default_value = &BigUint::from(0u32);
                    for (expected_key, expected_value) in eq.iter() {
                        let actual_value = account
                            .esdt
                            .get(&expected_key.value)
                            .unwrap_or(default_value);
                        assert!(
                            expected_value.check(actual_value),
                            "bad esdt value. Address: {}. Token Name: {}. Want: {}. Have: {}",
                            expected_address,
                            expected_key,
                            expected_value,
                            actual_value
                        );
                    }

                    let default_check_value = CheckValue::Equal(BigUintValue::default());

                    for (actual_key, actual_value) in account.esdt.iter() {
                        let expected_value = eq
                            .get(&actual_key.clone().into())
                            .unwrap_or(&default_check_value);
                        assert!(
                            expected_value.check(actual_value),
                            "bad esdt value. Address: {}. Token: {}. Want: {}. Have: {}",
                            expected_address,
                            verbose_hex(actual_key),
                            expected_value,
                            actual_value
                        );
                    }
                },
                CheckEsdt::Star => {
                    // nothing to be done for *
                },
            }

            if let CheckEsdt::Equal(eq) = &expected_account.esdt {
                let default_value = &BigUint::from(0u32);
                for (expected_key, expected_value) in eq.iter() {
                    let actual_value = account
                        .esdt
                        .get(&expected_key.value)
                        .unwrap_or(default_value);
                    assert!(
                        expected_value.check(actual_value),
                        "bad esdt value. Address: {}. Token Name: {}. Want: {}. Have: {}",
                        expected_address,
                        expected_key,
                        expected_value,
                        actual_value
                    );
                }

                let default_check_value = CheckValue::Equal(BigUintValue::default());

                for (actual_key, actual_value) in account.esdt.iter() {
                    let expected_value = eq
                        .get(&actual_key.clone().into())
                        .unwrap_or(&default_check_value);
                    assert!(
                        expected_value.check(actual_value),
                        "bad esdt value. Address: {}. Token: {}. Want: {}. Have: {}",
                        expected_address,
                        verbose_hex(actual_key),
                        expected_value,
                        actual_value
                    );
                }
            }
        } else if !accounts.other_accounts_allowed {
            panic!("Expected account not found");
        }
    }
}
