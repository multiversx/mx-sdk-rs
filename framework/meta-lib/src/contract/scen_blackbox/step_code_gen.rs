use multiversx_chain_scenario_format::serde_raw::{
    CheckAccountsRaw, StepRaw, TxCallRaw, TxDeployRaw, TxExpectRaw, TxQueryRaw,
};
use std::io::Write;

use super::{scenario_loader::scenario_to_function_name, test_gen::TestGenerator};

impl<'a> TestGenerator<'a> {
    /// Generates code for a single step
    pub fn generate_step_code(&mut self, step: &StepRaw) {
        match step {
            StepRaw::ExternalSteps { comment, path } => {
                self.generate_external_steps(path, comment.as_deref());
            }
            StepRaw::SetState {
                comment,
                accounts,
                new_addresses,
                ..
            } => {
                self.generate_set_state(comment.as_deref(), accounts, new_addresses);
            }
            StepRaw::ScDeploy {
                id,
                tx_id,
                comment,
                tx,
                expect,
                ..
            } => {
                self.generate_sc_deploy(
                    id.as_ref().or(tx_id.as_ref()),
                    comment.as_deref(),
                    tx,
                    expect.as_ref(),
                );
            }
            StepRaw::ScCall {
                id,
                tx_id,
                comment,
                tx,
                expect,
                ..
            } => {
                self.generate_sc_call(
                    id.as_ref().or(tx_id.as_ref()),
                    comment.as_deref(),
                    tx,
                    expect.as_ref(),
                );
            }
            StepRaw::ScQuery {
                id,
                tx_id,
                comment,
                tx,
                expect,
                ..
            } => {
                self.generate_sc_query(
                    id.as_ref().or(tx_id.as_ref()),
                    comment.as_deref(),
                    tx,
                    expect.as_ref(),
                );
            }
            StepRaw::CheckState { comment, accounts } => {
                self.generate_check_state(comment.as_deref(), accounts);
            }
            StepRaw::Transfer { .. } => {
                writeln!(self.file, "    // TODO: Transfer step").unwrap();
            }
            StepRaw::ValidatorReward { .. } => {
                writeln!(self.file, "    // TODO: ValidatorReward step").unwrap();
            }
            StepRaw::DumpState { .. } => {
                writeln!(self.file, "    // TODO: DumpState step").unwrap();
            }
        }
    }

    fn generate_external_steps(&mut self, path: &str, comment: Option<&str>) {
        if let Some(comment_text) = comment {
            writeln!(self.file, "    // {}", comment_text).unwrap();
        }

        let scenario_name = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(path);

        let steps_function_name = format!("{}_steps", scenario_to_function_name(scenario_name));

        writeln!(self.file, "    {}(world);", steps_function_name).unwrap();
        writeln!(self.file).unwrap();
    }

    fn generate_set_state(
        &mut self,
        comment: Option<&str>,
        accounts: &std::collections::BTreeMap<
            String,
            multiversx_chain_scenario_format::serde_raw::AccountRaw,
        >,
        new_addresses: &[multiversx_chain_scenario_format::serde_raw::NewAddressRaw],
    ) {
        if let Some(comment_text) = comment {
            writeln!(self.file, "    // {}", comment_text).unwrap();
        }

        // Generate account setup
        for (address_key, account) in accounts {
            let address_expr = Self::format_address(address_key);

            // Check if we need to set anything
            let has_nonce = account
                .nonce
                .as_ref()
                .map(|v| !Self::is_default_value(v))
                .unwrap_or(false);
            let has_balance = account
                .balance
                .as_ref()
                .map(|v| !Self::is_default_value(v))
                .unwrap_or(false);

            if has_nonce || has_balance {
                write!(self.file, "    world.account({})", address_expr).unwrap();

                if has_nonce {
                    if let Some(nonce) = &account.nonce {
                        writeln!(self.file, ".nonce({})", Self::format_value(nonce)).unwrap();
                        write!(self.file, "        ").unwrap();
                    }
                }

                if has_balance {
                    if let Some(balance) = &account.balance {
                        writeln!(self.file, ".balance({})", Self::format_value(balance)).unwrap();
                        write!(self.file, "        ").unwrap();
                    }
                }

                writeln!(self.file, ";").unwrap();
            }
        }

        // Store new addresses for later use in deploy steps
        for new_addr in new_addresses {
            let creator_key = new_addr.creator_address.to_concatenated_string();
            let new_address_key = new_addr.new_address.to_concatenated_string();
            self.new_address_map.insert(creator_key, new_address_key);
        }

        writeln!(self.file).unwrap();
    }

