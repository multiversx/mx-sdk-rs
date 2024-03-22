use std::{fs::File, io::Write};

use super::proxy_naming::{proxy_methods_type_name, proxy_type_name};

const PREFIX_AUTO_GENERATED: &str = "////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////
";

const PRELUDE: &str = "#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;";

pub(crate) fn write_header(file: &mut File) {
    writeln!(file, "{PREFIX_AUTO_GENERATED}").unwrap();
    writeln!(file, r#"{PRELUDE}"#).unwrap();
}

pub(crate) fn write_tx_proxy_type_def(file: &mut File, name: &str) {
    let proxy_type_name = proxy_type_name(name);
    writeln!(
        file,
        "
pub struct {proxy_type_name};"
    )
    .unwrap();
}

pub(crate) fn write_impl_for_tx_proxy(file: &mut File, name: &str) {
    let proxy_type_name = proxy_type_name(name);
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"
impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for {proxy_type_name}
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    type TxProxyMethods = {proxy_methods_type_name}<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {{
        {proxy_methods_type_name} {{ wrapped_tx: tx }}
    }}
}}"#
    )
    .unwrap();
}

pub(crate) fn write_struct_tx_proxy_methods(file: &mut File, name: &str) {
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"
pub struct {proxy_methods_type_name}<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}}"#
    )
    .unwrap();
}
