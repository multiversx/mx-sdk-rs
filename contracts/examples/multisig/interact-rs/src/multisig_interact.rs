use elrond_interaction::{
    elrond_wasm::{
        elrond_codec::multi_types::MultiValueVec, storage::mappers::SingleValue, types::Address,
    },
    elrond_wasm_debug::{
        mandos_system::model::{ScCallStep, TxExpect},
        ContractInfo, DebugApi,
    },
    erdrs::interactors::wallet::Wallet,
    tokio, Interactor,
};
use multisig::{
    multisig_propose::ProxyTrait as _, multisig_state::ProxyTrait as _, ProxyTrait as _,
};
use std::env::Args;

const GATEWAY: &str = elrond_interaction::erdrs::blockchain::rpc::TESTNET_GATEWAY;
const PEM: &str = "alice.pem";
const MULTISIG_ADDRESS_BECH32: &str =
    "bech32:erd1qqqqqqqqqqqqqpgq09aksdufwfs07e0vdypgev8dvsemwg4td8sssx9shk";

type MultisigContract = ContractInfo<multisig::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    let _ = DebugApi::dummy();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at lest one argument required");
    match cmd.as_str() {
        "send" => send(args).await,
        "quorum" => quorum(args).await,
        "board" => board(args).await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    multisig: MultisigContract,
}

impl State {
    async fn init() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let multisig = MultisigContract::new(MULTISIG_ADDRESS_BECH32);
        State {
            interactor,
            wallet_address,
            multisig,
        }
    }
}

async fn send(_args: Args) {
    let mut state = State::init().await;

    state
        .interactor
        .mandos_sc_call(
            ScCallStep::new()
                .from(&state.wallet_address)
                .call(state.multisig.propose_change_quorum(5usize))
                .gas_limit("5,000,000")
                .expect(TxExpect::ok()),
        )
        .await;
}

async fn quorum(_args: Args) {
    let mut state = State::init().await;

    let quorum: SingleValue<usize> = state.interactor.vm_query(state.multisig.quorum()).await;

    println!("quorum: {}", quorum.into());
}

async fn board(_args: Args) {
    let mut state = State::init().await;

    let board_members: MultiValueVec<Address> = state
        .interactor
        .vm_query(state.multisig.get_all_board_members())
        .await;

    println!("board members:");
    for board_member in board_members.iter() {
        println!("    {}", board_member.to_bech32());
    }
}
