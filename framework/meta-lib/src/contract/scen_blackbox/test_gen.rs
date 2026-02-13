use colored::Colorize;
use multiversx_sc::abi::ContractAbi;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::cargo_toml::CargoTomlContents;

use super::scenario_loader::{self, ScenarioFile, scenario_to_function_name};

/// Context for test generation, holding shared parameters and file reference
pub struct TestGenerator<'a> {
    pub crate_name: String,
    pub abi: ContractAbi,
    pub file: &'a mut File,
    /// Maps creator address to expected new address from setState.newAddresses
    pub new_address_map: HashMap<String, String>,
    /// Maps address value to constant name (for TestAddress/TestSCAddress)
    pub test_address_map: HashMap<String, String>,
    /// Maps hex address to constant name
    pub hex_address_map: HashMap<String, String>,
    /// Counter for hex address constants
    pub hex_address_counter: usize,
    /// Maps code path expression to constant name
    pub code_path_map: HashMap<String, String>,
    /// Buffer for constant declarations
    pub const_buffer: String,
    /// Buffer for test and step function code
    pub step_buffer: String,
}

impl<'a> TestGenerator<'a> {
    pub fn new(crate_name: String, abi: ContractAbi, file: &'a mut File) -> Self {
        Self {
            crate_name,
            abi,
            file,
            new_address_map: HashMap::new(),
            test_address_map: HashMap::new(),
            hex_address_map: HashMap::new(),
            hex_address_counter: 0,
            code_path_map: HashMap::new(),
            const_buffer: String::new(),
            step_buffer: String::new(),
        }
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

    /// Writes a formatted line to the const buffer
    pub(super) fn const_writeln(&mut self, text: impl AsRef<str>) {
        self.const_buffer.push_str(text.as_ref());
        self.const_buffer.push('\n');
    }

    /// Derives a constant name from a code path expression
    /// Example: "mxsc:../output/adder.mxsc.json" -> "ADDER_CODE_PATH"
    fn derive_code_path_const_name(code_path_expr: &str) -> String {
        // Extract the filename from the path
        let path_str = code_path_expr.strip_prefix("mxsc:").unwrap_or(code_path_expr);
        let filename = path_str.rsplit('/').next().unwrap_or(path_str);
        
        // Remove .mxsc.json extension
        let contract_name = filename
            .strip_suffix(".mxsc.json")
            .unwrap_or(filename)
            .replace('-', "_");
        
        format!("{}_CODE_PATH", contract_name.to_uppercase())
    }

    /// Formats a code path expression, generating a constant if needed
    pub(super) fn format_code_path(&mut self, code_path_expr: &str) -> String {
        // Check if we already have a constant for this path
        if let Some(const_name) = self.code_path_map.get(code_path_expr) {
            return const_name.clone();
        }

        // Generate a new constant
        let const_name = Self::derive_code_path_const_name(code_path_expr);
        
        // Extract the actual path (strip mxsc: prefix)
        let path_value = code_path_expr.strip_prefix("mxsc:").unwrap_or(code_path_expr);
        // Remove leading ../ if present to make it relative to contract root
        let path_value = path_value.strip_prefix("../").unwrap_or(path_value);
        
        // Generate the constant declaration
        self.const_writeln(format!(
            "const {}: MxscPath = MxscPath::new(\"{}\");",
            const_name, path_value
        ));
        
        // Store in map for future use
        self.code_path_map.insert(code_path_expr.to_string(), const_name.clone());
        
        const_name
    }

    /// Generates the combined test content to the file
    fn generate_combined_test_content(&mut self, scenario_files: &[ScenarioFile]) {

        // Generate a test function and steps function for each scenario
        for scenario_file in scenario_files {
            self.generate_scenario_test(scenario_file);
        }

        // Now write everything to file in order:
        // 1. File header and imports
        writeln!(self.file, "// Auto-generated blackbox tests from scenarios").unwrap();
        writeln!(self.file).unwrap();
        writeln!(self.file, "use multiversx_sc_scenario::imports::*;").unwrap();
        writeln!(self.file).unwrap();
        writeln!(self.file, "use {}::*;", self.crate_name).unwrap();
        writeln!(self.file).unwrap();

        // 2. Constants (code path + addresses)
        if !self.const_buffer.is_empty() {
            write!(self.file, "{}", self.const_buffer).unwrap();
            writeln!(self.file).unwrap();
        }

        // 3. World function
        writeln!(self.file, "fn world() -> ScenarioWorld {{").unwrap();
        writeln!(self.file, "    todo!()").unwrap();
        writeln!(self.file, "}}").unwrap();
        writeln!(self.file).unwrap();

        // 4. Test and step functions
        write!(self.file, "{}", self.step_buffer).unwrap();
    }

    /// Generates test and steps functions for a single scenario
    fn generate_scenario_test(&mut self, scenario_file: &ScenarioFile) {
        let scenario = &scenario_file.scenario;
        let test_name = scenario_to_function_name(&scenario_file.file_name);
        let steps_function_name = format!("{}_steps", test_name);

        // Write scenario comment if available
        if let Some(comment) = &scenario.comment {
            self.step_writeln(format!("// {}", comment));
        }

        // Write test function
        self.step_writeln("#[test]");
        self.step_writeln(format!("fn {}() {{", test_name));
        self.step_writeln("    let mut world = world();");
        self.step_writeln(format!("    {}(&mut world);", steps_function_name));
        self.step_writeln("}");
        self.step_writeln("");

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

/// Main entry point for blackbox test generation
pub fn generate_scen_blackbox_tests(overwrite: bool, abi: &ContractAbi) {
    let contract_path = Path::new("..");
    let scenarios_dir = contract_path.join("scenarios");

    if !scenarios_dir.exists() {
        println!(
            "{}",
            format!(
                "No scenarios folder found at {}, skipping blackbox test generation",
                scenarios_dir.display()
            )
            .yellow()
        );
        return;
    }

    let scenario_files = scenario_loader::load_scenario_files(&scenarios_dir);

    if scenario_files.is_empty() {
        println!(
            "{}",
            format!("No .scen.json files found in {}", scenarios_dir.display()).yellow()
        );
        return;
    }

    println!(
        "{}",
        format!(
            "Found {} scenario file(s), generating blackbox tests...",
            scenario_files.len()
        )
        .green()
    );

    let tests_dir = contract_path.join("tests");
    if !tests_dir.exists() {
        fs::create_dir_all(&tests_dir).expect("Failed to create tests directory");
    }

    // Get crate name from Cargo.toml
    let cargo_toml_path = contract_path.join("Cargo.toml");
    let cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let crate_name = cargo_toml.package_name().replace('-', "_");

    // Create test file
    let test_file_name = format!("{}_blackbox_from_scenarios.rs", crate_name);
    let test_file_path = tests_dir.join(&test_file_name);

    // Check if file exists and overwrite is not enabled
    if test_file_path.exists() && !overwrite {
        println!(
            "  {} {}",
            "Skipping".yellow(),
            format!(
                "{} (already exists, use --overwrite to replace)",
                test_file_name
            )
            .dimmed()
        );
        println!("{}", "Blackbox test generation complete!".green());
        return;
    }

    let mut file = File::create(&test_file_path)
        .unwrap_or_else(|_| panic!("Failed to create test file: {}", test_file_path.display()));

    let mut generator = TestGenerator::new(crate_name, abi.clone(), &mut file);
    generator.generate_combined_test_content(&scenario_files);

    println!("  {} {}", "Generated:".green(), test_file_name);
    println!("{}", "Blackbox test generation complete!".green());
}
