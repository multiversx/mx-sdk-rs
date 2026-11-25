//! Text coming from scenario files must only ever show up as inert data in the generated code.

use std::{fs, path::PathBuf, str::FromStr};

use multiversx_sc_meta::cmd::{
    scen_blackbox::{ContractScenarios, TestGenerator},
    scen_test_gen::format_test_fn_rs,
};
use proc_macro2::{TokenStream, TokenTree};
use serde_json::json;

/// Tries to break out of string literals, line comments, block comments and identifiers.
/// If it succeeds in any position, `injected` shows up as an identifier in the generated code.
const PAYLOAD: &str = "\"));\n*/ injected(); /*\\";

/// Shorter variant, because test address names are limited in length.
const SHORT_PAYLOAD: &str = "\");\n*/injected();/*";

fn contains_injected_ident(tokens: TokenStream) -> bool {
    tokens.into_iter().any(|token| match token {
        TokenTree::Ident(ident) => ident == "injected",
        TokenTree::Group(group) => contains_injected_ident(group.stream()),
        _ => false,
    })
}

fn assert_inert(code: &str) {
    syn::parse_file(code).unwrap_or_else(|err| panic!("invalid Rust: {err}\n{code}"));
    let tokens = TokenStream::from_str(code).unwrap();
    assert!(!contains_injected_ident(tokens), "code injected:\n{code}");
}

/// Creates a contract folder containing a single scenario file, and generates the tests for it.
fn generate(test_name: &str, file_name: &str, scenario: serde_json::Value) -> String {
    let contract_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(test_name);
    let scenarios_dir = contract_dir.join("scenarios");
    let _ = fs::remove_dir_all(&contract_dir);
    fs::create_dir_all(&scenarios_dir).unwrap();
    fs::write(
        contract_dir.join("Cargo.toml"),
        "[package]\nname = \"my-contract\"\n",
    )
    .unwrap();
    fs::write(scenarios_dir.join(file_name), scenario.to_string()).unwrap();

    let input = ContractScenarios::load(&contract_dir).expect("no scenarios loaded");
    let mut generator = TestGenerator::new(input.crate_name, Default::default());
    generator.generate_combined_test_content(&input.scenario_files)
}

#[test]
fn blackbox_hostile_values_are_inert() {
    let address = format!("address:{SHORT_PAYLOAD}");
    let sc_address = format!("sc:{SHORT_PAYLOAD}");
    let token = format!("str:{PAYLOAD}");
    let text = format!("str:{PAYLOAD}");

    let scenario = json!({
        "comment": PAYLOAD,
        "steps": [
            {
                "step": "externalSteps",
                "comment": PAYLOAD,
                "path": format!("{SHORT_PAYLOAD}.steps.json")
            },
            {
                "step": "setState",
                "comment": PAYLOAD,
                "accounts": {
                    &address: { "nonce": "1", "esdt": { &token: "5" } },
                    &sc_address: { "balance": "1" }
                }
            },
            {
                "step": "scDeploy",
                "id": PAYLOAD,
                "comment": PAYLOAD,
                "tx": {
                    "from": &address,
                    "contractCode": "",
                    "arguments": [&text, &text],
                    "gasLimit": "0"
                }
            },
            {
                "step": "scCall",
                "id": PAYLOAD,
                "comment": PAYLOAD,
                "tx": {
                    "from": &address,
                    "to": &sc_address,
                    "function": "someEndpoint",
                    "esdtValue": [{ "tokenIdentifier": &token, "value": "1" }],
                    "arguments": [&text],
                    "gasLimit": "0"
                },
                "expect": { "status": "4", "message": &text }
            },
            {
                "step": "scQuery",
                "id": PAYLOAD,
                "comment": PAYLOAD,
                "tx": {
                    "to": &sc_address,
                    "function": "someView",
                    "arguments": [&text]
                },
                "expect": { "status": "0", "out": [&text] }
            },
            {
                "step": "checkState",
                "comment": PAYLOAD,
                "accounts": {
                    &sc_address: { "storage": { &text: &text } }
                }
            }
        ]
    });

    let code = generate("hostile_values", "hostile.scen.json", scenario);
    assert_inert(&code);
}

#[test]
fn blackbox_hostile_file_name_is_inert() {
    let code = generate(
        "hostile_file_name",
        "1\"(){};injected();.scen.json",
        json!({ "steps": [] }),
    );
    assert_inert(&code);
}

#[test]
fn blackbox_similar_names_get_distinct_constants() {
    let scenario = json!({
        "steps": [{
            "step": "setState",
            "accounts": {
                "address:user-a": {},
                "address:user_a": {},
                "sc:user.a": {}
            }
        }]
    });

    let code = generate("similar_names", "similar.scen.json", scenario);
    assert_inert(&code);
    for expected in [
        "TestAddress::new(\"user-a\")",
        "TestAddress::new(\"user_a\")",
        "TestSCAddress::new(\"user.a\")",
        "const USER_A_ADDRESS:",
        "const USER_A_ADDRESS_2:",
        "const USER_A_ADDRESS_3:",
    ] {
        assert!(code.contains(expected), "missing {expected}:\n{code}");
    }
}

#[test]
#[should_panic(expected = "is not a valid Rust method name")]
fn blackbox_hostile_function_name_is_rejected() {
    let scenario = json!({
        "steps": [{
            "step": "scQuery",
            "tx": {
                "to": "sc:contract",
                "function": "view(); injected",
                "arguments": []
            }
        }]
    });

    generate("hostile_function_name", "hostile.scen.json", scenario);
}

#[test]
fn test_gen_hostile_file_name_is_inert() {
    for file_name in [PAYLOAD, SHORT_PAYLOAD, "1st", "a.b#c", "x() {} fn injected"] {
        assert_inert(&format!("#[test]{}", format_test_fn_rs(file_name, false)));
    }
}
