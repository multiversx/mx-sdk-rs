use multisig::{
    multisig_perform::MultisigPerformModule, multisig_propose::MultisigProposeModule,
    user_role::UserRole, Multisig,
};
use multiversx_sc::{
    api::ManagedTypeApi,
    codec::multi_types::OptionalValue,
    types::{Address, BigUint, BoxedBytes, CodeMetadata, ManagedBuffer, ManagedVec},
};
use multiversx_sc_scenario::{managed_address, rust_biguint, testing_framework::*, DebugApi};

const MULTISIG_WASM_PATH: &str = "multisig/output/multisig.wasm";
const QUORUM_SIZE: usize = 1;
pub const EGLD_TOKEN_ID: &[u8] = b"EGLD";

pub type RustBigUint = num_bigint::BigUint;

pub enum ActionRaw {
    _Nothing,
    AddBoardMember(Address),
    AddProposer(Address),
    RemoveUser(Address),
    ChangeQuorum(usize),
    SendTransferExecute(CallActionDataRaw),
    SendAsyncCall(CallActionDataRaw),
    SCDeployFromSource {
        amount: RustBigUint,
        source: Address,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
    SCUpgradeFromSource {
        sc_address: Address,
        amount: RustBigUint,
        source: Address,
        code_metadata: CodeMetadata,
        arguments: Vec<BoxedBytes>,
    },
}

pub struct CallActionDataRaw {
    pub to: Address,
    pub egld_amount: RustBigUint,
    pub endpoint_name: BoxedBytes,
    pub arguments: Vec<BoxedBytes>,
}

pub struct MultisigSetup<MultisigObjBuilder>
where
    MultisigObjBuilder: 'static + Copy + Fn() -> multisig::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub proposer_address: Address,
    pub board_member_address: Address,
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
        let board_member_address = b_mock.create_user_account(&rust_zero);

        let ms_wrapper = b_mock.create_sc_account(
            &rust_zero,
            Some(&owner_address),
            ms_builder,
            MULTISIG_WASM_PATH,
        );
        b_mock
            .execute_tx(&owner_address, &ms_wrapper, &rust_zero, |sc| {
                let mut board_members = ManagedVec::new();
                board_members.push(managed_address!(&board_member_address));

                sc.init(QUORUM_SIZE, board_members.into());
                sc.change_user_role(0, managed_address!(&proposer_address), UserRole::Proposer);
            })
            .assert_ok();

