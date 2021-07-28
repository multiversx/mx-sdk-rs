elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum UserStatus {
    New,
    Registered,
    Withdrawn,
}
