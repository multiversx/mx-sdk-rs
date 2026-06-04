mod interact_cli;

use clap::Parser;
use multiversx_sc_snippets::imports::*;
use ping_pong_egld::proxy::{self, ContractState, UserStatus};
use serde::{Deserialize, Serialize};

/// Ping Pong Interact configuration
#[derive(Debug, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub owner: WalletConfig,
    pub wallet: WalletConfig,
}

impl InteractorConfig for Config {
    fn connection(&self) -> &ConnectionConfig {
        &self.connection
    }

    fn register_wallets(&self) -> Vec<Wallet> {
        vec![self.owner.wallet().clone(), self.wallet.wallet().clone()]
    }
}

/// Ping Pong Interact state
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub ping_pong_egld_address: Option<Bech32Address>,
}

impl State {
    /// Returns the ping pong contract
    pub fn current_ping_pong_egld_address(&self) -> &Bech32Address {
        self.ping_pong_egld_address
            .as_ref()
            .expect("no known ping pong contract, deploy first")
    }
}

pub const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

const PING_PONG_CODE: MxscPath = MxscPath::new("../output/ping-pong-egld.mxsc.json");

pub async fn ping_pong_egld_cli() {
    env_logger::init();

    let mut interact = PingPongEgldInteract::new().await;

    let cli = interact_cli::InteractCli::parse();
    match &cli.command {
        Some(interact_cli::InteractCliCommand::Deploy(args)) => {
            interact
                .deploy(
                    args.ping_amount.clone(),
                    DurationMillis::new(args.duration),
                    args.opt_activation_timestamp.map(TimestampMillis::new),
                    OptionalValue::from(args.max_funds.clone()),
                )
                .await;
        }
        Some(interact_cli::InteractCliCommand::Upgrade(args)) => {
            interact
                .upgrade(
                    args.ping_amount.clone(),
                    DurationMillis::new(args.duration),
                    args.opt_activation_timestamp.map(TimestampMillis::new),
                    OptionalValue::from(args.max_funds.clone()),
                )
                .await
        }
        Some(interact_cli::InteractCliCommand::Ping(args)) => {
            let sender = interact.config.owner.address();
            interact
                .ping(args.cost.unwrap_or_default(), None, &sender)
                .await
        }
        Some(interact_cli::InteractCliCommand::Pong) => {
            let sender = interact.config.owner.address();
            interact.pong(None, &sender).await;
        }
        Some(interact_cli::InteractCliCommand::PongAll) => {
            let sender = interact.config.owner.address();
            interact.pong_all(None, &sender).await;
        }
        Some(interact_cli::InteractCliCommand::GetUserAddresses) => {
            let user_addresses = interact.get_user_addresses().await;
            println!("User addresses: ");
            for address in user_addresses {
                print!("{address} ");
            }
        }
        Some(interact_cli::InteractCliCommand::GetContractState) => {
            let contract_state = interact.get_contract_state().await;
            println!(
                "Contract state: ping_amount -> {:#?} | deadline -> {:#?} | activation_timestamp -> {:#?} | max_funds -> {:#?} | pong_all_last_user -> {:#?}",
                contract_state.ping_amount,
                contract_state.deadline,
                contract_state.activation_timestamp,
                contract_state.max_funds,
                contract_state.pong_all_last_user
            );
        }
        Some(interact_cli::InteractCliCommand::GetPingAmount) => {
            let ping_amount = interact.get_ping_amount().await;
            println!("Ping amount: {}", ping_amount);
        }
        Some(interact_cli::InteractCliCommand::GetDeadline) => {
            let deadline = interact.get_deadline().await;
            println!("Deadline: {}", deadline);
        }
        Some(interact_cli::InteractCliCommand::GetActivationTimestamp) => {
            let activation_timestamp = interact.get_activation_timestamp().await;
            println!("Activation timestamp: {}", activation_timestamp);
        }
        Some(interact_cli::InteractCliCommand::GetMaxFunds) => {
            let max_funds = interact.get_max_funds().await;
            match max_funds {
                Some(funds) => println!("Max funds: {}", funds),
                None => println!("Max funds: none"),
            }
        }
        Some(interact_cli::InteractCliCommand::GetUserStatus(args)) => {
            let user_status = interact.get_user_status(args.id).await;
            match user_status {
                UserStatus::New => println!("User status: unknown"),
                UserStatus::Registered => println!("User status: `ping`-ed"),
                UserStatus::Withdrawn => println!("User status: `pong`-ed"),
            }
        }
        Some(interact_cli::InteractCliCommand::PongAllLastUser) => {
            let pong_all_last_user = interact.pong_all_last_user().await;
            println!("Pong all last user: {pong_all_last_user}");
        }
        None => {}
    }
}

pub struct PingPongEgldInteract {
    pub interactor: Interactor,
    pub config: Config,
    pub state: AutoSave<State>,
}

