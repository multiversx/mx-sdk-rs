use multiversx_sc_scenario::scenario::model::Step;

use super::{scenario_loader::ScenarioFile, test_generator::TestGenerator};

impl TestGenerator {
    // -------------------------------------------------------------------------
    // Step dispatcher
    // -------------------------------------------------------------------------

    /// Generates code for a single step
    pub fn generate_step_code(&mut self, step: &Step) {
        match step {
            Step::ExternalSteps(step_data) => {
                self.generate_external_steps(&step_data.path, step_data.comment.as_deref());
            }
            Step::SetState(set_state) => {
                self.generate_set_state(set_state);
            }
            Step::ScDeploy(sc_deploy) => {
                self.generate_sc_deploy(sc_deploy);
            }
            Step::ScCall(sc_call) => {
                self.generate_sc_call(sc_call);
            }
            Step::ScQuery(sc_query) => {
                self.generate_sc_query(sc_query);
            }
            Step::CheckState(check_state) => {
                self.generate_check_state(check_state.comment.as_deref(), &check_state.accounts);
            }
            Step::Transfer(_transfer) => {
                self.step_writeln("    // TODO: Transfer step");
            }
            Step::ValidatorReward(_) => {
                self.step_writeln("    // TODO: ValidatorReward step");
            }
            Step::DumpState(_) => {
                self.step_writeln("    // TODO: DumpState step");
            }
        }
    }

    fn generate_external_steps(&mut self, path: &str, comment: Option<&str>) {
        if let Some(comment_text) = comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        let file_stem = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(path);

        // Look up generate_test for this file stem so we use the same naming logic
        // as the declaration site, avoiding `_steps_steps` for .steps.json files.
        let generate_test = self
            .scenario_file_map
            .get(file_stem)
            .copied()
            .unwrap_or(true);
        let steps_function_name = ScenarioFile::steps_function_name_of(file_stem, generate_test);

        self.step_writeln(format!("    {}(world);", steps_function_name));
        self.step_writeln("");
    }

    pub(super) fn generate_proxy_type(&self) -> String {
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
}