    fn generate_sc_deploy(
        &mut self,
        id: Option<&String>,
        comment: Option<&str>,
        tx: &TxDeployRaw,
        _expect: Option<&TxExpectRaw>,
    ) {
        if let Some(comment_text) = comment {
            writeln!(self.file, "    // {}", comment_text).unwrap();
        }

        writeln!(self.file, "    world.tx()").unwrap();

        if let Some(id_val) = id {
            writeln!(self.file, "        .id(\"{}\")", id_val).unwrap();
        }

        writeln!(
            self.file,
            "        .from({})",
            Self::format_address_value(&tx.from)
        )
        .unwrap();
        write!(self.file, "        ").unwrap();

        let proxy_type = self.generate_proxy_type();
        writeln!(self.file, ".typed({})", proxy_type).unwrap();
        write!(self.file, "        ").unwrap();

        write!(self.file, ".init(").unwrap();
        for (i, arg) in tx.arguments.iter().enumerate() {
            if i > 0 {
                write!(self.file, ", ").unwrap();
            }
            write!(self.file, "{}", Self::format_value(arg)).unwrap();
        }
        writeln!(self.file, ")").unwrap();
        write!(self.file, "        ").unwrap();

        writeln!(self.file, ".code(CODE_PATH)").unwrap();
        write!(self.file, "        ").unwrap();

        // Add new_address if we have a prediction from setState
        let from_address = tx.from.to_concatenated_string();
        if let Some(new_address) = self.new_address_map.get(&from_address) {
            // Format as TestSCAddress::new("name") if it's sc:name
            let address_expr = Self::format_address(new_address);
            writeln!(self.file, ".new_address({})", address_expr).unwrap();
            write!(self.file, "        ").unwrap();
        }

        writeln!(self.file, ".run();").unwrap();
        writeln!(self.file).unwrap();
    }

    fn generate_sc_call(
        &mut self,
        id: Option<&String>,
        comment: Option<&str>,
        tx: &TxCallRaw,
        _expect: Option<&TxExpectRaw>,
    ) {
        if let Some(comment_text) = comment {
            writeln!(self.file, "    // {}", comment_text).unwrap();
        }

        writeln!(self.file, "    world.tx()").unwrap();

        if let Some(id_val) = id {
            writeln!(self.file, "        .id(\"{}\")", id_val).unwrap();
        }

        writeln!(
            self.file,
            "        .from({})",
            Self::format_address_value(&tx.from)
        )
        .unwrap();

        writeln!(
            self.file,
            "        .to({})",
            Self::format_address_value(&tx.to)
        )
        .unwrap();
        write!(self.file, "        ").unwrap();

        let proxy_type = self.generate_proxy_type();
        writeln!(self.file, ".typed({})", proxy_type).unwrap();
        write!(self.file, "        ").unwrap();

        // Map the endpoint name from scenario to Rust method name
        let rust_method_name = self.map_endpoint_name(&tx.function);
        write!(self.file, ".{}(", rust_method_name).unwrap();
        for (i, arg) in tx.arguments.iter().enumerate() {
            if i > 0 {
                write!(self.file, ", ").unwrap();
            }
            write!(self.file, "{}", Self::format_value(arg)).unwrap();
        }
        writeln!(self.file, ")").unwrap();
        write!(self.file, "        ").unwrap();

        writeln!(self.file, ".run();").unwrap();
        writeln!(self.file).unwrap();
    }

