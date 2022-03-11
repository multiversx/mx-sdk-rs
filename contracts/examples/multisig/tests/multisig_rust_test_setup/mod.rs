use elrond_wasm::{
    elrond_codec::multi_types::OptionalValue,
    types::{Address, ManagedVec},
};
use elrond_wasm_debug::{
    managed_address, managed_biguint, managed_token_id, rust_biguint, testing_framework::*,
    tx_mock::TxResult, DebugApi,
};
use multisig::{action::Action, multisig_propose::MultisigProposeModule, Multisig};

const MULTISIG_WASM_PATH: &'static str = "multisig/output/multisig.wasm";
const QUORUM_SIZE: usize = 1;

pub struct MultisigSetup<MultisigObjBuilder>
where
    MultisigObjBuilder: 'static + Copy + Fn() -> multisig::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub proposer_address: Address,
    pub first_board_member: Address,
    pub second_board_member: Address,
    pub ms_wrapper: ContractObjWrapper<multisig::ContractObj<DebugApi>, MultisigObjBuilder>,
}

impl<MultisigObjBuilder> MultisigSetup<MultisigObjBuilder>
where
    MultisigObjBuilder: 'static + Copy + Fn() -> multisig::ContractObj<DebugApi>,
{
    pub fn new(ms_builder: MultisigObjBuilder) -> Self {
        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_address = b_mock.create_user_account(&rust_zero);
        let proposer_address = b_mock.create_user_account(&rust_biguint!(100_000_000));
        let first_board_member = b_mock.create_user_account(&rust_zero);
        let second_board_member = b_mock.create_user_account(&rust_zero);

        let ms_wrapper = b_mock.create_sc_account(
            &rust_zero,
            Some(&owner_address),
            ms_builder,
            MULTISIG_WASM_PATH,
        );
        b_mock
            .execute_tx(&owner_address, &ms_wrapper, &rust_zero, |sc| {
                let mut board_members = ManagedVec::new();
                board_members.push(managed_address!(&first_board_member));
                board_members.push(managed_address!(&second_board_member));

                sc.init(QUORUM_SIZE, board_members.into());
            })
            .assert_ok();

        Self {
            b_mock,
            owner_address,
            proposer_address,
            first_board_member,
            second_board_member,
            ms_wrapper,
        }
    }

    pub fn call_deposit(&mut self, caller: &Address, token: &[u8], amount: u64) -> TxResult {
        if token == b"EGLD" {
            self.b_mock
                .execute_tx(caller, &self.ms_wrapper, &rust_biguint!(amount), |sc| {
                    sc.deposit();
                })
        } else {
            self.b_mock.execute_esdt_transfer(
                caller,
                &self.ms_wrapper,
                token,
                0,
                &rust_biguint!(amount),
                |sc| {
                    sc.deposit();
                },
            )
        }
    }

    pub fn call_propose(
        &mut self,
        proposer: &Address,
        action: Action<DebugApi>,
    ) -> (usize, TxResult) {
        let egld_amount = match &action {
            Action::SendTransferExecute(call_data) => call_data.egld_amount.clone(),
            Action::SendAsyncCall(call_data) => call_data.egld_amount.clone(),
            Action::SCDeployFromSource { amount, .. } => amount.clone(),
            Action::SCUpgradeFromSource { amount, .. } => amount.clone(),
            _ => managed_biguint!(0),
        };
        let amount_bytes = egld_amount.to_bytes_be();
        let amount_rust_biguint = num_bigint::BigUint::from_bytes_be(amount_bytes.as_slice());

        let mut action_id = 0;
        let tx_result =
            self.b_mock
                .execute_tx(proposer, &self.ms_wrapper, &amount_rust_biguint, |sc| {
                    action_id = match action {
                        Action::Nothing => 0,
                        Action::AddBoardMember(addr) => sc.propose_add_board_member(addr),
                        Action::AddProposer(addr) => sc.propose_add_proposer(addr),
                        Action::RemoveUser(addr) => sc.propose_remove_user(addr),
                        Action::ChangeQuorum(new_size) => sc.propose_change_quorum(new_size),
                        Action::SendTransferExecute(call_data) => {
                            let opt_endpoint = if call_data.endpoint_name.is_empty() {
                                OptionalValue::None
                            } else {
                                OptionalValue::Some(call_data.endpoint_name)
                            };
                            sc.propose_transfer_execute(
                                call_data.to,
                                call_data.egld_amount,
                                opt_endpoint,
                                call_data.arguments.into(),
                            )
                        },
                        Action::SendAsyncCall(call_data) => {
                            let opt_endpoint = if call_data.endpoint_name.is_empty() {
                                OptionalValue::None
                            } else {
                                OptionalValue::Some(call_data.endpoint_name)
                            };
                            sc.propose_async_call(
                                call_data.to,
                                call_data.egld_amount,
                                opt_endpoint,
                                call_data.arguments.into(),
                            )
                        },
                        Action::SCDeployFromSource {
                            amount,
                            source,
                            code_metadata,
                            arguments,
                        } => sc.propose_sc_deploy_from_source(
                            amount,
                            source,
                            code_metadata,
                            arguments.into(),
                        ),
                        Action::SCUpgradeFromSource {
                            sc_address,
                            amount,
                            source,
                            code_metadata,
                            arguments,
                        } => sc.propose_sc_upgrade_from_source(
                            sc_address,
                            amount,
                            source,
                            code_metadata,
                            arguments.into(),
                        ),
                    }
                });

        (action_id, tx_result)
    }
}
