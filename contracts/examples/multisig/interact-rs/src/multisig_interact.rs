use elrond_interaction::{
    elrond_wasm::{
        elrond_codec::multi_types::MultiValueVec, storage::mappers::SingleValue, types::Address,
    },
    elrond_wasm_debug::{mandos_system::model::*, ContractInfo, DebugApi},
    env_logger,
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
// const MULTISIG_ADDRESS_BECH32: &str =
//     "bech32:erd1qqqqqqqqqqqqqpgq27w853kf76sehkzqtkpl96k7ejx4nf7gkrusmvzsy9";

type MultisigContract = ContractInfo<multisig::Proxy<DebugApi>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = DebugApi::dummy();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at lest one argument required");
    let mut state = State::init(args).await;
    match cmd.as_str() {
        "send" => state.send().await,
        "quorum" => state.quorum().await,
        "board" => state.board().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    multisig: MultisigContract,
    #[allow(dead_code)]
    args: Args,
}

impl State {
    async fn init(args: Args) -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let multisig = MultisigContract::new(MULTISIG_ADDRESS_BECH32);
        State {
            interactor,
            wallet_address,
            multisig,
            args,
        }
    }

    async fn send(&mut self) {
        let action_id: usize = self
            .interactor
            .sc_call(
                self.multisig
                    .propose_change_quorum(5usize)
                    .into_blockchain_call()
                    .from(&self.wallet_address)
                    .gas_limit("5,000,000")
                    .expect(TxExpect::ok()),
            )
            .await;
        println!("action id: {}", action_id);
    }

    async fn quorum(&mut self) {
        let quorum: SingleValue<usize> = self.interactor.vm_query(self.multisig.quorum()).await;

        println!("quorum: {}", quorum.into());
    }

    async fn board(&mut self) {
        let board_members: MultiValueVec<Address> = self
            .interactor
            .vm_query(self.multisig.get_all_board_members())
            .await;

        println!("board members:");
        for board_member in board_members.iter() {
            println!("    {}", board_member.to_bech32());
        }
    }
}