    fn generate_sc_query(
        &mut self,
        id: Option<&String>,
        comment: Option<&str>,
        tx: &TxQueryRaw,
        expect: Option<&TxExpectRaw>,
    ) {
        if let Some(comment_text) = comment {
            writeln!(self.file, "    // {}", comment_text).unwrap();
        }

        writeln!(self.file, "    world.query()").unwrap();

        if let Some(id_val) = id {
            writeln!(self.file, "        .id(\"{}\")", id_val).unwrap();
        }

        writeln!(
            self.file,
            "        .to({})",
            Self::format_address_value(&tx.to)
        )
        .unwrap();
        write!(self.file, "        ").unwrap();

        let proxy_type = self.generate_proxy_type();
        writeln!(self.file, ".typed({})", proxy_type).unwrap();
        write!(self.file, "        ").unwrap();

        // Map the endpoint name from scenario to Rust method name
        let rust_method_name = self.map_endpoint_name(&tx.function);
        write!(self.file, ".{}(", rust_method_name).unwrap();
        for (i, arg) in tx.arguments.iter().enumerate() {
            if i > 0 {
                write!(self.file, ", ").unwrap();
            }
            write!(self.file, "{}", Self::format_value(arg)).unwrap();
        }
        writeln!(self.file, ")").unwrap();
        write!(self.file, "        ").unwrap();

        // Add returns if we have expected output
        if let Some(expect_val) = expect {
            if let multiversx_chain_scenario_format::serde_raw::CheckValueListRaw::CheckList(
                ref out_values,
            ) = expect_val.out
            {
                write!(self.file, ".returns(ExpectValue(").unwrap();
                for (i, out) in out_values.iter().enumerate() {
                    if i > 0 {
                        write!(self.file, ", ").unwrap();
                    }
                    write!(self.file, "{}", Self::format_check_value(out)).unwrap();
                }
                writeln!(self.file, "))").unwrap();
                write!(self.file, "        ").unwrap();
            }
        }

        writeln!(self.file, ".run();").unwrap();
        writeln!(self.file).unwrap();
    }

    fn generate_check_state(&mut self, comment: Option<&str>, accounts: &CheckAccountsRaw) {
        if let Some(comment_text) = comment {
            writeln!(self.file, "    // {}", comment_text).unwrap();
        }

        for (address_key, account) in &accounts.accounts {
            let address_expr = Self::format_address(address_key);

            // Check if we need to check storage
            if let multiversx_chain_scenario_format::serde_raw::CheckStorageRaw::Equal(
                ref storage_details,
            ) = account.storage
            {
                if !storage_details.storages.is_empty() {
                    writeln!(self.file, "    world.check_account({})", address_expr).unwrap();

                    for (key, value) in &storage_details.storages {
                        writeln!(
                            self.file,
                            "        .check_storage(\"{}\", \"{}\")",
                            key,
                            Self::format_check_value_as_string(value)
                        )
                        .unwrap();
                    }

                    writeln!(self.file, "        ;").unwrap();
                }
            }
        }

        writeln!(self.file).unwrap();
    }

    fn format_address(addr: &str) -> String {
        // Remove quotes if present
        let clean = addr.trim_matches('"');

        // Handle address: and sc: prefixes
        if let Some(name) = clean.strip_prefix("address:") {
            format!("TestAddress::new(\"{}\")", name)
        } else if let Some(name) = clean.strip_prefix("sc:") {
            format!("TestSCAddress::new(\"{}\")", name)
        } else if clean.starts_with("0x") || clean.starts_with("0X") {
            // Hex address - generate Address::from_hex(...)
            format!("Address::from_hex(\"{}\")", clean)
        } else if clean.len() == 64 && clean.chars().all(|c| c.is_ascii_hexdigit()) {
            // Hex address without 0x prefix
            format!("Address::from_hex(\"{}\")", clean)
        } else {
            // Raw address - wrap in ScenarioValueRaw
            format!("ScenarioValueRaw::str(\"{}\")", clean)
        }
    }

    fn format_address_value(
        value: &multiversx_chain_scenario_format::serde_raw::ValueSubTree,
    ) -> String {
        use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
        match value {
            ValueSubTree::Str(s) => Self::format_address(s),
            _ => {
                // Fallback for non-string addresses
                Self::format_value(value)
            }
        }
    }

