use multiversx_sc_scenario::scenario::model::{BlockInfo, SetStateStep};

use super::{num_format, test_generator::TestGenerator};

impl TestGenerator {
    pub(super) fn generate_set_state(&mut self, set_state: &SetStateStep) {
        if let Some(comment_text) = &set_state.comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        // Generate current block info
        if let Some(block_info) = set_state.current_block_info.as_ref() {
            self.generate_block_info(block_info);
        }

        // Generate account setup
        for (address_key, account) in &set_state.accounts {
            let address_expr = self.format_address(&address_key.original);

            self.step_write(format!("    world.account({})", address_expr));

            if let Some(nonce) = &account.nonce {
                self.step_writeln(format!(".nonce({})", Self::format_nonce_value(nonce.value)));
                self.step_write("        ");
            }

            if let Some(balance) = &account.balance {
                self.step_writeln(format!(
                    ".balance({})",
                    Self::format_balance_value(&balance.value)
                ));
                self.step_write("        ");
            }

            if let Some(code) = &account.code {
                let code_path_expr = code.original.to_concatenated_string();
                let code_path_const = self.consts.format_code_path(&code_path_expr);
                self.step_writeln(format!(".code({})", code_path_const));
                self.step_write("        ");
            }

            for (token_key, esdt) in &account.esdt {
                let token_const = self.format_token_id_key(token_key);
                self.generate_esdt_balance_calls(&token_const, esdt);
            }

            self.step_writeln(";");
        }

        // Store new addresses for later use in deploy steps
        for new_addr in &set_state.new_addresses {
            let creator_key = new_addr.creator_address.original.to_concatenated_string();
            let new_address_key = new_addr.new_address.original.to_concatenated_string();
            self.new_address_map.insert(creator_key, new_address_key);
        }

        self.step_writeln("");
    }

    /// Generates `world.current_block().block_timestamp_millis(...)` and similar block info setters.
    fn generate_block_info(&mut self, block_info: &BlockInfo) {
        // blockTimestampMs takes priority over blockTimestamp
        if let Some(ref ts_ms) = block_info.block_timestamp_ms {
            let value = num_format::format_unsigned(&ts_ms.value.to_be_bytes(), "u64");
            self.step_writeln(format!(
                "    world.current_block().block_timestamp_millis(TimestampMillis::new({}));",
                value
            ));
        } else if let Some(ref ts) = block_info.block_timestamp {
            let value = num_format::format_unsigned(&ts.value.to_be_bytes(), "u64");
            self.step_writeln(format!(
                "    world.current_block().block_timestamp_seconds(TimestampSeconds::new({}));",
                value
            ));
        }

        if let Some(ref nonce) = block_info.block_nonce {
            self.step_writeln(format!(
                "    world.current_block().block_nonce({}u64);",
                nonce.value
            ));
        }

        if let Some(ref round) = block_info.block_round {
            self.step_writeln(format!(
                "    world.current_block().block_round({}u64);",
                round.value
            ));
        }

        if let Some(ref epoch) = block_info.block_epoch {
            self.step_writeln(format!(
                "    world.current_block().block_epoch({}u64);",
                epoch.value
            ));
        }
    }

    /// Generates `.esdt_balance(token, amount)` or `.esdt_nft_balance(token, nonce, amount, ())`
    /// calls depending on whether the ESDT instances have non-zero nonces.
    pub(super) fn generate_esdt_balance_calls(
        &mut self,
        token_const: &str,
        esdt: &multiversx_sc_scenario::scenario::model::Esdt,
    ) {
        use multiversx_sc_scenario::scenario::model::Esdt;
        match esdt {
            Esdt::Short(biguint_val) => {
                let amount = Self::format_biguint_value(&biguint_val.value);
                self.step_writeln(format!(".esdt_balance({}, {})", token_const, amount));
                self.step_write("        ");
            }
            Esdt::Full(esdt_obj) => {
                for instance in &esdt_obj.instances {
                    let nonce = instance.nonce.as_ref().map_or(0, |n| n.value);
                    let amount = instance
                        .balance
                        .as_ref()
                        .map(|b| Self::format_biguint_value(&b.value))
                        .unwrap_or_else(|| "0u64".to_string());

                    if nonce == 0 {
                        self.step_writeln(format!(".esdt_balance({}, {})", token_const, amount));
                    } else {
                        self.step_writeln(format!(
                            ".esdt_nft_balance({}, {}, {}, ())",
                            token_const, nonce, amount
                        ));
                    }
                    self.step_write("        ");
                }
            }
        }
    }
}
