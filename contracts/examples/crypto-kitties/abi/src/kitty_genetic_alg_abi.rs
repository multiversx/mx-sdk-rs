#![allow(dead_code)]

use crate::{Kitty, KittyGenes};

/// Framework-agnostic ABI description of the `kitty-genetic-alg` contract's interface. Mirrors
/// `kitty_genetic_alg_proxy.rs`, but written directly against pure ABI types, with no dependency
/// on `multiversx-sc`/`VMApi`.
#[multiversx_sc_abi::contract_abi(call = KittyGeneticAlgCall)]
pub trait KittyGeneticAlg {
    #[init]
    fn init(&self);

    #[endpoint(generateKittyGenes)]
    fn generate_kitty_genes(&self, matron: Kitty, sire: Kitty) -> KittyGenes;
}
