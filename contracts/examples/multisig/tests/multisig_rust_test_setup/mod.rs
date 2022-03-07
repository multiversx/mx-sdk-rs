use elrond_wasm::types::{Address, ManagedVec};
use elrond_wasm_debug::{
    managed_address, managed_biguint, managed_token_id, rust_biguint, testing_framework::*,
    tx_mock::TxResult, DebugApi,
};
use multisig::*;

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

    pub fn call_propose(&mut self, proposer: &Address) {}
}
