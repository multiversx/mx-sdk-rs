use multiversx_sc_scenario::scenario::model::{
    BigUintValue, CheckValue, ScCallStep, ScDeployStep, ScQueryStep, TxESDT, TxExpect,
};
use multiversx_sc_scenario::scenario_format::serde_raw::ValueSubTree;

use super::test_generator::TestGenerator;

impl TestGenerator {
    pub(super) fn generate_sc_deploy(&mut self, sc_deploy: &ScDeployStep) {
        let tx = &sc_deploy.tx;

        if let Some(comment_text) = &sc_deploy.comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        self.step_writeln("    world");
        self.step_writeln("        .tx()");

        if let Some(id_val) = &sc_deploy.tx_id {
            self.step_writeln(format!("        .id(\"{}\")", id_val));
        }

        let from_addr = self.format_address_value(&tx.from);
        self.step_writeln(format!("        .from({})", from_addr));
        self.step_write("        ");

        let proxy_type = self.generate_proxy_type();
        self.step_writeln(format!(".typed({})", proxy_type));
        self.step_write("        ");

        let inputs = self.abi.find_constructor_inputs();
        let formatted_args = self.format_inputs(&tx.arguments, inputs.as_deref());
        self.step_writeln(format!(".init({})", formatted_args));
        self.step_write("        ");

        // Generate EGLD payment for deploy (no ESDT support on deploy)
        self.generate_egld_payment(&tx.egld_value);

        // Generate code path from the contract_code field
        let code_path_expr = tx.contract_code.original.to_concatenated_string();
        let code_path_const = self.consts.format_code_path(&code_path_expr);
        self.step_writeln(format!(".code({})", code_path_const));
        self.step_write("        ");

        // Add new_address if we have a prediction from setState
        let from_address = tx.from.original.to_concatenated_string();
        if let Some(new_address) = self.new_address_map.get(&from_address).cloned() {
            // Format as TestSCAddress::new("name") if it's sc:name
            let address_expr = self.format_address(&new_address);
            self.step_writeln(format!(".new_address({})", address_expr));
            self.step_write("        ");
        }

        self.generate_expect_error(sc_deploy.expect.as_ref());

        self.step_writeln(".run();");
        self.step_writeln("");
    }

    pub(super) fn generate_sc_call(&mut self, sc_call: &ScCallStep) {
        let tx = &sc_call.tx;

        if let Some(comment_text) = &sc_call.comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        self.step_writeln("    world");
        self.step_writeln("        .tx()");

        if let Some(id_val) = &sc_call.tx_id {
            self.step_writeln(format!("        .id(\"{}\")", id_val));
        }

        let from_addr = self.format_address_value(&tx.from);
        self.step_writeln(format!("        .from({})", from_addr));

        let to_addr = self.format_address_value(&tx.to);
        self.step_writeln(format!("        .to({})", to_addr));
        self.step_write("        ");

        let proxy_type = self.generate_proxy_type();
        self.step_writeln(format!(".typed({})", proxy_type));
        self.step_write("        ");

        // Map the endpoint name from scenario to Rust method name
        let inputs = self.abi.find_endpoint_inputs(&tx.function);
        let formatted_args = self.format_inputs(&tx.arguments, inputs.as_deref());
        let rust_method_name = self.abi.find_endpoint_rust_name(&tx.function);
        self.step_writeln(format!(".{}({})", rust_method_name, formatted_args));
        self.step_write("        ");

        // Generate payments
        self.generate_payments(&tx.egld_value, &tx.esdt_value);

        self.generate_expect_error(sc_call.expect.as_ref());
        self.generate_expect_results(sc_call.expect.as_ref(), &tx.function);

        self.step_writeln(".run();");
        self.step_writeln("");
    }

    pub(super) fn generate_sc_query(&mut self, sc_query: &ScQueryStep) {
        let tx = &sc_query.tx;
        let expect = sc_query.expect.as_ref();

        if let Some(comment_text) = &sc_query.comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        self.step_writeln("    world");
        self.step_writeln("        .query()");

        if let Some(id_val) = &sc_query.tx_id {
            self.step_writeln(format!("        .id(\"{}\")", id_val));
        }

        let to_addr = self.format_address_value(&tx.to);
        self.step_writeln(format!("        .to({})", to_addr));
        self.step_write("        ");

        let proxy_type = self.generate_proxy_type();
        self.step_writeln(format!(".typed({})", proxy_type));
        self.step_write("        ");

        // Map the endpoint name from scenario to Rust method name
        let inputs = self.abi.find_endpoint_inputs(&tx.function);
        let formatted_args = self.format_inputs(&tx.arguments, inputs.as_deref());
        let rust_method_name = self.abi.find_endpoint_rust_name(&tx.function);
        self.step_writeln(format!(".{}({})", rust_method_name, formatted_args));
        self.step_write("        ");

        self.generate_expect_results(expect, &tx.function);

        self.step_writeln(".run();");
        self.step_writeln("");
    }

