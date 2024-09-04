mod basic_interact_state;

use basic_interact_state::State;
use ping_pong_egld::ping_pong_proxy;

use multiversx_sc_snippets::imports::*;

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
        let wallet_address = interactor.register_wallet(test_wallets::carol());

        Self {
            interactor,
            adder_owner_address: ping_pong_owner.into(),
            wallet_address: wallet_address.into(),
            state: State::load_state(),
        }
    }

    async fn set_state(&mut self) {
        println!("wallet address: {}", self.wallet_address);
        self.interactor
            .retrieve_account(&self.adder_owner_address)
            .await;
        self.interactor.retrieve_account(&self.wallet_address).await;
    }

    pub async fn deploy(&mut self) {
        let ping_amount = BigUint::<StaticApi>::from(10000000000000000u128);
        let duration_in_seconds = 1825457087u64;
        let opt_activation_timestamp = Option::Some(0u64);
        let max_funds = OptionalValue::Some(BigUint::<StaticApi>::from(100000000000000000000u128));

        let new_address = self
            .interactor
            .tx()
            .from(&self.adder_owner_address)
            .gas(30_000_000)
            .typed(ping_pong_proxy::PingPongProxy)
            .init(
                ping_amount,
                duration_in_seconds,
                opt_activation_timestamp,
                max_funds,
            )
            .code(PINGPONG_CODE_PATH)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewBech32Address)
            .prepare_async()
            .run()
            .await;

        println!("new address: {new_address}");
        self.state.set_contract_address(new_address);
    }

    pub async fn deadline(&mut self) {
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

        println!("Result: {result_value:?}");
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

        println!("Result: {result_value:?}");
        result_value.to_string()
    }

    pub async fn max_funds(&mut self) {
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

        println!("Result: {result_value:?}");
    }

    pub async fn ping(&mut self, value: u128) -> String {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_contract_address())
            .gas(30_000_000)
            .typed(ping_pong_proxy::PingPongProxy)
            .ping(IgnoreValue)
            .egld(BigUint::from(value))
            .prepare_async()
            .run()
            .await;

        "Success".to_string()
    }
}

#[tokio::test]
async fn test_deploy() {
    let mut interactor = ActixInteractor::init().await;
    interactor.deploy().await;
}

#[tokio::test]
async fn test_ping() {
    let mut interactor = ActixInteractor::init().await;
    interactor.ping(1).await;
}
