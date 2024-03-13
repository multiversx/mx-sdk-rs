use std::{fs::File, io::Write};

use crate::cmd::contract::generate_snippets::snippet_gen_common::write_newline;

const PREFIX_AUTO_GENERATED: &str = "////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////
";

const IMPORTS: &str = "#![allow(clippy::all)]

use multiversx_sc::imports::*;";

pub(crate) fn write_header(file: &mut File) {
    writeln!(file, "{PREFIX_AUTO_GENERATED}").unwrap();
    writeln!(file, r#"{IMPORTS}"#).unwrap();

    write_newline(file);
}

pub(crate) fn write_struct_template(file: &mut File) {
    writeln!(file, "pub struct TxProxy;").unwrap();
    write_newline(file)
}

pub(crate) fn write_impl_for_tx_proxy(file: &mut File) {
    writeln!(
        file,
        r#"impl<Env, From, To, Gas> TxProxyTraitV2<Env, From, To, Gas> for TxProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    type TxProxyMethods = TxProxyMethods<Env, From, To, Gas>;

    fn prepare_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {{
        TxProxyMethods {{ wrapped_tx: tx }}
    }}
}}"#
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_struct_tx_proxy_methods(file: &mut File) {
    writeln!(
        file,
        r#"pub struct TxProxyMethods<Env, From, To, Gas>
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

    write_newline(file);
}