        Self {
            b_mock,
            owner_address,
            proposer_address,
            board_member_address,
            ms_wrapper,
        }
    }

    pub fn call_deposit(&mut self, token: &[u8], amount: u64) -> TxResult {
        if token == b"EGLD" {
            self.b_mock.execute_tx(
                &self.proposer_address,
                &self.ms_wrapper,
                &rust_biguint!(amount),
                |sc| {
                    sc.deposit();
                },
            )
        } else {
            self.b_mock.execute_esdt_transfer(
                &self.proposer_address,
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

    pub fn call_propose(&mut self, action: ActionRaw) -> (usize, TxResult) {
        let egld_amount = match &action {
            ActionRaw::SendTransferExecute(call_data) => call_data.egld_amount.clone(),
            ActionRaw::SendAsyncCall(call_data) => call_data.egld_amount.clone(),
            ActionRaw::SCDeployFromSource { amount, .. } => amount.clone(),
            ActionRaw::SCUpgradeFromSource { amount, .. } => amount.clone(),
            _ => rust_biguint!(0),
        };
        let amount_bytes = egld_amount.to_bytes_be();
        let amount_rust_biguint = num_bigint::BigUint::from_bytes_be(amount_bytes.as_slice());

        let mut action_id = 0;
        let tx_result = self.b_mock.execute_tx(
            &self.proposer_address,
            &self.ms_wrapper,
            &amount_rust_biguint,
            |sc| {
                action_id = match action {
                    ActionRaw::_Nothing => panic!("Invalid action"),
                    ActionRaw::AddBoardMember(addr) => {
                        sc.propose_add_board_member(managed_address!(&addr))
                    },
                    ActionRaw::AddProposer(addr) => {
                        sc.propose_add_proposer(managed_address!(&addr))
                    },
                    ActionRaw::RemoveUser(addr) => sc.propose_remove_user(managed_address!(&addr)),
                    ActionRaw::ChangeQuorum(new_size) => sc.propose_change_quorum(new_size),
                    ActionRaw::SendTransferExecute(call_data) => {
                        let opt_endpoint = if call_data.endpoint_name.is_empty() {
                            OptionalValue::None
                        } else {
                            OptionalValue::Some(ManagedBuffer::new_from_bytes(
                                call_data.endpoint_name.as_slice(),
                            ))
                        };

                        sc.propose_transfer_execute(
                            managed_address!(&call_data.to),
                            BigUint::from_bytes_be(&call_data.egld_amount.to_bytes_be()),
                            opt_endpoint,
                            boxed_bytes_vec_to_managed(call_data.arguments).into(),
                        )
                    },
                    ActionRaw::SendAsyncCall(call_data) => {
                        let opt_endpoint = if call_data.endpoint_name.is_empty() {
                            OptionalValue::None
                        } else {
                            OptionalValue::Some(ManagedBuffer::new_from_bytes(
                                call_data.endpoint_name.as_slice(),
                            ))
                        };

                        sc.propose_async_call(
                            managed_address!(&call_data.to),
                            BigUint::from_bytes_be(&call_data.egld_amount.to_bytes_be()),
                            opt_endpoint,
                            boxed_bytes_vec_to_managed(call_data.arguments).into(),
                        )
                    },
                    ActionRaw::SCDeployFromSource {
                        amount,
                        source,
                        code_metadata,
                        arguments,
                    } => sc.propose_sc_deploy_from_source(
                        BigUint::from_bytes_be(&amount.to_bytes_be()),
                        managed_address!(&source),
                        code_metadata,
                        boxed_bytes_vec_to_managed(arguments).into(),
                    ),
                    ActionRaw::SCUpgradeFromSource {
                        sc_address,
                        amount,
                        source,
                        code_metadata,
                        arguments,
                    } => sc.propose_sc_upgrade_from_source(
                        managed_address!(&sc_address),
                        BigUint::from_bytes_be(&amount.to_bytes_be()),
                        managed_address!(&source),
                        code_metadata,
                        boxed_bytes_vec_to_managed(arguments).into(),
                    ),
                }
            },
        );

        (action_id, tx_result)
    }

    pub fn call_sign(&mut self, action_id: usize) -> TxResult {
        self.b_mock.execute_tx(
            &self.board_member_address,
            &self.ms_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.sign(action_id);
            },
        )
    }

    pub fn call_unsign(&mut self, action_id: usize) -> TxResult {
        self.b_mock.execute_tx(
            &self.board_member_address,
            &self.ms_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unsign(action_id);
            },
        )
    }

    pub fn call_perform_action(&mut self, action_id: usize) -> TxResult {
        self.b_mock.execute_tx(
            &self.board_member_address,
            &self.ms_wrapper,
            &rust_biguint!(0),
            |sc| {
                let _ = sc.perform_action_endpoint(action_id);
            },
        )
    }

    pub fn call_perform_action_with_result(&mut self, action_id: usize) -> (TxResult, Address) {
        let mut addr = Address::zero();
        let tx_result = self.b_mock.execute_tx(
            &self.board_member_address,
            &self.ms_wrapper,
            &rust_biguint!(0),
            |sc| {
                let opt_address = sc.perform_action_endpoint(action_id);
                addr = opt_address.into_option().unwrap().to_address();
            },
        );

        (tx_result, addr)
    }

    pub fn call_discard_action(&mut self, action_id: usize) -> TxResult {
        self.b_mock.execute_tx(
            &self.board_member_address,
            &self.ms_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.discard_action(action_id);
            },
        )
    }
}

fn boxed_bytes_vec_to_managed<M: ManagedTypeApi>(
    raw_vec: Vec<BoxedBytes>,
) -> ManagedVec<M, ManagedBuffer<M>> {
    let mut managed = ManagedVec::new();
    for elem in raw_vec {
        managed.push(ManagedBuffer::new_from_bytes(elem.as_slice()));
    }

    managed
}