    /// Generates `.payment(...)` calls for EGLD and ESDT transfers.
    pub(super) fn generate_payments(&mut self, egld_value: &BigUintValue, esdt_value: &[TxESDT]) {
        use multiversx_sc_scenario::num_bigint::BigUint;

        if !esdt_value.is_empty() {
            // ESDT payments (may include EGLD-000000)
            for esdt in esdt_value {
                self.generate_esdt_payment(esdt);
            }
        } else if egld_value.value > BigUint::from(0u32) {
            // Plain EGLD payment
            self.generate_egld_payment(egld_value);
        }
    }

    /// Generates a `.payment(Payment::try_new(...).unwrap())` call for an EGLD value.
    pub(super) fn generate_egld_payment(&mut self, egld_value: &BigUintValue) {
        use multiversx_sc_scenario::num_bigint::BigUint;

        if egld_value.value > BigUint::from(0u32) {
            let amount = Self::format_biguint_value(&egld_value.value);
            self.step_writeln(format!(
                ".payment(Payment::try_new(TestTokenId::EGLD_000000, 0, {}).unwrap())",
                amount
            ));
            self.step_write("        ");
        }
    }

    /// Generates a `.payment(Payment::try_new(...).unwrap())` call for an ESDT transfer.
    fn generate_esdt_payment(&mut self, esdt: &TxESDT) {
        let nonce = esdt.nonce.value;
        let amount = Self::format_biguint_value(&esdt.esdt_value.value);

        if esdt.is_egld() {
            self.step_writeln(format!(
                ".payment(Payment::try_new(TestTokenId::EGLD_000000, {}, {}).unwrap())",
                nonce, amount
            ));
        } else {
            let token_const = self.format_token_id_value(&esdt.esdt_token_identifier);
            self.step_writeln(format!(
                ".payment(Payment::try_new({}, {}, {}).unwrap())",
                token_const, nonce, amount
            ));
        }
        self.step_write("        ");
    }

    /// Generates `.with_result(ExpectError(status, "message"))` or `.with_result(ExpectStatus(status))`
    /// when the expected status is non-zero (the latter when the expected message is `*`).
    pub(super) fn generate_expect_error(&mut self, expect: Option<&TxExpect>) {
        let Some(expect_val) = expect else {
            return;
        };

        // Extract status code; skip if it's "*" or 0
        let status_code = match &expect_val.status {
            CheckValue::Equal(u64_val) => u64_val.value,
            CheckValue::Star => return,
        };

        if status_code == 0 {
            return;
        }

        // Extract message string
        match &expect_val.message {
            CheckValue::Equal(bytes_val) => {
                let message = match &bytes_val.original {
                    ValueSubTree::Str(s) => {
                        // Strip "str:" prefix if present
                        s.strip_prefix("str:").unwrap_or(s).to_string()
                    }
                    _ => String::new(),
                };
                self.step_writeln(format!(
                    ".with_result(ExpectError({}, \"{}\"))",
                    status_code,
                    Self::escape_string(&message)
                ));
            }
            CheckValue::Star => {
                self.step_writeln(format!(".with_result(ExpectStatus({status_code}))"));
            }
        };

        self.step_write("        ");
    }

    /// Generates `.returns(ExpectValue(...))` when the expected status is 0 (success)
    /// and the scenario specifies concrete `out` values (not `"*"`).
    ///
    /// Uses ABI output type information to produce typed Rust literals, falling back
    /// to `ScenarioValueRaw` when ABI info is unavailable.
    pub(super) fn generate_expect_results(
        &mut self,
        expect: Option<&TxExpect>,
        endpoint_name: &str,
    ) {
        let Some(expect_val) = expect else {
            return;
        };

        // Skip if error is expected
        match &expect_val.status {
            CheckValue::Equal(u64_val) if u64_val.value != 0 => return,
            _ => {}
        }

        // Skip if out is Star (ignore) or empty
        let out_values = match &expect_val.out {
            CheckValue::Star => return,
            CheckValue::Equal(values) if values.is_empty() => return,
            CheckValue::Equal(values) => values,
        };

        // Skip if any individual output is Star (can't do a partial typed check)
        if out_values.iter().any(|v| matches!(v, CheckValue::Star)) {
            return;
        }

        let formatted = self.format_out_values(out_values, endpoint_name);
        self.step_writeln(format!(".returns(ExpectValue({}))", formatted));
        self.step_write("        ");
    }
}
