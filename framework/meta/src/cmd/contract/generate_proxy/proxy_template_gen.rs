use std::{fs::File, io::Write};

use crate::cmd::contract::generate_snippets::snippet_gen_common::write_newline;

pub(crate) fn write_imports(file: &mut File) {
    writeln!(file, r#"multiversx_sc::imports!();"#).unwrap();

    write_newline(file);
}

pub(crate) fn write_struct_template(file: &mut File) {
    writeln!(file, "pub struct TxProxy;").unwrap();
    write_newline(file)
}

pub(crate) fn write_impl_for_tx_proxy(file: &mut File) {
    writeln!(
        file,
        r#"impl<Env> TxProxyTrait<Env> for TxProxy
where
    Env: TxEnv,
{{
    type TxProxyMethods = TxProxyMethods<Env>;

    fn env(self, env: Env) -> Self::TxProxyMethods {{
        TxProxyMethods {{ env }}
    }}
}}"#
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_tx_proxy_method_header(file: &mut File) {
    writeln!(
        file,
        r#"impl<Env: TxEnv + multiversx_sc::api::CallTypeApi> TxProxyMethods<Env> {{"#
    )
    .unwrap();
}
