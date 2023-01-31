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
                sc.init();
            })
            .assert_ok();
            
        Self {
            b_mock,
            owner,
            sc_wrapper
        }
    }

    pub fn no_arg_no_result_endpoint(&self, caller: &Address, ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.no_arg_no_result_endpoint();
            })
    }

    pub fn no_arg_one_result_endpoint(&self, caller: &Address, ) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.no_arg_one_result_endpoint();
            })
    }

    pub fn one_arg_no_result_endpoint(&self, caller: &Address, _arg: u64) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.one_arg_no_result_endpoint(_arg.into());
            })
    }

    pub fn one_arg_one_result_endpoint(&self, caller: &Address, _arg: u64) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.one_arg_one_result_endpoint(_arg.into());
            })
    }

    pub fn multi_result(&self, caller: &Address, _arg: ManagedBuffer<DebugApi>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.multi_result(_arg.into());
            })
    }

    pub fn nested_result(&self, caller: &Address, _arg: ManagedBuffer<DebugApi>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.nested_result(_arg.into());
            })
    }

    pub fn custom_struct(&self, caller: &Address, _arg: MyCoolStruct<DebugApi>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.custom_struct(_arg.into());
            })
    }

    pub fn optional_type(&self, caller: &Address, _arg: OptionalValue<RustBigUint>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.optional_type(_arg.into());
            })
    }

    pub fn option_type(&self, caller: &Address, _arg: Option<ManagedVec<DebugApi, ManagedBuffer<DebugApi>>>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.option_type(_arg.into());
            })
    }

    pub fn esdt_token_payment(&self, caller: &Address, _arg: OptionalValue<TxTokenTransfer>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.esdt_token_payment(_arg.into());
            })
    }

    pub fn egld_or_esdt_payment(&self, caller: &Address, arg: TxTokenTransfer) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.egld_or_esdt_payment(arg.into());
            })
    }

    pub fn egld_only_endpoint(&self, caller: &Address, egld_value: &RustBigUint) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, egld_value, |sc| {
                sc.egld_only_endpoint();
            })
    }

    pub fn payable_endpoint(&self, caller: &Address, esdt_transfers: &[TxTokenTransfer]) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_esdt_multi_transfer(caller, &self.sc_wrapper, esdt_transfers, |sc| {
                sc.payable_endpoint();
            })
    }

    pub fn managed_buffer(&self, caller: &Address, _arg: Option<ManagedBuffer<DebugApi>>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.managed_buffer(_arg.into());
            })
    }

    pub fn multi_value_2(&self, caller: &Address, arg: MultiValue2<u64, RustBigUint>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.multi_value_2(arg.into());
            })
    }

    pub fn multi_value_4(&self, caller: &Address, arg: MultiValue4<u64, RustBigUint, MyCoolStruct<DebugApi>, ManagedBuffer<DebugApi>>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.multi_value_4(arg.into());
            })
    }

    pub fn complex_multi_values(&self, caller: &Address, arg: MultiValueVec<MultiValue3<ManagedBuffer<DebugApi>, u64, RustBigUint>>) -> TxResult {
        self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                sc.complex_multi_values(arg.into());
            })
    }

}
