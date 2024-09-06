mod basic_interact_state;

use basic_interact_state::State;
use ping_pong_egld::ping_pong_proxy;

use multiversx_sc_snippets::{hex, imports::*};

const INTERACTOR_SCENARIO_TRACE_PATH: &str = "interactor_trace.scen.json"; // wrong - doesn't exist

const PINGPONG_CODE_PATH: MxscPath = MxscPath::new("../../../output/ping-pong-egld.mxsc.json");
#[allow(unused)]
pub struct ActixInteractor {
    interactor: Interactor,
    adder_owner_address: Bech32Address,
    wallet_address: Bech32Address,
    state: State,
}

impl ActixInteractor {
    pub async fn init() -> Self {
        // let config = Config::load_config();
        let mut interactor = Interactor::new("https://devnet-gateway.multiversx.com")
            .await
            .with_tracer(INTERACTOR_SCENARIO_TRACE_PATH)
            .await;

        let ping_pong_owner = interactor.register_wallet(Wallet::from(test_wallets::alice()));
        let wallet_address = interactor.register_wallet(test_wallets::heidi());

        let mut state = State::load_state();
        state.set_contract_address(Bech32Address::from_bech32_string("erd1qqqqqqqqqqqqqpgqmg79q8dvh4x8equxrtccf5jqw2ruc2d3d8ssww2c7t".to_string()));

        Self {
            interactor,
            adder_owner_address: ping_pong_owner.into(),
            wallet_address: wallet_address.into(),
            state,
        }
    }

    async fn _set_state(&mut self) {
        println!("wallet address: {}", self.wallet_address);
        self.interactor
            .retrieve_account(&self.adder_owner_address)
            .await;
        self.interactor.retrieve_account(&self.wallet_address).await;
    }

    pub async fn deploy(
        &mut self,
        ping_amount: u128,
        duration_in_seconds: u64,
        opt_activation_timestamp: Option<u64>,
        max_funds: u128,
        deployer: String,
    ) -> String {
        let ping_amount = BigUint::<StaticApi>::from(ping_amount);
        // let duration_in_seconds = 1825457087u64;
        // let opt_activation_timestamp = Option::Someping_amount0u64);
        let max_funds_option = OptionalValue::Some(BigUint::<StaticApi>::from(max_funds));

        let new_address = self
            .interactor
            .tx()
            .from(Bech32Address::from_bech32_string(deployer))
            .gas(30_000_000)
            .typed(ping_pong_proxy::PingPongProxy)
            .init(
                ping_amount,
                duration_in_seconds,
                opt_activation_timestamp,
                max_funds_option,
            )
            .code(PINGPONG_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;

        let str_addr = new_address.to_string();

        println!("new address: {new_address}");
        self.state.set_contract_address(new_address);

        str_addr
    }

    pub async fn deadline(&mut self) -> String {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(ping_pong_proxy::PingPongProxy)
            .deadline()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        result_value.to_string()
    }

    pub async fn activation_timestamp(&mut self) -> String {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(ping_pong_proxy::PingPongProxy)
            .activation_timestamp()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        result_value.to_string()
    }

    pub async fn max_funds(&mut self) -> String {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(ping_pong_proxy::PingPongProxy)
            .max_funds()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        result_value.unwrap().to_string()
    }

    pub async fn ping_amount(&mut self) -> String {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(ping_pong_proxy::PingPongProxy)
            .ping_amount()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        result_value.to_string()
    }

    pub async fn user_addresses(&mut self) -> String {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_contract_address())
            .typed(ping_pong_proxy::PingPongProxy)
            .get_user_addresses()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        // ??? HEADCANON
        let addresses_str: Vec<String> = result_value
            .iter()
            .map(|address| hex::encode(address.as_bytes()))
            .collect();

        let result = addresses_str.join(", ");

        result
    }

    pub async fn ping(&mut self, sender: String, contract_address: String, value: u128) -> String {
        self.interactor
            .tx()
            .from(Bech32Address::from_bech32_string(sender))
            .to(Bech32Address::from_bech32_string(contract_address))
            .gas(30_000_000)
            .typed(ping_pong_proxy::PingPongProxy)
            .ping(IgnoreValue)
            .egld(BigUint::from(value))
            .prepare_async()
            .run()
            .await;

        "Tx successful".to_string()
    }

    pub async fn pong(&mut self, sender: String, contract_address: String) -> String {
        self.interactor
            .tx()
            .from(Bech32Address::from_bech32_string(sender))
            .to(Bech32Address::from_bech32_string(contract_address))
            .gas(30_000_000)
            .typed(ping_pong_proxy::PingPongProxy)
            .pong()
            .prepare_async()
            .run()
            .await;

        "Tx successful".to_string()
    }
}

#[tokio::test]
async fn test() {
    let mut interactor = ActixInteractor::init().await;

    // interactor
    //     .deploy(
    //         1000000000000000u128,
    //         60,
    //         None,
    //         1000000000000000000000000000000u128,
    //         "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th".to_string(),
    //     )
    //     .await;

    // interactor
    //     .pong(
    //         "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th".to_string(),
    //         "erd1qqqqqqqqqqqqqpgqmdsgk535ujtcjnhcs7fvzuemksqygchwd8ss0za8wa".to_string(),
    //     )
    //     .await;
}

#[tokio::test]
async fn test_deadline() {
    let mut interactor = ActixInteractor::init().await;
    let res = interactor.deadline().await;
    println!("Deadline: {}", res);
}

#[tokio::test]
async fn test_user_addresses() {
    let mut interactor = ActixInteractor::init().await;
    let res = interactor.user_addresses().await;
    println!("User Addresses: {}", res);
}

#[tokio::test]
async fn test_max_funds() {
    let mut interactor = ActixInteractor::init().await;
    let res = interactor.max_funds().await;
    println!("Max Funds: {}", res);
}

