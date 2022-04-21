use adder::ProxyTrait as AdderProxyTrait;
use elrond_wasm::{
    elrond_codec::multi_types::{MultiValueVec, OptionalValue},
    storage::mappers::SingleValue,
    types::{Address, CodeMetadata},
};
use elrond_wasm_debug::{
    mandos::{
        interpret_trait::{InterpretableFrom, InterpreterContext},
        model::{
            Account, AddressKey, AddressValue, ScCallStep, ScDeployStep, SetStateStep, TxExpect,
        },
    },
    BlockchainMock, ContractInfo, DebugApi,
};
use multisig::{
    multisig_perform::ProxyTrait as MultisigPerformProxyTrait,
    multisig_propose::ProxyTrait as MultisigProposeProxyTrait, ProxyTrait as MultisigProxyTrait,
};
use num_bigint::BigUint;

type MultisigContract = ContractInfo<multisig::Proxy<DebugApi>>;
type AdderContract = ContractInfo<adder::Proxy<DebugApi>>;

struct Users {
    alice: Address,
    bob: Address,
    carol: Address,
}

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_contract_builder("file:test-contracts/adder.wasm", adder::ContractBuilder);
    blockchain.register_contract_builder("file:output/multisig.wasm", multisig::ContractBuilder);
    blockchain
}

#[test]
fn basic_setup_test() {
    let _ = DebugApi::dummy();

    let world = &mut world();
    let ic = &world.interpreter_context();

    let users = setup_users(world, ic);

    let multisig = &mut multisig_deploy(&users, world, ic);

    let board_members: MultiValueVec<Address> = world.mandos_sc_call_get_result(
        multisig.get_all_board_members(),
        ScCallStep::new()
            .from(users.alice.as_array())
            .expect(TxExpect::ok()),
    );

    let expected_board_members: Vec<_> = [users.alice, users.bob, users.carol].into();

    assert_eq!(board_members.into_vec(), expected_board_members);
}

#[test]
fn multisig_adder_test() {
    let _ = DebugApi::dummy();

    let world = &mut world();
    let ic = &world.interpreter_context();

    let users = setup_users(world, ic);
    let source_adder = &mut adder_deploy(world, ic);
    let multisig = &mut multisig_deploy(&users, world, ic);
    let caller = &users.alice;
    let signers = [&users.alice, &users.bob, &users.carol];
    let adder = &mut multisig_deploy_adder(
        multisig,
        source_adder,
        caller,
        signers.as_slice(),
        world,
        ic,
    );
    let expected_adder_address = AddressKey::interpret_from("sc:adder-multisig", ic);
    assert_eq!(
        adder.mandos_address_expr.value,
        expected_adder_address.value
    );

    let first_value: BigUint = 42u64.into();
    let second_value: BigUint = 43u64.into();
    let expected_sum = first_value.clone() + second_value.clone();

    multisig_call_adder_add(adder, first_value, caller, &signers, multisig, world);
    multisig_call_adder_add(adder, second_value, caller, &signers, multisig, world);
    adder_expect_get_sum(adder, expected_sum, caller, world);
}

fn adder_deploy(world: &mut BlockchainMock, ic: &InterpreterContext) -> AdderContract {
    let adder_owner = AddressValue::interpret_from("address:adder_owner", &ic);
    let mut adder = AdderContract::new("sc:adder", &ic);

    world.mandos_set_state(
        SetStateStep::new()
            .put_account(&adder_owner, Account::new().nonce(1))
            .new_address(&adder_owner, 1, &adder),
    );

    let (_new_address, ()) = world.mandos_sc_deploy_get_result(
        adder.init(0u64),
        ScDeployStep::new()
            .from(&adder_owner)
            .contract_code("file:test-contracts/adder.wasm", &ic)
            .gas_limit("5,000,000")
            .expect(TxExpect::ok().no_result()),
    );

    adder
}

fn multisig_deploy(
    users: &Users,
    world: &mut BlockchainMock,
    ic: &InterpreterContext,
) -> MultisigContract {
    let owner_address = AddressValue::interpret_from("address:owner", &ic);
    let mut multisig = MultisigContract::new("sc:multisig", &ic);

    world.mandos_set_state(
        SetStateStep::new()
            .put_account(&owner_address, Account::new().nonce(1))
            .new_address(&owner_address, 1, &multisig),
    );

    let board: MultiValueVec<Address> =
        vec![users.alice.clone(), users.bob.clone(), users.carol.clone()].into();

    let (_new_address, ()) = world.mandos_sc_deploy_get_result(
        multisig.init(2u32, board),
        ScDeployStep::new()
            .from(owner_address)
            .contract_code("file:output/multisig.wasm", &ic)
            .gas_limit("5,000,000")
            .expect(TxExpect::ok().no_result()),
    );

    multisig
}

fn address_from(name: &str, ic: &InterpreterContext) -> Address {
    AddressValue::interpret_from(name, ic).value.into()
}

