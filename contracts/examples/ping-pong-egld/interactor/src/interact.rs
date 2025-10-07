mod interact_cli;
mod interact_config;
mod interact_state;

use crate::interact_state::State;
use clap::Parser;
pub use interact_config::Config;
use ping_pong_egld::proxy_ping_pong_egld::{self, ContractState, UserStatus};

use multiversx_sc_snippets::imports::*;

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json";

const PING_PONG_CODE: MxscPath = MxscPath::new("../output/ping-pong-egld.mxsc.json");

pub async fn ping_pong_egld_cli() {
    env_logger::init();

    let config = Config::load_config();

    let mut interact = PingPongEgldInteract::init(config).await;

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
            let sender = interact.ping_pong_owner_address.clone();
            interact
                .ping(args.cost.unwrap_or_default(), None, &sender)
                .await
        }
        Some(interact_cli::InteractCliCommand::Pong) => {
            let sender = interact.ping_pong_owner_address.clone();
            interact.pong(None, &sender).await;
        }
        Some(interact_cli::InteractCliCommand::PongAll) => {
            let sender = interact.ping_pong_owner_address.clone();
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
            println!("Contract state: ping_amount -> {:#?} | deadline -> {:#?} | activation_timestamp -> {:#?} | max_funds -> {:#?} | pong_all_last_user -> {:#?}", 
            contract_state.ping_amount,
            contract_state.deadline,
            contract_state.activation_timestamp,
            contract_state.max_funds,
            contract_state.pong_all_last_user);
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
    pub ping_pong_owner_address: Bech32Address,
    pub wallet_address: Bech32Address,
    pub state: State,
}

impl PingPongEgldInteract {
    pub async fn init(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator())
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;

        interactor.set_current_dir_from_workspace("contracts/examples/ping-pong-egld/interactor");
        let ping_pong_owner_address = interactor.register_wallet(test_wallets::eve()).await;
        let wallet_address = interactor.register_wallet(test_wallets::mallory()).await;

        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_all_activations().await;

        Self {
            interactor,
            ping_pong_owner_address: ping_pong_owner_address.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    pub async fn set_state(&mut self) {
        println!("wallet address: {}", self.wallet_address);
        self.interactor
            .retrieve_account(&self.ping_pong_owner_address)
            .await;
        self.interactor.retrieve_account(&self.wallet_address).await;
    }

    pub async fn deploy(
        &mut self,
        ping_amount: RustBigUint,
        duration: DurationMillis,
        opt_activation_timestamp: Option<TimestampMillis>,
        max_funds: OptionalValue<RustBigUint>,
    ) -> (u64, String) {
        self.set_state().await;

        let (new_address, status, message) = self
            .interactor
            .tx()
            .from(&self.ping_pong_owner_address)
            .gas(30_000_000u64)
            .typed(proxy_ping_pong_egld::PingPongProxy)
            .init(ping_amount, duration, opt_activation_timestamp, max_funds)
            .code(PING_PONG_CODE)
            .returns(ReturnsNewBech32Address)
            .returns(ReturnsStatus)
            .returns(ReturnsMessage)
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_ping_pong_egld_address(new_address);

        (status, message)
    }

    pub async fn upgrade(
        &mut self,
        ping_amount: RustBigUint,
        duration: DurationMillis,
        opt_activation_timestamp: Option<TimestampMillis>,
        max_funds: OptionalValue<RustBigUint>,
    ) {
        let response = self
            .interactor
            .tx()
            .to(self.state.current_ping_pong_egld_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy_ping_pong_egld::PingPongProxy)
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
            .typed(proxy_ping_pong_egld::PingPongProxy)
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
            .typed(proxy_ping_pong_egld::PingPongProxy)
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
            .typed(proxy_ping_pong_egld::PingPongProxy)
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
            .typed(proxy_ping_pong_egld::PingPongProxy)
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
            .typed(proxy_ping_pong_egld::PingPongProxy)
            .get_contract_state()
            .returns(ReturnsResult)
            .run()
            .await
    }

    pub async fn get_ping_amount(&mut self) -> RustBigUint {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy_ping_pong_egld::PingPongProxy)
            .ping_amount()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_deadline(&mut self) -> TimestampMillis {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy_ping_pong_egld::PingPongProxy)
            .deadline()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_activation_timestamp(&mut self) -> TimestampMillis {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy_ping_pong_egld::PingPongProxy)
            .activation_timestamp()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_max_funds(&mut self) -> Option<RustBigUint> {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy_ping_pong_egld::PingPongProxy)
            .max_funds()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_user_status(&mut self, user_id: usize) -> UserStatus {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy_ping_pong_egld::PingPongProxy)
            .user_status(user_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn pong_all_last_user(&mut self) -> usize {
        self.interactor
            .query()
            .to(self.state.current_ping_pong_egld_address())
            .typed(proxy_ping_pong_egld::PingPongProxy)
            .pong_all_last_user()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }
}
