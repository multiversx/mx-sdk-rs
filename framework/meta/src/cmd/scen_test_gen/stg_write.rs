use convert_case::{Case, Casing};

use super::stg_section::ScenarioTestFn;

pub type WriteTestFn = fn(&str, bool) -> String;

pub const WORLD_FN_DECLARATION: &str = "fn world() ->";
pub const DEFAULT_SETUP_GO: &str = "use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}";
pub const DEFAULT_SETUP_RS: &str = "use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    todo!()
}";

pub fn contains_world_fn(s: &str) -> bool {
    s.contains(WORLD_FN_DECLARATION)
}

fn insert_ghost_accounts_snippet(insert_ghost_accounts: bool) -> &'static str {
    if insert_ghost_accounts {
        ".insert_ghost_accounts()"
    } else {
        ""
    }
}

fn scenario_name_to_fn_name(scenario_file_name: &str) -> String {
    scenario_file_name
        .replace(['/', '\\'], "_")
        .to_case(Case::Snake)
}

pub fn format_test_fn_rs(scenario_file_name: &str, insert_ghost_accounts: bool) -> String {
    let ghost = insert_ghost_accounts_snippet(insert_ghost_accounts);
    format!(
        "
fn {}_rs() {{
    world(){ghost}.run(\"scenarios/{}.scen.json\");
}}",
        scenario_name_to_fn_name(scenario_file_name),
        scenario_file_name,
    )
}

pub fn format_test_fn_go(scenario_file_name: &str, insert_ghost_accounts: bool) -> String {
    let ghost = insert_ghost_accounts_snippet(insert_ghost_accounts);
    format!(
        "
fn {}_go() {{
    world(){ghost}.run(\"scenarios/{}.scen.json\");
}}",
        scenario_name_to_fn_name(scenario_file_name),
        scenario_file_name,
    )
}

pub fn format_section(test_fn: &ScenarioTestFn, write_test_fn: WriteTestFn) -> String {
    let mut section_str = test_fn.docs.clone();
    section_str.push_str(&test_fn.test_line);
    if let Some(ignore_line) = &test_fn.ignore_line {
        section_str.push('\n');
        section_str.push_str(ignore_line);
    }
    if let Some(should_panic_line) = &test_fn.should_panic_line {
        section_str.push('\n');
        section_str.push_str(should_panic_line);
    }
    section_str.push_str(&write_test_fn(
        &test_fn.scenario_file_name,
        test_fn.insert_ghost_accounts,
    ));
    section_str
}
