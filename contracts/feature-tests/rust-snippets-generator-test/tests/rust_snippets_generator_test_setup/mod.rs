#![allow(dead_code, unused_imports)]

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
        let owner = caller.clone();
        let sc_wrapper = b_mock
            .borrow_mut()
            .create_sc_account(&rust_biguint!(0), Some(&owner), builder, "rust-snippets-generator-test.wasm");
            
        b_mock
            .borrow_mut()
            .execute_tx(&owner, &sc_wrapper, &rust_biguint!(0), |sc| {
                let _ = sc.init();
            })
            .assert_ok();
            
        Self {
            b_mock,
            owner,
            sc_wrapper
        }
    }

    pub fn no_arg_no_result_endpoint(&self, caller: &Address, ) -> WrappedTxResult<()> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.no_arg_no_result_endpoint();
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn no_arg_one_result_endpoint(&self, caller: &Address, ) -> WrappedTxResult<u64> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.no_arg_one_result_endpoint();
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn one_arg_no_result_endpoint(&self, caller: &Address, _arg: u64) -> WrappedTxResult<()> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.one_arg_no_result_endpoint(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn one_arg_one_result_endpoint(&self, caller: &Address, _arg: u64) -> WrappedTxResult<BigUint<DebugApi>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.one_arg_one_result_endpoint(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn multi_result(&self, caller: &Address, _arg: TokenIdentifier<DebugApi>) -> WrappedTxResult<MultiValueVec<BigUint<DebugApi>>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.multi_result(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn nested_result(&self, caller: &Address, _arg: TokenIdentifier<DebugApi>) -> WrappedTxResult<ManagedVec<DebugApi, ManagedVec<DebugApi, BigUint<DebugApi>>>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.nested_result(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn custom_struct(&self, caller: &Address, _arg: MyCoolStruct<DebugApi>) -> WrappedTxResult<MyCoolStruct<DebugApi>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.custom_struct(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn optional_type(&self, caller: &Address, _arg: OptionalValue<BigUint<DebugApi>>) -> WrappedTxResult<OptionalValue<TokenIdentifier<DebugApi>>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.optional_type(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn option_type(&self, caller: &Address, _arg: Option<ManagedVec<DebugApi, TokenIdentifier<DebugApi>>>) -> WrappedTxResult<Option<u64>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.option_type(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn esdt_token_payment(&self, caller: &Address, _arg: OptionalValue<EsdtTokenPayment<DebugApi>>) -> WrappedTxResult<EsdtTokenPayment<DebugApi>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.esdt_token_payment(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn egld_or_esdt_payment(&self, caller: &Address, arg: EgldOrEsdtTokenPayment<DebugApi>) -> WrappedTxResult<EgldOrEsdtTokenIdentifier<DebugApi>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.egld_or_esdt_payment(arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn egld_only_endpoint(&self, caller: &Address, egld_value: &RustBigUint) -> WrappedTxResult<()> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, egld_value, |sc| {
                let res = sc.egld_only_endpoint();
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn payable_endpoint(&self, caller: &Address, esdt_transfers: &[TxTokenTransfer]) -> WrappedTxResult<()> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_esdt_multi_transfer(caller, &self.sc_wrapper, esdt_transfers, |sc| {
                let res = sc.payable_endpoint();
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn managed_buffer(&self, caller: &Address, _arg: Option<ManagedBuffer<DebugApi>>) -> WrappedTxResult<MultiValueVec<ManagedVec<DebugApi, MyCoolStruct<DebugApi>>>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.managed_buffer(_arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn multi_value_2(&self, caller: &Address, arg: MultiValue2<u64, BigUint<DebugApi>>) -> WrappedTxResult<MultiValue2<u64, BigUint<DebugApi>>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.multi_value_2(arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn multi_value_4(&self, caller: &Address, arg: MultiValue4<u64, BigUint<DebugApi>, MyCoolStruct<DebugApi>, TokenIdentifier<DebugApi>>) -> WrappedTxResult<MultiValue4<u64, BigUint<DebugApi>, MyCoolStruct<DebugApi>, TokenIdentifier<DebugApi>>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.multi_value_4(arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn complex_multi_values(&self, caller: &Address, arg: MultiValueVec<MultiValue3<TokenIdentifier<DebugApi>, u64, BigUint<DebugApi>>>) -> WrappedTxResult<MultiValueVec<MultiValue3<TokenIdentifier<DebugApi>, u64, BigUint<DebugApi>>>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_tx(caller, &self.sc_wrapper, &rust_biguint!(0), |sc| {
                let res = sc.complex_multi_values(arg.into());
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn view_func(&self, ) -> WrappedTxResult<u64> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_query(&self.sc_wrapper, |sc| {
                let res = sc.view_func();
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

    pub fn view_custom_type(&self, ) -> WrappedTxResult<MyCoolStruct<DebugApi>> {
        let mut opt_endpoint_result = Option::None;
        let tx_result = self.b_mock
            .borrow_mut()
            .execute_query(&self.sc_wrapper, |sc| {
                let res = sc.view_custom_type();
                opt_endpoint_result = Some(res);
            });

        WrappedTxResult::new(tx_result, opt_endpoint_result)
    }

}
