use std::{cell::RefCell, rc::Rc};

use rust_snippets_generator_test::ProxyTrait as _;
use rust_snippets_generator_test::*;

use multiversx_sc::{types::*, codec::multi_types::*};
use multiversx_sc_scenario::{*, testing_framework::*};

type RustBigUint = num_bigint::BigUint;

pub struct RustSnippetsGeneratorTestSetup<RustSnippetsGeneratorTestObjBuilder>
where
    RustSnippetsGeneratorTestObjBuilder: 'static + Copy + Fn() -> rust_snippets_generator_test::ContractObj<DebugApi>,
{
    pub b_mock: Rc<RefCell<BlockchainStateWrapper>>,
    pub owner: Address,
    pub sc_wrapper:
        ContractObjWrapper<rust_snippets_generator_test::ContractObj<DebugApi>, RustSnippetsGeneratorTestObjBuilder>,
}

impl<RustSnippetsGeneratorTestObjBuilder> RustSnippetsGeneratorTestSetup<RustSnippetsGeneratorTestObjBuilder>
where
    RustSnippetsGeneratorTestObjBuilder: 'static + Copy + Fn() -> rust_snippets_generator_test::ContractObj<DebugApi>,
{
    pub fn new(
        b_mock: Rc<RefCell<BlockchainStateWrapper>>,
        builder: RustSnippetsGeneratorTestObjBuilder,
        caller: &Address, 
    ) -> Self {
        let owner = b_mock.borrow_mut().create_user_account(&rust_biguint!(0));
        let sc_wrapper = b_mock
            .borrow_mut()
            .create_sc_account(&rust_biguint!(0), Some(&owner), builder, "rust-snippets-generator-test.wasm");
            
        b_mock
            .borrow_mut()
            .execute_tx(&owner, &sc_wrapper, &rust_biguint!(0), |sc| {
                sc.init(caller, &sc_wrapper);
            })
            .assert_ok();
            
        Self {
            b_mock,
            owner,
            sc_wrapper
        }
    }

pub fn no_arg_no_result_endpoint(&self, caller: &Address, ) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper)
        }

pub fn no_arg_one_result_endpoint(&self, caller: &Address, ) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper)
        }

pub fn one_arg_no_result_endpoint(&self, caller: &Address, _arg: u64) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn one_arg_one_result_endpoint(&self, caller: &Address, _arg: u64) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn multi_result(&self, caller: &Address, _arg: &[u8]) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn nested_result(&self, caller: &Address, _arg: &[u8]) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn custom_struct(&self, caller: &Address, _arg: MyCoolStruct<DebugApi>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn optional_type(&self, caller: &Address, _arg: OptionalValue<RustBigUint>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn option_type(&self, caller: &Address, _arg: Option<ManagedVec<DebugApi, &[u8]>>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn esdt_token_payment(&self, caller: &Address, _arg: OptionalValue<TxTokenTransfer>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn egld_or_esdt_payment(&self, caller: &Address, arg: TxTokenTransfer) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, arg)
        }

pub fn egld_only_endpoint(&self, caller: &Address, egld_value: RustBigUint) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper)
        }

pub fn payable_endpoint(&self, caller: &Address, esdt_transfers: Vec<TxTokenTransfer>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_esdt_multi_transfer(caller, &sc_wrapper)
        }

pub fn managed_buffer(&self, caller: &Address, _arg: Option<&[u8]>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, _arg)
        }

pub fn multi_value_2(&self, caller: &Address, arg: MultiValue2<u64, RustBigUint>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, arg)
        }

pub fn multi_value_4(&self, caller: &Address, arg: MultiValue4<u64, RustBigUint, MyCoolStruct<DebugApi>, &[u8]>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, arg)
        }

pub fn complex_multi_values(&self, caller: &Address, arg: MultiValueVec<MultiValue3<&[u8], u64, RustBigUint>>) -> TxResult {
            self.b_mock.borrow_mut()
                .execute_tx(caller, &sc_wrapper, arg)
        }

}
