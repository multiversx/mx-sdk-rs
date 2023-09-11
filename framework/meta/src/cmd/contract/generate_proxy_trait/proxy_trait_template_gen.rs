use std::{fs::File, io::Write};

pub(crate) fn write_proxy_imports(file: &mut File, abi_name: &str) {
    write!(
        file,
        "multiversx_sc::imports!();

#[multiversx_sc::proxy]
pub trait {abi_name} {{
"
    )
        .unwrap();
}