impl PingPongEgldInteract {
    pub async fn new() -> Self {
        let (interactor, config) = HttpInteractorBuilder::new()
            .crate_dir(env!("CARGO_MANIFEST_DIR"))
            .build()
            .await;
        let interactor = interactor.with_tracer(INTERACTOR_SCENARIO_TRACE_PATH).await;
        let state = interactor.load_state::<State>();
        Self {
            interactor,
            config,
            state,
        }
    }

    pub async fn set_state(&mut self) {
        let owner_address = self.config.owner.address();
        let wallet_address = self.config.wallet.address();
        println!("wallet address: {}", wallet_address);
        self.interactor.retrieve_account(&owner_address).await;
        self.interactor.retrieve_account(&wallet_address).await;
    }

    pub async fn deploy(
        &mut self,
        ping_amount: RustBigUint,
        duration: DurationMillis,
        opt_activation_timestamp: Option<TimestampMillis>,
        max_funds: OptionalValue<RustBigUint>,
    ) -> (u64, String) {
        self.set_state().await;

        let owner_address = self.config.owner.address();
        let (new_address, status, message) = self
            .interactor
            .tx()
            .from(&owner_address)
            .gas(30_000_000u64)
            .typed(proxy::PingPongEgldProxy)
            .init(ping_amount, duration, opt_activation_timestamp, max_funds)
            .code(PING_PONG_CODE)
            .returns(ReturnsNewBech32Address)
            .returns(ReturnsStatus)
            .returns(ReturnsMessage)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.ping_pong_egld_address = Some(new_address);

        (status, message)
    }

    pub async fn upgrade(
        &mut self,
        ping_amount: RustBigUint,
        duration: DurationMillis,
        opt_activation_timestamp: Option<TimestampMillis>,
        max_funds: OptionalValue<RustBigUint>,
    ) {
        let wallet_address = self.config.wallet.address();
        let response = self
            .interactor
            .tx()
            .to(self.state.current_ping_pong_egld_address())
            .from(&wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::PingPongEgldProxy)
            .upgrade(ping_amount, duration, opt_activation_timestamp, max_funds)
            .code(PING_PONG_CODE)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ping(&mut self, egld_amount: u64, message: Option<&str>, sender: &Bech32Address) {
        let _data: IgnoreValue = IgnoreValue;

        let response = self
            .interactor
            .tx()
            .from(sender)
            .to(self.state.current_ping_pong_egld_address())
            .gas(30_000_000u64)
            .typed(proxy::PingPongEgldProxy)
            .ping(_data)
            .egld(egld_amount)
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("Ping successful!"),
            Err(err) => {
                println!("Ping failed with message: {}", err.message);
                assert_eq!(message.unwrap_or_default(), err.message);
            }
        }
    }

    pub async fn pong(&mut self, message: Option<&str>, sender: &Bech32Address) {
        let response = self
            .interactor
            .tx()
            .from(sender)
            .to(self.state.current_ping_pong_egld_address())
            .gas(30_000_000u64)
            .typed(proxy::PingPongEgldProxy)
            .pong()
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("Pong successful!"),
            Err(err) => {
                println!("Pong failed with message: {}", err.message);
                assert_eq!(message.unwrap_or_default(), err.message);
            }
        }
    }

    pub async fn pong_all(&mut self, message: Option<String>, sender: &Bech32Address) {
        let response = self
            .interactor
            .tx()
            .from(sender)
            .to(self.state.current_ping_pong_egld_address())
            .gas(30_000_000u64)
            .typed(proxy::PingPongEgldProxy)
            .pong_all()
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        match response {
            Ok(_) => println!("Pong All successful!"),
            Err(err) => {
                println!("Pong All failed with message: {}", err.message);
                assert_eq!(message.unwrap_or_default(), err.message);
            }
        }
    }

    pub async fn get_user_addresses(&mut self) -> Vec<String> {
        let response = self
            .interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy::PingPongEgldProxy)
            .get_user_addresses()
            .returns(ReturnsResult)
            .run()
            .await;

        let mut response_vec: Vec<String> = Vec::new();
        for r in response.to_vec().into_vec() {
            response_vec.push(r.as_managed_buffer().to_string());
        }

        response_vec
    }

    pub async fn get_contract_state(&mut self) -> ContractState<StaticApi> {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy::PingPongEgldProxy)
            .get_contract_state()
            .returns(ReturnsResult)
            .run()
            .await
    }

    pub async fn get_ping_amount(&mut self) -> RustBigUint {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy::PingPongEgldProxy)
            .ping_amount()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_deadline(&mut self) -> TimestampMillis {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy::PingPongEgldProxy)
            .deadline()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_activation_timestamp(&mut self) -> TimestampMillis {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy::PingPongEgldProxy)
            .activation_timestamp()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_max_funds(&mut self) -> Option<RustBigUint> {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy::PingPongEgldProxy)
            .max_funds()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_user_status(&mut self, user_id: usize) -> UserStatus {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy::PingPongEgldProxy)
            .user_status(user_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn pong_all_last_user(&mut self) -> usize {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy::PingPongEgldProxy)
            .pong_all_last_user()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }
}
