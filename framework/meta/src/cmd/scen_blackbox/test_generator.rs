use multiversx_sc::abi::ContractAbi;
use std::collections::HashMap;

use super::const_state::ConstState;
use super::scenario_loader::ScenarioFile;

const WORLD_FN_TODO: &str = "fn world() -> ScenarioWorld {
    todo!()
}
";

/// Context for test generation, holding shared parameters
pub struct TestGenerator {
    pub crate_name: String,
    pub abi: ContractAbi,
    pub world_fn_declaration: &'static str,
    /// Maps creator address to expected new address from setState.newAddresses
    pub new_address_map: HashMap<String, String>,
    /// All constant-related state
    pub consts: ConstState,
    /// Buffer for test and step function code
    pub step_buffer: String,
    /// Maps scenario file_name (stem) → generate_test flag, for ExternalSteps lookup
    pub scenario_file_map: HashMap<String, bool>,
}

impl TestGenerator {
    pub fn new(crate_name: String, abi: ContractAbi) -> Self {
        Self {
            crate_name,
            abi,
            world_fn_declaration: WORLD_FN_TODO,
            new_address_map: HashMap::new(),
            consts: ConstState::default(),
            step_buffer: String::new(),
            scenario_file_map: HashMap::new(),
        }
    }

    pub fn override_world_fn(&mut self, declaration: &'static str) {
        self.world_fn_declaration = declaration;
    }

    /// Writes a formatted line to the step buffer
    pub(super) fn step_writeln(&mut self, text: impl AsRef<str>) {
        self.step_buffer.push_str(text.as_ref());
        self.step_buffer.push('\n');
    }

    /// Writes text to the step buffer without a newline
    pub(super) fn step_write(&mut self, text: impl AsRef<str>) {
        self.step_buffer.push_str(text.as_ref());
    }

    /// Generates the combined test content, storing it in `output`.
    pub fn generate_combined_test_content(&mut self, scenario_files: &[ScenarioFile]) -> String {
        // Build lookup map so ExternalSteps resolution can find generate_test by file stem
        self.scenario_file_map = scenario_files
            .iter()
            .map(|sf| (sf.file_name.clone(), sf.generate_test))
            .collect();

        // Generate a test function and steps function for each scenario
        for scenario_file in scenario_files {
            self.generate_scenario_test(scenario_file);
        }

        let mut output = String::new();

        // Now assemble everything in order:
        // 1. File header and imports
        output.push_str("// Auto-generated blackbox tests from scenarios\n");
        output.push('\n');
        output.push_str("use multiversx_sc_scenario::imports::*;\n");
        output.push('\n');
        output.push_str(&format!("use {}::*;\n", self.crate_name));
        output.push('\n');

        // 2. Constants (code path + addresses)
        let const_output = self.consts.render_constants();
        if !const_output.is_empty() {
            output.push_str(&const_output);
            output.push('\n');
        }

        // 3. World function
        output.push_str(self.world_fn_declaration);
        output.push('\n');

        // 4. Test and step functions
        let step_buffer = std::mem::take(&mut self.step_buffer);
        output.push_str(&step_buffer);

        output
    }

    /// Generates test and steps functions for a single scenario
    fn generate_scenario_test(&mut self, scenario_file: &ScenarioFile) {
        let scenario = &scenario_file.scenario;
        let test_name = scenario_file.test_name();
        let steps_function_name = scenario_file.steps_function_name();

        // Write scenario comment if available
        if let Some(comment) = &scenario.comment {
            self.step_writeln(format!("// {}", comment));
        }

        if scenario_file.generate_test {
            // Write test function
            self.step_writeln("#[test]");
            self.step_writeln(format!("fn {}() {{", test_name));
            self.step_writeln("    let mut world = world();");
            self.step_writeln(format!("    {}(&mut world);", steps_function_name));
            self.step_writeln("}");
            self.step_writeln("");
        }

        // Write steps function
        self.step_writeln(format!(
            "pub fn {}(world: &mut ScenarioWorld) {{",
            steps_function_name
        ));

        // Generate code for each step (addresses discovered on-the-fly)
        for step in &scenario.steps {
            self.generate_step_code(step);
        }

        self.step_writeln("}");
        self.step_writeln("");
    }
}
