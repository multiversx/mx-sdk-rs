use std::{fs::File, io::Write};

use crate::generate_snippets::snippet_gen_common::write_newline;

use super::test_gen_common::{capitalize_first_letter, to_camel_case};

pub(crate) fn write_test_setup_imports(file: &mut File, contract_module_name: &str) {
    writeln!(
        file,
        "use std::{{cell::RefCell, rc::Rc}};

use {contract_module_name}::ProxyTrait as _;
use {contract_module_name}::*;

use multiversx_sc::{{types::*, codec::multi_types::*}};
use multiversx_sc_scenario::{{*, testing_framework::*}};
"
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_test_setup_struct_declaration(file: &mut File, contract_crate_name: &str) {
    let mut base_name = to_camel_case(contract_crate_name.to_string());
    capitalize_first_letter(&mut base_name);

    let struct_name = base_name.clone() + "Setup";
    let builder_func_name = base_name.clone() + "ObjBuilder";

    writeln!(
        file,
        "pub struct {struct_name}<{builder_func_name}>
where
    {builder_func_name}: 'static + Copy + Fn() -> {contract_crate_name}::ContractObj<DebugApi>,
{{"
    )
    .unwrap();

    write_newline(file);
}