    fn format_value(value: &multiversx_chain_scenario_format::serde_raw::ValueSubTree) -> String {
        use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
        match value {
            ValueSubTree::Str(s) => {
                format!("ScenarioValueRaw::str(\"{}\")", Self::escape_string(s))
            }
            ValueSubTree::List(items) => {
                if items.is_empty() {
                    "ScenarioValueRaw::list(&[])".to_string()
                } else {
                    let formatted_items: Vec<String> =
                        items.iter().map(Self::format_value).collect();
                    format!("ScenarioValueRaw::list(&[{}])", formatted_items.join(", "))
                }
            }
            ValueSubTree::Map(map) => {
                if map.is_empty() {
                    "ScenarioValueRaw::map(&[])".to_string()
                } else {
                    let formatted_entries: Vec<String> = map
                        .iter()
                        .map(|(k, v)| {
                            format!(
                                "(\"{}\", {})",
                                Self::escape_string(k),
                                Self::format_value(v)
                            )
                        })
                        .collect();
                    format!("ScenarioValueRaw::map(&[{}])", formatted_entries.join(", "))
                }
            }
        }
    }

    fn format_check_value(
        value: &multiversx_chain_scenario_format::serde_raw::CheckBytesValueRaw,
    ) -> String {
        use multiversx_chain_scenario_format::serde_raw::CheckBytesValueRaw;
        match value {
            CheckBytesValueRaw::Unspecified => "ScenarioValueRaw::str(\"\")".to_string(),
            CheckBytesValueRaw::Star => "ScenarioValueRaw::str(\"*\")".to_string(),
            CheckBytesValueRaw::Equal(v) => Self::format_value(v),
        }
    }

    fn escape_string(s: &str) -> String {
        s.replace('\\', "\\\\").replace('"', "\\\"")
    }

    /// Maps an endpoint name from the scenario (usually camelCase) to the Rust method name (snake_case)
    /// by looking it up in the contract ABI.
    fn map_endpoint_name(&self, scenario_endpoint_name: &str) -> String {
        // Look up the endpoint in the ABI
        for endpoint in &self.abi.endpoints {
            if endpoint.name == scenario_endpoint_name {
                return endpoint.rust_method_name.clone();
            }
        }

        // If not found, return the original name (might be a special case or already in the correct format)
        scenario_endpoint_name.to_string()
    }

    fn generate_proxy_type(&self) -> String {
        // Convert crate name to CamelCase for the proxy struct name
        let struct_name = self
            .crate_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        format!("{}_proxy::{}Proxy", self.crate_name, struct_name)
    }

    fn format_check_value_as_string(
        value: &multiversx_chain_scenario_format::serde_raw::CheckBytesValueRaw,
    ) -> String {
        use multiversx_chain_scenario_format::serde_raw::CheckBytesValueRaw;
        match value {
            CheckBytesValueRaw::Unspecified => String::new(),
            CheckBytesValueRaw::Star => "*".to_string(),
            CheckBytesValueRaw::Equal(v) => Self::format_value_as_string(v),
        }
    }

    fn format_value_as_string(
        value: &multiversx_chain_scenario_format::serde_raw::ValueSubTree,
    ) -> String {
        use multiversx_chain_scenario_format::serde_raw::ValueSubTree;
        match value {
            ValueSubTree::Str(s) => s.clone(),
            ValueSubTree::List(items) => {
                let strs: Vec<String> = items.iter().map(Self::format_value_as_string).collect();
                strs.join("|")
            }
            ValueSubTree::Map(map) => {
                let strs: Vec<String> = map.values().map(Self::format_value_as_string).collect();
                strs.join("|")
            }
        }
    }

    fn is_default_value(value: &multiversx_chain_scenario_format::serde_raw::ValueSubTree) -> bool {
        let val_str = format!("{:?}", value);
        val_str == "\"0\"" || val_str == "\"\"" || val_str.is_empty()
    }
}
