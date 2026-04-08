use multiversx_sc_scenario::scenario::model::{
    BytesValue, CheckAccounts, CheckStorage, CheckValue,
};

use super::test_generator::TestGenerator;

impl TestGenerator {
    pub(super) fn generate_check_state(&mut self, comment: Option<&str>, accounts: &CheckAccounts) {
        if let Some(comment_text) = comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        for (address_key, account) in &accounts.accounts {
            let address_expr = self.format_address(&address_key.original);

            // Check if we need to check storage
            if let CheckStorage::Equal(ref storage_details) = account.storage {
                if !storage_details.storages.is_empty() {
                    self.step_writeln(format!(
                        "    world\n        .check_account({})",
                        address_expr
                    ));

                    for (key, value) in &storage_details.storages {
                        let value_str = Self::format_check_value_for_storage(value);
                        self.step_writeln(format!(
                            "        .check_storage(\"{}\", \"{}\")",
                            Self::escape_string(&key.original),
                            Self::escape_string(&value_str),
                        ));
                    }

                    self.step_writeln("        ;");
                }
            }
        }

        self.step_writeln("");
    }

    fn format_check_value_for_storage(value: &CheckValue<BytesValue>) -> String {
        match value {
            CheckValue::Star => "*".to_string(),
            CheckValue::Equal(v) => Self::format_value_as_string(&v.original),
        }
    }
}
