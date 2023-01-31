use std::{fs::File, io::Write};

pub(crate) fn write_test_setup_imports(file: &mut File, contract_module_name: &str) {
    writeln!(
        file,
        "use std::{{cell::RefCell, rc::Rc}};

use {contract_module_name}::ProxyTrait as _;
use {contract_module_name}::*;

use multiversx_sc::{{types::*, codec::multi_types::*}};
use multiversx_sc_scenario::{{*, testing_framework::*}};

type RustBigUint = num_bigint::BigUint;
"
    )
    .unwrap();
}

pub(crate) fn write_test_setup_struct_declaration(
    file: &mut File,
    contract_crate_name: &str,
    struct_name: &str,
    builder_func_name: &str,
) {
    writeln!(
        file,
        "pub struct {struct_name}<{builder_func_name}>
where
    {builder_func_name}: 'static + Copy + Fn() -> {contract_crate_name}::ContractObj<DebugApi>,
{{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub owner: Address,
    pub sc_wrapper:
        ContractObjWrapper<{contract_crate_name}::ContractObj<DebugApi>, {builder_func_name}>,
}}

impl<{builder_func_name}> {struct_name}<{builder_func_name}>
where
    {builder_func_name}: 'static + Copy + Fn() -> {contract_crate_name}::ContractObj<DebugApi>,
{{"
    )
    .unwrap();
}
