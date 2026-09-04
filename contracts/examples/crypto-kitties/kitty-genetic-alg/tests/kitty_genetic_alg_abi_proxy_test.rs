use kitty::{Kitty, KittyGenes};
use multiversx_sc_scenario::imports::*;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const KITTY_GENETIC_ALG_ADDRESS: TestSCAddress = TestSCAddress::new("kitty-genetic-alg");
const CODE_PATH: MxscPath = MxscPath::new("output/kitty-genetic-alg.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain
        .set_current_dir_from_workspace("contracts/examples/crypto-kitties/kitty-genetic-alg");
    blockchain.register_contract(CODE_PATH, kitty_genetic_alg::ContractBuilder);
    blockchain
}

/// Proves the auto-generated `KittyGeneticAlgAbiProxy` (produced by `#[contract_abi]`, with no
/// dependency on `multiversx-sc`/`VMApi`) interoperates with the real framework `Tx` builder via
/// `Tx::abi_typed`, exactly like the hand-written `AdderAbiProxy` does in `adder_blackbox_test.rs`.
#[test]
fn kitty_genetic_alg_abi_proxy_blackbox() {
    let mut world = world();
    world.account(OWNER_ADDRESS).nonce(1);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .abi_typed(kitty::kitty_genetic_alg_abi::KittyGeneticAlgAbiProxy)
        .init()
        .code(CODE_PATH)
        .new_address(KITTY_GENETIC_ALG_ADDRESS)
        .run();

    let matron = Kitty::new(KittyGenes::default(), TimestampMillis::zero(), 0, 0, 1);
    let sire = Kitty::new(KittyGenes::default(), TimestampMillis::zero(), 0, 0, 1);

    world
        .query()
        .to(KITTY_GENETIC_ALG_ADDRESS)
        .abi_typed(kitty::kitty_genetic_alg_abi::KittyGeneticAlgAbiProxy)
        .generate_kitty_genes(matron, sire)
        .returns(ReturnsResult)
        .run();
}
