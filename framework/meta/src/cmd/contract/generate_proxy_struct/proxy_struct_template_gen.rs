use std::{fs::File, io::Write};

use crate::cmd::contract::generate_snippets::snippet_gen_common::write_newline;

pub(crate) fn write_imports(file: &mut File) {
    writeln!(
        file,
        r#"#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]"#
    )
        .unwrap();

    write_newline(file);
}

pub(crate) fn write_struct_template(file: &mut File) {
    write!(
        file,
        "pub struct Proxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{{
    pub address: multiversx_sc::types::ManagedOption<
        A,
        multiversx_sc::types::ManagedAddress<A>,
    >,
}}

impl<A> Proxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{{
    multiversx_sc_wasm_adapter::endpoints_proxy! {{
"
    )
        .unwrap();
}