fn setup_users(world: &mut BlockchainMock, ic: &InterpreterContext) -> Users {
    let users = Users {
        alice: address_from("address:alice", ic),
        bob: address_from("address:bob", ic),
        carol: address_from("address:carol", ic),
    };

    world.mandos_set_state(
        SetStateStep::new()
            .put_account(users.alice.as_array(), Account::new().nonce(1))
            .put_account(users.bob.as_array(), Account::new().nonce(1))
            .put_account(users.carol.as_array(), Account::new().nonce(1)),
    );

    users
}

fn multisig_sign(
    action_id: usize,
    signer: &Address,
    multisig: &mut MultisigContract,
    world: &mut BlockchainMock,
) {
    let () = world.mandos_sc_call_get_result(
        multisig.sign(action_id),
        ScCallStep::new()
            .from(signer.as_array())
            .gas_limit("5,000,000")
            .expect(TxExpect::ok().no_result()),
    );
}

fn multisig_sign_multiple(
    action_id: usize,
    signers: &[&Address],
    multisig: &mut MultisigContract,
    world: &mut BlockchainMock,
) {
    for &signer in signers {
        multisig_sign(action_id, signer, multisig, world)
    }
}

fn multisig_perform(
    action_id: usize,
    caller: &Address,
    multisig: &mut MultisigContract,
    world: &mut BlockchainMock,
) -> Option<Address> {
    let result: OptionalValue<Address> = world.mandos_sc_call_get_result(
        multisig.perform_action_endpoint(action_id),
        ScCallStep::new()
            .from(caller.as_array())
            .gas_limit("5,000,000")
            .expect(TxExpect::ok()),
    );
    result.into_option()
}

fn multisig_sign_and_perform(
    action_id: usize,
    caller: &Address,
    signers: &[&Address],
    multisig: &mut MultisigContract,
    world: &mut BlockchainMock,
) -> Option<Address> {
    multisig_sign_multiple(action_id, signers, multisig, world);
    multisig_perform(action_id, caller, multisig, world)
}

fn multisig_deploy_adder(
    multisig: &mut MultisigContract,
    source_adder: &mut AdderContract,
    caller: &Address,
    signers: &[&Address],
    world: &mut BlockchainMock,
    ic: &InterpreterContext,
) -> AdderContract {
    let action_id = multisig_propose_adder_deploy(multisig, source_adder, &caller, ic, world);
    let address = multisig_sign_and_perform(action_id, caller, signers, multisig, world).unwrap();
    AdderContract::new(address.as_array(), ic)
}

fn multisig_propose_adder_deploy(
    multisig: &mut MultisigContract,
    source_adder: &mut AdderContract,
    caller: &Address,
    ic: &InterpreterContext,
    world: &mut BlockchainMock,
) -> usize {
    let adder_multisig = AddressValue::interpret_from("sc:adder-multisig", &ic);

    world.mandos_set_state(SetStateStep::new().new_address(
        &multisig.mandos_address_expr.value,
        0,
        adder_multisig,
    ));

    let source_adder_address: Address = source_adder.mandos_address_expr.value.into();

    let action_id = world.mandos_sc_call_get_result(
        multisig.propose_sc_deploy_from_source(
            0u64,
            source_adder_address,
            CodeMetadata::DEFAULT,
            source_adder
                .init(0u64)
                .arg_buffer
                .into_multi_value_encoded(),
        ),
        ScCallStep::new()
            .from(caller.as_array())
            .gas_limit("5,000,000")
            .expect(TxExpect::ok()),
    );
    action_id
}

fn multisig_call_adder_add(
    adder: &mut AdderContract,
    number: BigUint,
    caller: &Address,
    signers: &[&Address],
    multisig: &mut MultisigContract,
    world: &mut BlockchainMock,
) {
    let action_id = multisig_propose_adder_add(adder, number, caller, multisig, world);
    multisig_sign_and_perform(action_id, caller, signers, multisig, world);
}

fn multisig_propose_adder_add(
    adder: &mut AdderContract,
    number: BigUint,
    caller: &Address,
    multisig: &mut MultisigContract,
    world: &mut BlockchainMock,
) -> usize {
    let adder_call = adder.add(number);
    world.mandos_sc_call_get_result(
        multisig.propose_transfer_execute(
            adder.mandos_address_expr.value,
            0u32,
            adder_call.endpoint_name,
            adder_call.arg_buffer.into_multi_value_encoded(),
        ),
        ScCallStep::new()
            .from(caller.as_array())
            .gas_limit("5,000,000")
            .expect(TxExpect::ok()),
    )
}

fn adder_expect_get_sum(
    adder: &mut AdderContract,
    expected_sum: BigUint,
    caller: &Address,
    world: &mut BlockchainMock,
) -> BigUint {
    let value: SingleValue<BigUint> = world.mandos_sc_call_get_result(
        adder.sum(),
        ScCallStep::new()
            .from(caller.as_array())
            .gas_limit("5,000,000")
            .expect(TxExpect::ok().result(&format!("{}", expected_sum))),
    );
    value.into()
}
