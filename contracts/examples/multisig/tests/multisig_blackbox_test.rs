#![allow(unused)]

use adder::*;
use multisig::{
    multisig_perform::ProxyTrait as _, multisig_propose::ProxyTrait as _, ProxyTrait as _,
};

use multiversx_sc::{
    codec::multi_types::{MultiValueVec, OptionalValue},
    storage::mappers::SingleValue,
    types::{Address, CodeMetadata},
};
use multiversx_sc_scenario::{
    api::StaticApi,
    scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
    scenario_model::*,
    ContractInfo, ScenarioWorld,
};
use num_bigint::BigUint;

const MULTISIG_PATH_EXPR: &str = "file:output/multisig.wasm";
const ADDER_PATH_EXPR: &str = "file:test-contracts/adder.wasm";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_contract(ADDER_PATH_EXPR, adder::ContractBuilder);
    blockchain.register_contract(MULTISIG_PATH_EXPR, multisig::ContractBuilder);
    blockchain
}

#[test]
fn basic_setup_test() {
    let mut test = MultisigTestState::setup();
    test.multisig_deploy();

    // TODO: transform into a framework test and replace here with sc_call_get_result
    let mut board_members = MultiValueVec::<Address>::new();
    test.world.sc_call_use_result(
        ScCallStep::new()
            .from(&test.alice)
            .call(test.multisig.get_all_board_members()),
        |tr| {
            board_members = tr.result.unwrap();
        },
    );

    let expected_board_members: Vec<_> = [
        test.alice.to_address(),
        test.bob.to_address(),
        test.carol.to_address(),
    ]
    .into();

    assert_eq!(board_members.into_vec(), expected_board_members);
}

#[test]
fn multisig_adder_test() {
    let mut test = MultisigTestState::setup();
    test.adder_deploy().multisig_deploy();

    let caller = &test.alice.to_address();
    let signers = [
        &test.alice.to_address(),
        &test.bob.to_address(),
        &test.carol.to_address(),
    ];
    let deployed_sc_address = test.multisig_deploy_adder(caller, signers.as_slice());
    assert_eq!(deployed_sc_address, test.adder_multisig.to_address());

    let first_value: BigUint = 42u64.into();
    let second_value: BigUint = 43u64.into();
    let expected_sum = first_value.clone() + second_value.clone();

    test.multisig_call_adder_add(first_value, caller, &signers);
    test.multisig_call_adder_add(second_value, caller, &signers);
    test.adder_expect_get_sum(expected_sum, caller);
}

type MultisigContract = ContractInfo<multisig::Proxy<StaticApi>>;
type AdderContract = ContractInfo<adder::Proxy<StaticApi>>;

struct MultisigTestState {
    world: ScenarioWorld,
    owner: AddressValue,
    alice: AddressValue,
    bob: AddressValue,
    carol: AddressValue,
    multisig: MultisigContract,
    adder: AdderContract,
    adder_multisig: AdderContract,
}

impl MultisigTestState {
    fn setup() -> Self {
        let world = world();
        let ic = &world.interpreter_context();

        let mut state = MultisigTestState {
            world,
            owner: "address:owner".into(),
            alice: "address:alice".into(),
            bob: "address:bob".into(),
            carol: "address:carol".into(),
            multisig: MultisigContract::new("sc:multisig"),
            adder: AdderContract::new("sc:adder"),
            adder_multisig: AdderContract::new("sc:adder-multisig"),
        };

        state.world.set_state_step(
            SetStateStep::new()
                .put_account(&state.owner, Account::new().nonce(1))
                .put_account(&state.alice, Account::new().nonce(1))
                .put_account(&state.bob, Account::new().nonce(1))
                .put_account(&state.carol, Account::new().nonce(1)),
        );

        state
    }

