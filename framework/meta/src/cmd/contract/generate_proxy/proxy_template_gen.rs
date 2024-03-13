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

pub(crate) fn write_struct_template(file: &mut File, name: &String) {
    writeln!(file, "pub struct Tx{name};").unwrap();
    write_newline(file)
}

pub(crate) fn write_impl_for_tx_proxy(file: &mut File, name: &String) {
    writeln!(
        file,
        r#"impl<Env, From, To, Gas> TxProxyTraitV2<Env, From, To, Gas> for Tx{name}
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    type TxProxyMethods = Tx{name}Methods<Env, From, To, Gas>;

    fn prepare_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {{
        Tx{name}Methods {{ wrapped_tx: tx }}
    }}
}}"#
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_struct_tx_proxy_methods(file: &mut File, name: &String) {
    writeln!(
        file,
        r#"pub struct Tx{name}Methods<Env, From, To, Gas>
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