    fn multisig_deploy(&mut self) -> &mut Self {
        self.world.set_state_step(
            SetStateStep::new()
                .put_account(&self.owner, Account::new().nonce(1))
                .new_address(&self.owner, 1, &self.multisig),
        );

        let board: MultiValueVec<Address> = vec![
            self.alice.value.clone(),
            self.bob.value.clone(),
            self.carol.value.clone(),
        ]
        .into();

        let ic = &self.world.interpreter_context();
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(self.owner.clone())
                .code(self.world.code_expression(MULTISIG_PATH_EXPR))
                .call(self.multisig.init(2u32, board)),
        );

        self
    }

    fn adder_deploy(&mut self) -> &mut Self {
        let ic = &self.world.interpreter_context();
        self.world.set_state_step(
            SetStateStep::new()
                .put_account(&self.owner, Account::new().nonce(1))
                .new_address(&self.owner, 1, &self.adder),
        );
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(self.owner.clone())
                .code(self.world.code_expression(ADDER_PATH_EXPR))
                .call(self.adder.init(0u64)),
        );

        self
    }

    fn multisig_sign(&mut self, action_id: usize, signer: &Address) {
        self.world.sc_call(
            ScCallStep::new()
                .from(signer)
                .call(self.multisig.sign(action_id)),
        );
    }

    fn multisig_sign_multiple(&mut self, action_id: usize, signers: &[&Address]) {
        for &signer in signers {
            self.multisig_sign(action_id, signer)
        }
    }

    fn multisig_perform(&mut self, action_id: usize, caller: &Address) -> Option<Address> {
        let output: OptionalValue<Address> = self.world.sc_call_get_result(
            ScCallStep::new()
                .from(caller)
                .call(self.multisig.perform_action_endpoint(action_id)),
        );
        output.into_option()
    }

    fn multisig_sign_and_perform(
        &mut self,
        action_id: usize,
        caller: &Address,
        signers: &[&Address],
    ) -> Option<Address> {
        self.multisig_sign_multiple(action_id, signers);
        self.multisig_perform(action_id, caller)
    }

    fn multisig_deploy_adder(&mut self, caller: &Address, signers: &[&Address]) -> Address {
        let action_id = self.multisig_propose_adder_deploy(caller);
        self.multisig_sign_and_perform(action_id, caller, signers)
            .unwrap()
    }

    fn multisig_propose_adder_deploy(&mut self, caller: &Address) -> usize {
        self.world.set_state_step(SetStateStep::new().new_address(
            &self.multisig.scenario_address_expr,
            0,
            &self.adder_multisig,
        ));

        let adder_init_args = self.adder.init(0u64).arg_buffer.into_multi_value_encoded();
        self.world
            .sc_call_get_result(ScCallStep::new().from(caller).call(
                self.multisig.propose_sc_deploy_from_source(
                    0u64,
                    &self.adder,
                    CodeMetadata::DEFAULT,
                    adder_init_args,
                ),
            ))
    }

    fn multisig_call_adder_add(&mut self, number: BigUint, caller: &Address, signers: &[&Address]) {
        let action_id = self.multisig_propose_adder_add(number, caller);
        self.multisig_sign_and_perform(action_id, caller, signers);
    }

    fn multisig_propose_adder_add(&mut self, number: BigUint, caller: &Address) -> usize {
        let adder_call = self.adder.add(number);
        self.world
            .sc_call_get_result(ScCallStep::new().from(caller).call(
                self.multisig.propose_transfer_execute(
                    &self.adder.to_address(),
                    0u32,
                    adder_call.endpoint_name,
                    adder_call.arg_buffer.into_multi_value_encoded(),
                ),
            ))
    }

    fn adder_expect_get_sum(&mut self, expected_sum: BigUint, caller: &Address) -> BigUint {
        let value: SingleValue<BigUint> = self.world.sc_query_get_result(
            ScQueryStep::new()
                .call(self.adder.sum())
                .expect_value(SingleValue::from(expected_sum)),
        );
        value.into()
    }
}
