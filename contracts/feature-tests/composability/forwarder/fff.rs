#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2018::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
mod call_async {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use elrond_wasm::elrond_codec;
    use elrond_wasm::elrond_codec::elrond_codec_derive::{
        NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode, TopEncodeOrDefault,
    };
    use elrond_wasm_derive::TypeAbi;
    pub struct CallbackData<BigUint: BigUintApi> {
        callback_name: BoxedBytes,
        token_identifier: TokenIdentifier,
        token_nonce: u64,
        token_amount: BigUint,
        args: Vec<BoxedBytes>,
    }
    impl<BigUint: BigUintApi> elrond_codec::TopEncode for CallbackData<BigUint> {
        fn top_encode<O: elrond_codec::TopEncodeOutput>(
            &self,
            output: O,
        ) -> core::result::Result<(), elrond_codec::EncodeError> {
            let mut buffer = elrond_codec::Vec::<u8>::new();
            let dest = &mut buffer;
            elrond_codec::NestedEncode::dep_encode(&self.callback_name, dest)?;
            elrond_codec::NestedEncode::dep_encode(&self.token_identifier, dest)?;
            elrond_codec::NestedEncode::dep_encode(&self.token_nonce, dest)?;
            elrond_codec::NestedEncode::dep_encode(&self.token_amount, dest)?;
            elrond_codec::NestedEncode::dep_encode(&self.args, dest)?;
            output.set_slice_u8(&buffer[..]);
            core::result::Result::Ok(())
        }
        fn top_encode_or_exit<O: elrond_codec::TopEncodeOutput, ExitCtx: Clone>(
            &self,
            output: O,
            c: ExitCtx,
            exit: fn(ExitCtx, elrond_codec::EncodeError) -> !,
        ) {
            let mut buffer = elrond_codec::Vec::<u8>::new();
            let dest = &mut buffer;
            elrond_codec::NestedEncode::dep_encode_or_exit(
                &self.callback_name,
                dest,
                c.clone(),
                exit,
            );
            elrond_codec::NestedEncode::dep_encode_or_exit(
                &self.token_identifier,
                dest,
                c.clone(),
                exit,
            );
            elrond_codec::NestedEncode::dep_encode_or_exit(
                &self.token_nonce,
                dest,
                c.clone(),
                exit,
            );
            elrond_codec::NestedEncode::dep_encode_or_exit(
                &self.token_amount,
                dest,
                c.clone(),
                exit,
            );
            elrond_codec::NestedEncode::dep_encode_or_exit(&self.args, dest, c.clone(), exit);
            output.set_slice_u8(&buffer[..]);
        }
    }
    impl<BigUint: BigUintApi> elrond_codec::TopDecode for CallbackData<BigUint> {
        fn top_decode<I: elrond_codec::TopDecodeInput>(
            top_input: I,
        ) -> core::result::Result<Self, elrond_codec::DecodeError> {
            let bytes = top_input.into_boxed_slice_u8();
            let input = &mut &*bytes;
            let result = CallbackData {
                callback_name: <BoxedBytes as elrond_codec::NestedDecode>::dep_decode(input)?,
                token_identifier: <TokenIdentifier as elrond_codec::NestedDecode>::dep_decode(
                    input,
                )?,
                token_nonce: <u64 as elrond_codec::NestedDecode>::dep_decode(input)?,
                token_amount: <BigUint as elrond_codec::NestedDecode>::dep_decode(input)?,
                args: <Vec<BoxedBytes> as elrond_codec::NestedDecode>::dep_decode(input)?,
            };
            if !input.is_empty() {
                return core::result::Result::Err(elrond_codec::DecodeError::INPUT_TOO_LONG);
            }
            core::result::Result::Ok(result)
        }
        fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
            top_input: I,
            c: ExitCtx,
            exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
        ) -> Self {
            let bytes = top_input.into_boxed_slice_u8();
            let input = &mut &*bytes;
            let result = CallbackData {
                callback_name: <BoxedBytes as elrond_codec::NestedDecode>::dep_decode_or_exit(
                    input,
                    c.clone(),
                    exit,
                ),
                token_identifier:
                    <TokenIdentifier as elrond_codec::NestedDecode>::dep_decode_or_exit(
                        input,
                        c.clone(),
                        exit,
                    ),
                token_nonce: <u64 as elrond_codec::NestedDecode>::dep_decode_or_exit(
                    input,
                    c.clone(),
                    exit,
                ),
                token_amount: <BigUint as elrond_codec::NestedDecode>::dep_decode_or_exit(
                    input,
                    c.clone(),
                    exit,
                ),
                args: <Vec<BoxedBytes> as elrond_codec::NestedDecode>::dep_decode_or_exit(
                    input,
                    c.clone(),
                    exit,
                ),
            };
            if !input.is_empty() {
                exit(c, elrond_codec::DecodeError::INPUT_TOO_LONG);
            }
            result
        }
    }
    impl<BigUint: BigUintApi> elrond_wasm::abi::TypeAbi for CallbackData<BigUint> {
        fn type_name() -> elrond_wasm::String {
            "CallbackData".into()
        }
        fn provide_type_descriptions<TDC: elrond_wasm::abi::TypeDescriptionContainer>(
            accumulator: &mut TDC,
        ) {
            let type_name = Self::type_name();
            if !accumulator.contains_type(&type_name) {
                accumulator.reserve_type_name(type_name.clone());
                let mut field_descriptions = elrond_wasm::Vec::new();
                field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                    docs: &[],
                    name: "callback_name",
                    field_type: <BoxedBytes>::type_name(),
                });
                <BoxedBytes>::provide_type_descriptions(accumulator);
                field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                    docs: &[],
                    name: "token_identifier",
                    field_type: <TokenIdentifier>::type_name(),
                });
                <TokenIdentifier>::provide_type_descriptions(accumulator);
                field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                    docs: &[],
                    name: "token_nonce",
                    field_type: <u64>::type_name(),
                });
                <u64>::provide_type_descriptions(accumulator);
                field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                    docs: &[],
                    name: "token_amount",
                    field_type: <BigUint>::type_name(),
                });
                <BigUint>::provide_type_descriptions(accumulator);
                field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                    docs: &[],
                    name: "args",
                    field_type: <Vec<BoxedBytes>>::type_name(),
                });
                <Vec<BoxedBytes>>::provide_type_descriptions(accumulator);
                accumulator.insert(
                    type_name.clone(),
                    elrond_wasm::abi::TypeDescription {
                        docs: &[],
                        name: type_name,
                        contents: elrond_wasm::abi::TypeContents::Struct(field_descriptions),
                    },
                );
            }
        }
    }
    use vault::ProxyTrait as _;
    pub trait ForwarderAsyncCallModule: elrond_wasm::api::ContractBase + Sized
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn forward_async_accept_funds(
            &self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) -> AsyncCall<Self::SendApi> {
            self.vault_proxy()
                .contract(to)
                .accept_funds(token, payment)
                .with_nft_nonce(token_nonce)
                .async_call()
        }
        fn forward_async_accept_funds_half_payment(
            &self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
        ) -> AsyncCall<Self::SendApi> {
            let half_payment = payment / 2u32.into();
            self.vault_proxy()
                .contract(to)
                .accept_funds(token, half_payment)
                .async_call()
        }
        fn forward_async_retrieve_funds(
            &self,
            to: Address,
            token: TokenIdentifier,
            token_nonce: u64,
            amount: Self::BigUint,
        ) -> AsyncCall<Self::SendApi> {
            self.vault_proxy()
                .contract(to)
                .retrieve_funds(token, token_nonce, amount, OptionalArg::None)
                .async_call()
                .with_callback(self.callbacks().retrieve_funds_callback())
        }
        fn retrieve_funds_callback(
            &self,
            token: TokenIdentifier,
            nonce: u64,
            payment: Self::BigUint,
        ) {
            self.retrieve_funds_callback_event(&token, nonce, &payment);
            let _ = self.callback_data().push(&CallbackData {
                callback_name: BoxedBytes::from(&b"retrieve_funds_callback"[..]),
                token_identifier: token,
                token_nonce: nonce,
                token_amount: payment,
                args: Vec::new(),
            });
        }
        fn send_funds_twice(
            &self,
            to: &Address,
            token_identifier: &TokenIdentifier,
            amount: &Self::BigUint,
        ) -> AsyncCall<Self::SendApi> {
            self.vault_proxy()
                .contract(to.clone())
                .accept_funds(token_identifier.clone(), amount.clone())
                .async_call()
                .with_callback(self.callbacks().send_funds_twice_callback(
                    to,
                    token_identifier,
                    amount,
                ))
        }
        fn send_funds_twice_callback(
            &self,
            to: &Address,
            token_identifier: &TokenIdentifier,
            cb_amount: &Self::BigUint,
        ) -> AsyncCall<Self::SendApi> {
            self.vault_proxy()
                .contract(to.clone())
                .accept_funds(token_identifier.clone(), cb_amount.clone())
                .async_call()
        }
        fn callback_data_at_index(
            &self,
            index: usize,
        ) -> MultiResult5<BoxedBytes, TokenIdentifier, u64, Self::BigUint, MultiResultVec<BoxedBytes>>
        {
            let cb_data = self.callback_data().get(index);
            (
                cb_data.callback_name,
                cb_data.token_identifier,
                cb_data.token_nonce,
                cb_data.token_amount,
                cb_data.args.into(),
            )
                .into()
        }
        fn clear_callback_data(&self) {
            self.callback_data().clear();
        }
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;
        fn retrieve_funds_callback_event(
            &self,
            token: &TokenIdentifier,
            nonce: u64,
            payment: &Self::BigUint,
        );
        fn callback_data(&self) -> VecMapper<Self::Storage, CallbackData<Self::BigUint>>;
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ForwarderAsyncCallModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl,
    {
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi> {
            vault::Proxy::new_proxy_obj(self.send())
        }
        fn retrieve_funds_callback_event(
            &self,
            token: &TokenIdentifier,
            nonce: u64,
            payment: &Self::BigUint,
        ) {
            let mut ___topic_buffer___ = elrond_wasm::types::ArgBuffer::new();
            ___topic_buffer___.push_argument_bytes(
                &[
                    114u8, 101u8, 116u8, 114u8, 105u8, 101u8, 118u8, 101u8, 95u8, 102u8, 117u8,
                    110u8, 100u8, 115u8, 95u8, 99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8,
                ][..],
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nonce,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                payment,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            self.log_api_raw().write_event_log(&___topic_buffer___, &[]);
        }
        fn callback_data(&self) -> VecMapper<Self::Storage, CallbackData<Self::BigUint>> {
            let key: &'static [u8] = b"callback_data";
            < VecMapper < Self :: Storage , CallbackData < Self :: BigUint > > as elrond_wasm :: storage :: mappers :: StorageMapper < Self :: Storage > > :: new (self . get_storage_raw () , elrond_wasm :: types :: BoxedBytes :: from (key))
        }
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi> {
            < self :: CallbackProxyObj < Self :: SendApi > as elrond_wasm :: api :: CallbackProxyObjApi > :: new_cb_proxy_obj (self . send ())
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + ForwarderAsyncCallModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_forward_async_accept_funds(&self) {
            let (payment, token) = self.call_value().payment_token_pair();
            let token_nonce = self.call_value().esdt_token_nonce();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let result = self.forward_async_accept_funds(to, token, payment, token_nonce);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_forward_async_accept_funds_half_payment(&self) {
            let (payment, token) = self.call_value().payment_token_pair();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let result = self.forward_async_accept_funds_half_payment(to, token, payment);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_forward_async_retrieve_funds(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 4i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let token = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"token"[..]),
            );
            let token_nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"token_nonce"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                3i32,
                ArgId::from(&b"amount"[..]),
            );
            let result = self.forward_async_retrieve_funds(to, token, token_nonce, amount);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_send_funds_twice(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 3i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"amount"[..]),
            );
            let result = self.send_funds_twice(&to, &token_identifier, &amount);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_callback_data(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
            let result = self.callback_data();
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_callback_data_at_index(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let index = elrond_wasm::load_single_arg::<Self::ArgumentApi, usize>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"index"[..]),
            );
            let result = self.callback_data_at_index(index);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_clear_callback_data(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
            self.clear_callback_data();
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 97u8, 115u8, 121u8, 110u8, 99u8, 95u8, 97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8] =>
                {
                    self.call_forward_async_accept_funds();
                    true
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 97u8, 115u8, 121u8, 110u8, 99u8, 95u8, 97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8, 95u8, 104u8, 97u8, 108u8, 102u8, 95u8, 112u8, 97u8, 121u8, 109u8, 101u8, 110u8, 116u8] =>
                {
                    self.call_forward_async_accept_funds_half_payment();
                    true
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 97u8, 115u8, 121u8, 110u8, 99u8, 95u8, 114u8, 101u8, 116u8, 114u8, 105u8, 101u8, 118u8, 101u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8] =>
                {
                    self.call_forward_async_retrieve_funds();
                    true
                }
                [115u8, 101u8, 110u8, 100u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8, 95u8, 116u8, 119u8, 105u8, 99u8, 101u8] =>
                {
                    self.call_send_funds_twice();
                    true
                }
                [99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8, 95u8, 100u8, 97u8, 116u8, 97u8] =>
                {
                    self.call_callback_data();
                    true
                }
                [99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8, 95u8, 100u8, 97u8, 116u8, 97u8, 95u8, 97u8, 116u8, 95u8, 105u8, 110u8, 100u8, 101u8, 120u8] =>
                {
                    self.call_callback_data_at_index();
                    true
                }
                [99u8, 108u8, 101u8, 97u8, 114u8, 95u8, 99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8, 95u8, 100u8, 97u8, 116u8, 97u8] =>
                {
                    self.call_clear_callback_data();
                    true
                }
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            let mut ___call_result_loader___ = EndpointDynArgLoader::new(self.argument_api());
            match ___cb_data_deserializer___.get_func_name() {
                [] => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                [114u8, 101u8, 116u8, 114u8, 105u8, 101u8, 118u8, 101u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8, 95u8, 99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8] =>
                {
                    let (payment, token) = self.call_value().payment_token_pair();
                    let nonce = self.call_value().esdt_token_nonce();
                    let mut ___cb_closure_loader___ =
                        CallDataArgLoader::new(___cb_data_deserializer___, self.error_api());
                    ___cb_closure_loader___.assert_no_more_args();
                    self.retrieve_funds_callback(token, nonce, payment);
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                [115u8, 101u8, 110u8, 100u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8, 95u8, 116u8, 119u8, 105u8, 99u8, 101u8, 95u8, 99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8] =>
                {
                    let mut ___cb_closure_loader___ =
                        CallDataArgLoader::new(___cb_data_deserializer___, self.error_api());
                    let to: &Address = &elrond_wasm::load_dyn_arg(
                        &mut ___cb_closure_loader___,
                        ArgId::from(&b"to"[..]),
                    );
                    let token_identifier: &TokenIdentifier = &elrond_wasm::load_dyn_arg(
                        &mut ___cb_closure_loader___,
                        ArgId::from(&b"token_identifier"[..]),
                    );
                    let cb_amount: &Self::BigUint = &elrond_wasm::load_dyn_arg(
                        &mut ___cb_closure_loader___,
                        ArgId::from(&b"cb_amount"[..]),
                    );
                    ___cb_closure_loader___.assert_no_more_args();
                    let result = self.send_funds_twice_callback(&to, &token_identifier, &cb_amount);
                    elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                _ => {}
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {
            let ___tx_hash___ = elrond_wasm::api::BlockchainApi::get_tx_hash(&self.blockchain());
            let ___cb_data_raw___ = elrond_wasm::api::StorageReadApi::storage_load_boxed_bytes(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
            );
            elrond_wasm::api::StorageWriteApi::storage_store_slice_u8(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
                &[],
            );
            let mut ___cb_data_deserializer___ =
                elrond_wasm::hex_call_data::HexCallDataDeserializer::new(
                    ___cb_data_raw___.as_slice(),
                );
            if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                self::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)
            {
                self.error_api().signal_error(err_msg::CALLBACK_BAD_FUNC);
            }
        }
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ForwarderAsyncCallModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "forward_async_accept_funds",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "forward_async_accept_funds_half_payment",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "forward_async_retrieve_funds",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<u64>("token_nonce");
            contract_abi.add_type_descriptions::<u64>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "send_funds_twice",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&Address>("to");
            contract_abi.add_type_descriptions::<&Address>();
            endpoint_abi.add_input::<&TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<&TokenIdentifier>();
            endpoint_abi.add_input::<&Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<&Self::BigUint>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "callback_data",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_output::<VecMapper<Self::Storage, CallbackData<Self::BigUint>>>(&[]);
            contract_abi
                .add_type_descriptions::<VecMapper<Self::Storage, CallbackData<Self::BigUint>>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "callback_data_at_index",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<usize>("index");
            contract_abi.add_type_descriptions::<usize>();
            endpoint_abi.add_output::<MultiResult5<
                BoxedBytes,
                TokenIdentifier,
                u64,
                Self::BigUint,
                MultiResultVec<BoxedBytes>,
            >>(&[]);
            contract_abi.add_type_descriptions::<MultiResult5<
                BoxedBytes,
                TokenIdentifier,
                u64,
                Self::BigUint,
                MultiResultVec<BoxedBytes>,
            >>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "clear_callback_data",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
        #[allow(clippy::too_many_arguments)]
        fn forward_async_accept_funds(
            self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                token,
                payment,
                token_nonce,
                elrond_wasm::types::BoxedBytes::from(&b"forward_async_accept_funds"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn forward_async_accept_funds_half_payment(
            self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                token,
                payment,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(
                    &b"forward_async_accept_funds_half_payment"[..],
                ),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn forward_async_retrieve_funds(
            self,
            to: Address,
            token: TokenIdentifier,
            token_nonce: u64,
            amount: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"forward_async_retrieve_funds"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_nonce,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn send_funds_twice(
            self,
            to: &Address,
            token_identifier: &TokenIdentifier,
            amount: &Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"send_funds_twice"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]        fn callback_data (self) -> elrond_wasm :: types :: ContractCall < Self :: SendApi , < VecMapper < Self :: Storage , CallbackData < Self :: BigUint > > as elrond_wasm :: io :: EndpointResult > :: DecodeAs >{
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"callback_data"[..]),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn callback_data_at_index(
            self,
            index: usize,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <MultiResult5<
                BoxedBytes,
                TokenIdentifier,
                u64,
                Self::BigUint,
                MultiResultVec<BoxedBytes>,
            > as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"callback_data_at_index"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                index,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn clear_callback_data(
            self,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"clear_callback_data"[..]),
            );
            ___contract_call___
        }
    }
    pub struct CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        pub api: SA,
    }
    impl<SA> elrond_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        type BigUint = SA::AmountType;
        type BigInt = SA::ProxyBigInt;
        type EllipticCurve = SA::ProxyEllipticCurve;
        type Storage = SA::ProxyStorage;
        type SendApi = SA;
        type ErrorApi = SA;
        fn new_cb_proxy_obj(api: SA) -> Self {
            CallbackProxyObj { api }
        }
        fn into_api(self) -> Self::ErrorApi {
            self.api
        }
    }
    pub trait CallbackProxy: elrond_wasm::api::CallbackProxyObjApi + Sized {
        fn retrieve_funds_callback(self) -> elrond_wasm::types::CallbackCall {
            let ___api___ = self.into_api();
            let mut ___closure_arg_buffer___ = elrond_wasm::types::ArgBuffer::new();
            elrond_wasm::types::CallbackCall::from_arg_buffer(
                &b"retrieve_funds_callback"[..],
                &___closure_arg_buffer___,
            )
        }
        fn send_funds_twice_callback(
            self,
            to: &Address,
            token_identifier: &TokenIdentifier,
            cb_amount: &Self::BigUint,
        ) -> elrond_wasm::types::CallbackCall {
            let ___api___ = self.into_api();
            let mut ___closure_arg_buffer___ = elrond_wasm::types::ArgBuffer::new();
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                &mut ___closure_arg_buffer___,
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                &mut ___closure_arg_buffer___,
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                cb_amount,
                &mut ___closure_arg_buffer___,
                ___api___.clone(),
            );
            elrond_wasm::types::CallbackCall::from_arg_buffer(
                &b"send_funds_twice_callback"[..],
                &___closure_arg_buffer___,
            )
        }
    }
    impl<SA> self::CallbackProxy for CallbackProxyObj<SA> where SA: elrond_wasm::api::SendApi + 'static {}
}
mod call_sync {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use vault::ProxyTrait as _;
    pub trait ForwarderSyncCallModule: elrond_wasm::api::ContractBase + Sized
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn echo_arguments_sync(&self, to: Address, args: VarArgs<BoxedBytes>) {
            let half_gas = self.blockchain().get_gas_left() / 2;
            let result = self
                .vault_proxy()
                .contract(to)
                .echo_arguments(args)
                .with_gas_limit(half_gas)
                .execute_on_dest_context();
            self.execute_on_dest_context_result_event(result.as_slice());
        }
        fn echo_arguments_sync_range(
            &self,
            to: Address,
            start: usize,
            end: usize,
            args: VarArgs<BoxedBytes>,
        ) {
            let half_gas = self.blockchain().get_gas_left() / 2;
            let result = self
                .vault_proxy()
                .contract(to)
                .echo_arguments(args)
                .with_gas_limit(half_gas)
                .execute_on_dest_context_custom_range(|_, _| (start, end));
            self.execute_on_dest_context_result_event(result.as_slice());
        }
        fn echo_arguments_sync_twice(&self, to: Address, args: VarArgs<BoxedBytes>) {
            let one_third_gas = self.blockchain().get_gas_left() / 3;
            let result = self
                .vault_proxy()
                .contract(to.clone())
                .echo_arguments(args.clone())
                .with_gas_limit(one_third_gas)
                .execute_on_dest_context();
            self.execute_on_dest_context_result_event(result.as_slice());
            let result = self
                .vault_proxy()
                .contract(to)
                .echo_arguments(args)
                .with_gas_limit(one_third_gas)
                .execute_on_dest_context();
            self.execute_on_dest_context_result_event(result.as_slice());
        }
        fn forward_sync_accept_funds(
            &self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) {
            let half_gas = self.blockchain().get_gas_left() / 2;
            let result: MultiResult4<TokenIdentifier, BoxedBytes, Self::BigUint, u64> = self
                .vault_proxy()
                .contract(to)
                .accept_funds_echo_payment(token, payment, token_nonce)
                .with_gas_limit(half_gas)
                .execute_on_dest_context();
            let (token_identifier, token_type_str, token_payment, token_nonce) =
                result.into_tuple();
            self.accept_funds_sync_result_event(
                &token_identifier,
                token_type_str.as_slice(),
                &token_payment,
                token_nonce,
            );
        }
        fn forward_sync_accept_funds_then_read(
            &self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) -> usize {
            let _ = self
                .vault_proxy()
                .contract(to.clone())
                .with_nft_nonce(token_nonce)
                .accept_funds(token, payment)
                .execute_on_dest_context();
            self.vault_proxy()
                .contract(to)
                .call_counts(b"accept_funds")
                .execute_on_dest_context()
        }
        fn forward_sync_retrieve_funds(
            &self,
            to: Address,
            token: TokenIdentifier,
            token_nonce: u64,
            amount: Self::BigUint,
        ) {
            self.vault_proxy()
                .contract(to)
                .retrieve_funds(token, token_nonce, amount, OptionalArg::None)
                .execute_on_dest_context()
        }
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;
        fn execute_on_dest_context_result_event(&self, result: &[BoxedBytes]);
        fn accept_funds_sync_result_event(
            &self,
            token_identifier: &TokenIdentifier,
            token_type: &[u8],
            token_payment: &Self::BigUint,
            token_nonce: u64,
        );
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ForwarderSyncCallModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl,
    {
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi> {
            vault::Proxy::new_proxy_obj(self.send())
        }
        fn execute_on_dest_context_result_event(&self, result: &[BoxedBytes]) {
            let mut ___topic_buffer___ = elrond_wasm::types::ArgBuffer::new();
            ___topic_buffer___.push_argument_bytes(
                &[
                    101u8, 99u8, 104u8, 111u8, 95u8, 97u8, 114u8, 103u8, 117u8, 109u8, 101u8,
                    110u8, 116u8, 115u8, 95u8, 115u8, 121u8, 110u8, 99u8, 95u8, 114u8, 101u8,
                    115u8, 117u8, 108u8, 116u8,
                ][..],
            );
            let ___data_bytes___ =
                elrond_wasm::log_util::serialize_log_data(result, self.log_api_raw());
            self.log_api_raw()
                .write_event_log(&___topic_buffer___, ___data_bytes___.as_slice());
        }
        fn accept_funds_sync_result_event(
            &self,
            token_identifier: &TokenIdentifier,
            token_type: &[u8],
            token_payment: &Self::BigUint,
            token_nonce: u64,
        ) {
            let mut ___topic_buffer___ = elrond_wasm::types::ArgBuffer::new();
            ___topic_buffer___.push_argument_bytes(
                &[
                    97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8,
                    95u8, 115u8, 121u8, 110u8, 99u8, 95u8, 114u8, 101u8, 115u8, 117u8, 108u8,
                    116u8,
                ][..],
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_type,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_payment,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_nonce,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            self.log_api_raw().write_event_log(&___topic_buffer___, &[]);
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + ForwarderSyncCallModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_echo_arguments_sync(&self) {
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let to: Address =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"to"[..]));
            let args: VarArgs<BoxedBytes> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"args"[..]));
            ___arg_loader.assert_no_more_args();
            self.echo_arguments_sync(to, args);
        }
        #[inline]
        fn call_echo_arguments_sync_range(&self) {
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let to: Address =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"to"[..]));
            let start: usize =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"start"[..]));
            let end: usize =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"end"[..]));
            let args: VarArgs<BoxedBytes> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"args"[..]));
            ___arg_loader.assert_no_more_args();
            self.echo_arguments_sync_range(to, start, end, args);
        }
        #[inline]
        fn call_echo_arguments_sync_twice(&self) {
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let to: Address =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"to"[..]));
            let args: VarArgs<BoxedBytes> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"args"[..]));
            ___arg_loader.assert_no_more_args();
            self.echo_arguments_sync_twice(to, args);
        }
        #[inline]
        fn call_forward_sync_accept_funds(&self) {
            let (payment, token) = self.call_value().payment_token_pair();
            let token_nonce = self.call_value().esdt_token_nonce();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            self.forward_sync_accept_funds(to, token, payment, token_nonce);
        }
        #[inline]
        fn call_forward_sync_accept_funds_then_read(&self) {
            let (payment, token) = self.call_value().payment_token_pair();
            let token_nonce = self.call_value().esdt_token_nonce();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let result = self.forward_sync_accept_funds_then_read(to, token, payment, token_nonce);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_forward_sync_retrieve_funds(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 4i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let token = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"token"[..]),
            );
            let token_nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"token_nonce"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                3i32,
                ArgId::from(&b"amount"[..]),
            );
            self.forward_sync_retrieve_funds(to, token, token_nonce, amount);
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [101u8, 99u8, 104u8, 111u8, 95u8, 97u8, 114u8, 103u8, 117u8, 109u8, 101u8, 110u8, 116u8, 115u8, 95u8, 115u8, 121u8, 110u8, 99u8] =>
                {
                    self.call_echo_arguments_sync();
                    true
                }
                [101u8, 99u8, 104u8, 111u8, 95u8, 97u8, 114u8, 103u8, 117u8, 109u8, 101u8, 110u8, 116u8, 115u8, 95u8, 115u8, 121u8, 110u8, 99u8, 95u8, 114u8, 97u8, 110u8, 103u8, 101u8] =>
                {
                    self.call_echo_arguments_sync_range();
                    true
                }
                [101u8, 99u8, 104u8, 111u8, 95u8, 97u8, 114u8, 103u8, 117u8, 109u8, 101u8, 110u8, 116u8, 115u8, 95u8, 115u8, 121u8, 110u8, 99u8, 95u8, 116u8, 119u8, 105u8, 99u8, 101u8] =>
                {
                    self.call_echo_arguments_sync_twice();
                    true
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 115u8, 121u8, 110u8, 99u8, 95u8, 97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8] =>
                {
                    self.call_forward_sync_accept_funds();
                    true
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 115u8, 121u8, 110u8, 99u8, 95u8, 97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8, 95u8, 116u8, 104u8, 101u8, 110u8, 95u8, 114u8, 101u8, 97u8, 100u8] =>
                {
                    self.call_forward_sync_accept_funds_then_read();
                    true
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 115u8, 121u8, 110u8, 99u8, 95u8, 114u8, 101u8, 116u8, 114u8, 105u8, 101u8, 118u8, 101u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8] =>
                {
                    self.call_forward_sync_retrieve_funds();
                    true
                }
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {}
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ForwarderSyncCallModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "echo_arguments_sync",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<VarArgs<BoxedBytes>>("args");
            contract_abi.add_type_descriptions::<VarArgs<BoxedBytes>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "echo_arguments_sync_range",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<usize>("start");
            contract_abi.add_type_descriptions::<usize>();
            endpoint_abi.add_input::<usize>("end");
            contract_abi.add_type_descriptions::<usize>();
            endpoint_abi.add_input::<VarArgs<BoxedBytes>>("args");
            contract_abi.add_type_descriptions::<VarArgs<BoxedBytes>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "echo_arguments_sync_twice",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<VarArgs<BoxedBytes>>("args");
            contract_abi.add_type_descriptions::<VarArgs<BoxedBytes>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "forward_sync_accept_funds",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "forward_sync_accept_funds_then_read",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_output::<usize>(&[]);
            contract_abi.add_type_descriptions::<usize>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "forward_sync_retrieve_funds",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<u64>("token_nonce");
            contract_abi.add_type_descriptions::<u64>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
        #[allow(clippy::too_many_arguments)]
        fn echo_arguments_sync(
            self,
            to: Address,
            args: VarArgs<BoxedBytes>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"echo_arguments_sync"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                args,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn echo_arguments_sync_range(
            self,
            to: Address,
            start: usize,
            end: usize,
            args: VarArgs<BoxedBytes>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"echo_arguments_sync_range"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                start,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                end,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                args,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn echo_arguments_sync_twice(
            self,
            to: Address,
            args: VarArgs<BoxedBytes>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"echo_arguments_sync_twice"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                args,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn forward_sync_accept_funds(
            self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                token,
                payment,
                token_nonce,
                elrond_wasm::types::BoxedBytes::from(&b"forward_sync_accept_funds"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn forward_sync_accept_funds_then_read(
            self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <usize as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                token,
                payment,
                token_nonce,
                elrond_wasm::types::BoxedBytes::from(&b"forward_sync_accept_funds_then_read"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn forward_sync_retrieve_funds(
            self,
            to: Address,
            token: TokenIdentifier,
            token_nonce: u64,
            amount: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"forward_sync_retrieve_funds"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_nonce,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
}
mod call_transf_exec {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use vault::ProxyTrait as _;
    pub trait ForwarderTransferExecuteModule: elrond_wasm::api::ContractBase + Sized
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn forward_transf_exec_accept_funds(
            &self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) {
            self.vault_proxy()
                .contract(to)
                .accept_funds(token, payment)
                .with_nft_nonce(token_nonce)
                .transfer_execute();
        }
        fn forward_transf_exec_accept_funds_twice(
            &self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) {
            let half_payment = payment / Self::BigUint::from(2u32);
            let half_gas = self.blockchain().get_gas_left() / 2;
            self.vault_proxy()
                .contract(to.clone())
                .accept_funds(token.clone(), half_payment.clone())
                .with_nft_nonce(token_nonce)
                .with_gas_limit(half_gas)
                .transfer_execute();
            self.vault_proxy()
                .contract(to)
                .accept_funds(token, half_payment)
                .with_nft_nonce(token_nonce)
                .with_gas_limit(half_gas)
                .transfer_execute();
        }
        /// Test that the default gas provided to the transfer_execute call
        /// leaves enough in the transaction for finish to happen.
        fn forward_transf_exec_accept_funds_return_values(
            &self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) -> MultiResult4<u64, u64, Self::BigUint, TokenIdentifier> {
            let gas_left_before = self.blockchain().get_gas_left();
            self.vault_proxy()
                .contract(to)
                .accept_funds(token.clone(), payment)
                .with_nft_nonce(token_nonce)
                .transfer_execute();
            let gas_left_after = self.blockchain().get_gas_left();
            (
                gas_left_before,
                gas_left_after,
                Self::BigUint::zero(),
                token,
            )
                .into()
        }
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ForwarderTransferExecuteModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl,
    {
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi> {
            vault::Proxy::new_proxy_obj(self.send())
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + ForwarderTransferExecuteModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_forward_transf_exec_accept_funds(&self) {
            let (payment, token) = self.call_value().payment_token_pair();
            let token_nonce = self.call_value().esdt_token_nonce();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            self.forward_transf_exec_accept_funds(to, token, payment, token_nonce);
        }
        #[inline]
        fn call_forward_transf_exec_accept_funds_twice(&self) {
            let (payment, token) = self.call_value().payment_token_pair();
            let token_nonce = self.call_value().esdt_token_nonce();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            self.forward_transf_exec_accept_funds_twice(to, token, payment, token_nonce);
        }
        #[inline]
        fn call_forward_transf_exec_accept_funds_return_values(&self) {
            let (payment, token) = self.call_value().payment_token_pair();
            let token_nonce = self.call_value().esdt_token_nonce();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let result = self.forward_transf_exec_accept_funds_return_values(
                to,
                token,
                payment,
                token_nonce,
            );
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 95u8, 101u8, 120u8, 101u8, 99u8, 95u8, 97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8] =>
                {
                    self.call_forward_transf_exec_accept_funds();
                    true
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 95u8, 101u8, 120u8, 101u8, 99u8, 95u8, 97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8, 95u8, 116u8, 119u8, 105u8, 99u8, 101u8] =>
                {
                    self.call_forward_transf_exec_accept_funds_twice();
                    true
                }
                [102u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 95u8, 116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 95u8, 101u8, 120u8, 101u8, 99u8, 95u8, 97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 102u8, 117u8, 110u8, 100u8, 115u8, 95u8, 114u8, 101u8, 116u8, 117u8, 114u8, 110u8, 95u8, 118u8, 97u8, 108u8, 117u8, 101u8, 115u8] =>
                {
                    self.call_forward_transf_exec_accept_funds_return_values();
                    true
                }
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {}
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ForwarderTransferExecuteModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "forward_transf_exec_accept_funds",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "forward_transf_exec_accept_funds_twice",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[
                    "Test that the default gas provided to the transfer_execute call",
                    "leaves enough in the transaction for finish to happen.",
                ],
                name: "forward_transf_exec_accept_funds_return_values",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_output::<MultiResult4<u64, u64, Self::BigUint, TokenIdentifier>>(&[]);
            contract_abi
                .add_type_descriptions::<MultiResult4<u64, u64, Self::BigUint, TokenIdentifier>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
        #[allow(clippy::too_many_arguments)]
        fn forward_transf_exec_accept_funds(
            self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                token,
                payment,
                token_nonce,
                elrond_wasm::types::BoxedBytes::from(&b"forward_transf_exec_accept_funds"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn forward_transf_exec_accept_funds_twice(
            self,
            to: Address,
            token: TokenIdentifier,
            payment: Self::BigUint,
            token_nonce: u64,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                token,
                payment,
                token_nonce,
                elrond_wasm::types::BoxedBytes::from(
                    &b"forward_transf_exec_accept_funds_twice"[..],
                ),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]        fn forward_transf_exec_accept_funds_return_values (self , to : Address , token : TokenIdentifier , payment : Self :: BigUint , token_nonce : u64) -> elrond_wasm :: types :: ContractCall < Self :: SendApi , < MultiResult4 < u64 , u64 , Self :: BigUint , TokenIdentifier > as elrond_wasm :: io :: EndpointResult > :: DecodeAs >{
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                token,
                payment,
                token_nonce,
                elrond_wasm::types::BoxedBytes::from(
                    &b"forward_transf_exec_accept_funds_return_values"[..],
                ),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
}
mod contract_change_owner {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use vault::ProxyTrait as _;
    pub trait ChangeOwnerModule: elrond_wasm::api::ContractBase + Sized
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn change_owner(&self, child_sc_address: Address, new_owner: Address) -> Address {
            self.send()
                .change_owner_address(&child_sc_address, &new_owner);
            self.get_owner_of_vault_contract(child_sc_address)
        }
        fn get_owner_of_vault_contract(&self, address: Address) -> Address {
            self.vault_proxy()
                .contract(address)
                .get_owner_address()
                .execute_on_dest_context()
        }
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ChangeOwnerModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl,
    {
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi> {
            vault::Proxy::new_proxy_obj(self.send())
        }
    }
    pub trait EndpointWrappers: elrond_wasm::api::ContractPrivateApi + ChangeOwnerModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_change_owner(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 2i32);
            let child_sc_address = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"child_sc_address"[..]),
            );
            let new_owner = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"new_owner"[..]),
            );
            let result = self.change_owner(child_sc_address, new_owner);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [99u8, 104u8, 97u8, 110u8, 103u8, 101u8, 79u8, 119u8, 110u8, 101u8, 114u8, 65u8, 100u8, 100u8, 114u8, 101u8, 115u8, 115u8] =>
                {
                    self.call_change_owner();
                    true
                }
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {}
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ChangeOwnerModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "changeOwnerAddress",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("child_sc_address");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<Address>("new_owner");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_output::<Address>(&[]);
            contract_abi.add_type_descriptions::<Address>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
        #[allow(clippy::too_many_arguments)]
        fn change_owner(
            self,
            child_sc_address: Address,
            new_owner: Address,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <Address as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"changeOwnerAddress"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                child_sc_address,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                new_owner,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
}
mod contract_deploy {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use vault::ProxyTrait as _;
    pub trait DeployContractModule: elrond_wasm::api::ContractBase + Sized
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn deploy_contract(&self, code: BoxedBytes) -> SCResult<Address> {
            let deployed_contract_address = self.deploy_vault(&code).ok_or("Deploy failed")?;
            Ok(deployed_contract_address)
        }
        fn deploy_from_source(
            &self,
            source_contract_address: Address,
            arguments: VarArgs<BoxedBytes>,
        ) -> SCResult<Address> {
            self.send()
                .deploy_from_source_contract(
                    self.blockchain().get_gas_left(),
                    &Self::BigUint::zero(),
                    &source_contract_address,
                    CodeMetadata::DEFAULT,
                    &arguments.as_slice().into(),
                )
                .ok_or("Deploy from source contract failed")
                .into()
        }
        fn deploy_two_contracts(
            &self,
            code: BoxedBytes,
        ) -> SCResult<MultiResult2<Address, Address>> {
            let first_deployed_contract_address =
                self.deploy_vault(&code).ok_or("First deploy failed")?;
            let second_deployed_contract_address =
                self.deploy_vault(&code).ok_or("Second deploy failed")?;
            Ok((
                first_deployed_contract_address,
                second_deployed_contract_address,
            )
                .into())
        }
        fn deploy_vault(&self, code: &BoxedBytes) -> Option<Address> {
            self.vault_proxy()
                .init()
                .deploy_contract(code, CodeMetadata::DEFAULT)
        }
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> DeployContractModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl,
    {
        fn vault_proxy(&self) -> vault::Proxy<Self::SendApi> {
            vault::Proxy::new_proxy_obj(self.send())
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + DeployContractModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_deploy_contract(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let code = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"code"[..]),
            );
            let result = self.deploy_contract(code);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_deploy_from_source(&self) {
            self.call_value().check_not_payable();
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let source_contract_address: Address = elrond_wasm::load_dyn_arg(
                &mut ___arg_loader,
                ArgId::from(&b"source_contract_address"[..]),
            );
            let arguments: VarArgs<BoxedBytes> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"arguments"[..]));
            ___arg_loader.assert_no_more_args();
            let result = self.deploy_from_source(source_contract_address, arguments);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_deploy_two_contracts(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let code = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"code"[..]),
            );
            let result = self.deploy_two_contracts(code);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_deploy_vault(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let code = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"code"[..]),
            );
            let result = self.deploy_vault(&code);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [100u8, 101u8, 112u8, 108u8, 111u8, 121u8, 95u8, 99u8, 111u8, 110u8, 116u8, 114u8, 97u8, 99u8, 116u8] =>
                {
                    self.call_deploy_contract();
                    true
                }
                [100u8, 101u8, 112u8, 108u8, 111u8, 121u8, 70u8, 114u8, 111u8, 109u8, 83u8, 111u8, 117u8, 114u8, 99u8, 101u8] =>
                {
                    self.call_deploy_from_source();
                    true
                }
                [100u8, 101u8, 112u8, 108u8, 111u8, 121u8, 95u8, 116u8, 119u8, 111u8, 95u8, 99u8, 111u8, 110u8, 116u8, 114u8, 97u8, 99u8, 116u8, 115u8] =>
                {
                    self.call_deploy_two_contracts();
                    true
                }
                [100u8, 101u8, 112u8, 108u8, 111u8, 121u8, 95u8, 118u8, 97u8, 117u8, 108u8, 116u8] =>
                {
                    self.call_deploy_vault();
                    true
                }
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {}
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "DeployContractModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "deploy_contract",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<BoxedBytes>("code");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_output::<SCResult<Address>>(&[]);
            contract_abi.add_type_descriptions::<SCResult<Address>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "deployFromSource",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("source_contract_address");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<VarArgs<BoxedBytes>>("arguments");
            contract_abi.add_type_descriptions::<VarArgs<BoxedBytes>>();
            endpoint_abi.add_output::<SCResult<Address>>(&[]);
            contract_abi.add_type_descriptions::<SCResult<Address>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "deploy_two_contracts",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<BoxedBytes>("code");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_output::<SCResult<MultiResult2<Address, Address>>>(&[]);
            contract_abi.add_type_descriptions::<SCResult<MultiResult2<Address, Address>>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "deploy_vault",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&BoxedBytes>("code");
            contract_abi.add_type_descriptions::<&BoxedBytes>();
            endpoint_abi.add_output::<Option<Address>>(&[]);
            contract_abi.add_type_descriptions::<Option<Address>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
        #[allow(clippy::too_many_arguments)]
        fn deploy_contract(
            self,
            code: BoxedBytes,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <SCResult<Address> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"deploy_contract"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                code,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn deploy_from_source(
            self,
            source_contract_address: Address,
            arguments: VarArgs<BoxedBytes>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <SCResult<Address> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"deployFromSource"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                source_contract_address,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                arguments,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn deploy_two_contracts(
            self,
            code: BoxedBytes,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <SCResult<MultiResult2<Address, Address>> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"deploy_two_contracts"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                code,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn deploy_vault(
            self,
            code: &BoxedBytes,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <Option<Address> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"deploy_vault"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                code,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
}
mod contract_update {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    pub trait UpgradeContractModule: elrond_wasm::api::ContractBase + Sized
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn upgrade_child_contract(
            &self,
            child_sc_address: Address,
            new_code: BoxedBytes,
            arguments: VarArgs<BoxedBytes>,
        ) {
            self.upgrade(&child_sc_address, &new_code, &arguments.into_vec());
        }
        fn upgrade(
            &self,
            child_sc_address: &Address,
            new_code: &BoxedBytes,
            arguments: &[BoxedBytes],
        ) {
            self.send().upgrade_contract(
                child_sc_address,
                self.blockchain().get_gas_left(),
                &Self::BigUint::zero(),
                new_code,
                CodeMetadata::DEFAULT,
                &arguments.into(),
            );
        }
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> UpgradeContractModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl,
    {
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + UpgradeContractModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_upgrade_child_contract(&self) {
            self.call_value().check_not_payable();
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let child_sc_address: Address = elrond_wasm::load_dyn_arg(
                &mut ___arg_loader,
                ArgId::from(&b"child_sc_address"[..]),
            );
            let new_code: BoxedBytes =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"new_code"[..]));
            let arguments: VarArgs<BoxedBytes> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"arguments"[..]));
            ___arg_loader.assert_no_more_args();
            self.upgrade_child_contract(child_sc_address, new_code, arguments);
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [117u8, 112u8, 103u8, 114u8, 97u8, 100u8, 101u8, 67u8, 104u8, 105u8, 108u8, 100u8, 67u8, 111u8, 110u8, 116u8, 114u8, 97u8, 99u8, 116u8] =>
                {
                    self.call_upgrade_child_contract();
                    true
                }
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {}
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "UpgradeContractModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "upgradeChildContract",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("child_sc_address");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<BoxedBytes>("new_code");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<VarArgs<BoxedBytes>>("arguments");
            contract_abi.add_type_descriptions::<VarArgs<BoxedBytes>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
        #[allow(clippy::too_many_arguments)]
        fn upgrade_child_contract(
            self,
            child_sc_address: Address,
            new_code: BoxedBytes,
            arguments: VarArgs<BoxedBytes>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"upgradeChildContract"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                child_sc_address,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                new_code,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                arguments,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
}
mod esdt {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use super::storage;
    pub trait ForwarderEsdtModule:
        elrond_wasm::api::ContractBase + Sized + storage::ForwarderStorageModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn get_fungible_esdt_balance(&self, token_identifier: &TokenIdentifier) -> Self::BigUint {
            self.blockchain().get_esdt_balance(
                &self.blockchain().get_sc_address(),
                token_identifier,
                0,
            )
        }
        fn send_esdt(
            &self,
            to: &Address,
            token_id: TokenIdentifier,
            amount: &Self::BigUint,
            opt_data: OptionalArg<BoxedBytes>,
        ) {
            let data = match &opt_data {
                OptionalArg::Some(data) => data.as_slice(),
                OptionalArg::None => &[],
            };
            self.send().direct(to, &token_id, 0, amount, data);
        }
        fn send_esdt_twice(
            &self,
            to: &Address,
            token_id: TokenIdentifier,
            amount_first_time: &Self::BigUint,
            amount_second_time: &Self::BigUint,
            opt_data: OptionalArg<BoxedBytes>,
        ) {
            let data = match &opt_data {
                OptionalArg::Some(data) => data.as_slice(),
                OptionalArg::None => &[],
            };
            self.send()
                .direct(to, &token_id, 0, amount_first_time, data);
            self.send()
                .direct(to, &token_id, 0, amount_second_time, data);
        }
        fn issue_fungible_token(
            &self,
            issue_cost: Self::BigUint,
            token_display_name: BoxedBytes,
            token_ticker: BoxedBytes,
            initial_supply: Self::BigUint,
        ) -> AsyncCall<Self::SendApi> {
            let caller = self.blockchain().get_caller();
            ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
                .issue_fungible(
                    issue_cost,
                    &token_display_name,
                    &token_ticker,
                    &initial_supply,
                    FungibleTokenProperties {
                        num_decimals: 0,
                        can_freeze: true,
                        can_wipe: true,
                        can_pause: true,
                        can_mint: true,
                        can_burn: true,
                        can_change_owner: true,
                        can_upgrade: true,
                        can_add_special_roles: true,
                    },
                )
                .async_call()
                .with_callback(self.callbacks().esdt_issue_callback(&caller))
        }
        fn esdt_issue_callback(
            &self,
            caller: &Address,
            token_identifier: TokenIdentifier,
            returned_tokens: Self::BigUint,
            result: AsyncCallResult<()>,
        ) {
            match result {
                AsyncCallResult::Ok(()) => {
                    self.last_issued_token().set(&token_identifier);
                    self.last_error_message().clear();
                }
                AsyncCallResult::Err(message) => {
                    if token_identifier.is_egld() && returned_tokens > 0 {
                        self.send().direct_egld(caller, &returned_tokens, &[]);
                    }
                    self.last_error_message().set(&message.err_msg);
                }
            }
        }
        fn local_mint(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) {
            self.send().esdt_local_mint(&token_identifier, 0, &amount);
        }
        fn local_burn(&self, token_identifier: TokenIdentifier, amount: Self::BigUint) {
            self.send().esdt_local_burn(&token_identifier, 0, &amount);
        }
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ForwarderEsdtModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl + storage::ForwarderStorageModule,
    {
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi> {
            < self :: CallbackProxyObj < Self :: SendApi > as elrond_wasm :: api :: CallbackProxyObjApi > :: new_cb_proxy_obj (self . send ())
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + ForwarderEsdtModule + storage::EndpointWrappers
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_get_fungible_esdt_balance(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 1i32);
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let result = self.get_fungible_esdt_balance(&token_identifier);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_send_esdt(&self) {
            self.call_value().check_not_payable();
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let to: &Address =
                &elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"to"[..]));
            let token_id: TokenIdentifier =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"token_id"[..]));
            let amount: &Self::BigUint =
                &elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"amount"[..]));
            let opt_data: OptionalArg<BoxedBytes> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"opt_data"[..]));
            ___arg_loader.assert_no_more_args();
            self.send_esdt(&to, token_id, &amount, opt_data);
        }
        #[inline]
        fn call_send_esdt_twice(&self) {
            self.call_value().check_not_payable();
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let to: &Address =
                &elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"to"[..]));
            let token_id: TokenIdentifier =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"token_id"[..]));
            let amount_first_time: &Self::BigUint = &elrond_wasm::load_dyn_arg(
                &mut ___arg_loader,
                ArgId::from(&b"amount_first_time"[..]),
            );
            let amount_second_time: &Self::BigUint = &elrond_wasm::load_dyn_arg(
                &mut ___arg_loader,
                ArgId::from(&b"amount_second_time"[..]),
            );
            let opt_data: OptionalArg<BoxedBytes> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"opt_data"[..]));
            ___arg_loader.assert_no_more_args();
            self.send_esdt_twice(
                &to,
                token_id,
                &amount_first_time,
                &amount_second_time,
                opt_data,
            );
        }
        #[inline]
        fn call_issue_fungible_token(&self) {
            let issue_cost = self.call_value().require_egld();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 3i32);
            let token_display_name = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_display_name"[..]),
            );
            let token_ticker = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"token_ticker"[..]),
            );
            let initial_supply = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"initial_supply"[..]),
            );
            let result = self.issue_fungible_token(
                issue_cost,
                token_display_name,
                token_ticker,
                initial_supply,
            );
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_local_mint(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 2i32);
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"amount"[..]),
            );
            self.local_mint(token_identifier, amount);
        }
        #[inline]
        fn call_local_burn(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 2i32);
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"amount"[..]),
            );
            self.local_burn(token_identifier, amount);
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [103u8, 101u8, 116u8, 70u8, 117u8, 110u8, 103u8, 105u8, 98u8, 108u8, 101u8, 69u8, 115u8, 100u8, 116u8, 66u8, 97u8, 108u8, 97u8, 110u8, 99u8, 101u8] =>
                {
                    self.call_get_fungible_esdt_balance();
                    true
                }
                [115u8, 101u8, 110u8, 100u8, 95u8, 101u8, 115u8, 100u8, 116u8] => {
                    self.call_send_esdt();
                    true
                }
                [115u8, 101u8, 110u8, 100u8, 95u8, 101u8, 115u8, 100u8, 116u8, 95u8, 116u8, 119u8, 105u8, 99u8, 101u8] =>
                {
                    self.call_send_esdt_twice();
                    true
                }
                [105u8, 115u8, 115u8, 117u8, 101u8, 95u8, 102u8, 117u8, 110u8, 103u8, 105u8, 98u8, 108u8, 101u8, 95u8, 116u8, 111u8, 107u8, 101u8, 110u8] =>
                {
                    self.call_issue_fungible_token();
                    true
                }
                [108u8, 111u8, 99u8, 97u8, 108u8, 95u8, 109u8, 105u8, 110u8, 116u8] => {
                    self.call_local_mint();
                    true
                }
                [108u8, 111u8, 99u8, 97u8, 108u8, 95u8, 98u8, 117u8, 114u8, 110u8] => {
                    self.call_local_burn();
                    true
                }
                other => false,
            } {
                return true;
            }
            if storage::EndpointWrappers::call(self, fn_name) {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            let mut ___call_result_loader___ = EndpointDynArgLoader::new(self.argument_api());
            match ___cb_data_deserializer___.get_func_name() {
                [] => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                [101u8, 115u8, 100u8, 116u8, 95u8, 105u8, 115u8, 115u8, 117u8, 101u8, 95u8, 99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8] =>
                {
                    let (returned_tokens, token_identifier) =
                        self.call_value().payment_token_pair();
                    let mut ___cb_closure_loader___ =
                        CallDataArgLoader::new(___cb_data_deserializer___, self.error_api());
                    let caller: &Address = &elrond_wasm::load_dyn_arg(
                        &mut ___cb_closure_loader___,
                        ArgId::from(&b"caller"[..]),
                    );
                    let result: AsyncCallResult<()> = elrond_wasm::load_dyn_arg(
                        &mut ___call_result_loader___,
                        ArgId::from(&b"result"[..]),
                    );
                    ___cb_closure_loader___.assert_no_more_args();
                    ___call_result_loader___.assert_no_more_args();
                    self.esdt_issue_callback(&caller, token_identifier, returned_tokens, result);
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                _ => {}
            }
            match storage::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
                elrond_wasm::types::CallbackSelectorResult::Processed => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                    ___cb_data_deserializer___ = recovered_deser;
                }
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {
            let ___tx_hash___ = elrond_wasm::api::BlockchainApi::get_tx_hash(&self.blockchain());
            let ___cb_data_raw___ = elrond_wasm::api::StorageReadApi::storage_load_boxed_bytes(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
            );
            elrond_wasm::api::StorageWriteApi::storage_store_slice_u8(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
                &[],
            );
            let mut ___cb_data_deserializer___ =
                elrond_wasm::hex_call_data::HexCallDataDeserializer::new(
                    ___cb_data_raw___.as_slice(),
                );
            if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                self::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)
            {
                self.error_api().signal_error(err_msg::CALLBACK_BAD_FUNC);
            }
        }
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ForwarderEsdtModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "getFungibleEsdtBalance",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<&TokenIdentifier>();
            endpoint_abi.add_output::<Self::BigUint>(&[]);
            contract_abi.add_type_descriptions::<Self::BigUint>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "send_esdt",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&Address>("to");
            contract_abi.add_type_descriptions::<&Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token_id");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<&Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<&Self::BigUint>();
            endpoint_abi.add_input::<OptionalArg<BoxedBytes>>("opt_data");
            contract_abi.add_type_descriptions::<OptionalArg<BoxedBytes>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "send_esdt_twice",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&Address>("to");
            contract_abi.add_type_descriptions::<&Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token_id");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<&Self::BigUint>("amount_first_time");
            contract_abi.add_type_descriptions::<&Self::BigUint>();
            endpoint_abi.add_input::<&Self::BigUint>("amount_second_time");
            contract_abi.add_type_descriptions::<&Self::BigUint>();
            endpoint_abi.add_input::<OptionalArg<BoxedBytes>>("opt_data");
            contract_abi.add_type_descriptions::<OptionalArg<BoxedBytes>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "issue_fungible_token",
                only_owner: false,
                payable_in_tokens: &["EGLD"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<BoxedBytes>("token_display_name");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<BoxedBytes>("token_ticker");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<Self::BigUint>("initial_supply");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "local_mint",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "local_burn",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> storage::AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> storage::EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized + storage::ProxyTrait {
        #[allow(clippy::too_many_arguments)]
        fn get_fungible_esdt_balance(
            self,
            token_identifier: &TokenIdentifier,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <Self::BigUint as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"getFungibleEsdtBalance"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn send_esdt(
            self,
            to: &Address,
            token_id: TokenIdentifier,
            amount: &Self::BigUint,
            opt_data: OptionalArg<BoxedBytes>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"send_esdt"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_id,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                opt_data,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn send_esdt_twice(
            self,
            to: &Address,
            token_id: TokenIdentifier,
            amount_first_time: &Self::BigUint,
            amount_second_time: &Self::BigUint,
            opt_data: OptionalArg<BoxedBytes>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"send_esdt_twice"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_id,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount_first_time,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount_second_time,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                opt_data,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn issue_fungible_token(
            self,
            issue_cost: Self::BigUint,
            token_display_name: BoxedBytes,
            token_ticker: BoxedBytes,
            initial_supply: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                issue_cost,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"issue_fungible_token"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_display_name,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_ticker,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                initial_supply,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn local_mint(
            self,
            token_identifier: TokenIdentifier,
            amount: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"local_mint"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn local_burn(
            self,
            token_identifier: TokenIdentifier,
            amount: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"local_burn"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
    pub struct CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        pub api: SA,
    }
    impl<SA> elrond_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        type BigUint = SA::AmountType;
        type BigInt = SA::ProxyBigInt;
        type EllipticCurve = SA::ProxyEllipticCurve;
        type Storage = SA::ProxyStorage;
        type SendApi = SA;
        type ErrorApi = SA;
        fn new_cb_proxy_obj(api: SA) -> Self {
            CallbackProxyObj { api }
        }
        fn into_api(self) -> Self::ErrorApi {
            self.api
        }
    }
    pub trait CallbackProxy: elrond_wasm::api::CallbackProxyObjApi + Sized {
        fn esdt_issue_callback(self, caller: &Address) -> elrond_wasm::types::CallbackCall {
            let ___api___ = self.into_api();
            let mut ___closure_arg_buffer___ = elrond_wasm::types::ArgBuffer::new();
            elrond_wasm::io::serialize_contract_call_arg(
                caller,
                &mut ___closure_arg_buffer___,
                ___api___.clone(),
            );
            elrond_wasm::types::CallbackCall::from_arg_buffer(
                &b"esdt_issue_callback"[..],
                &___closure_arg_buffer___,
            )
        }
    }
    impl<SA> self::CallbackProxy for CallbackProxyObj<SA> where SA: elrond_wasm::api::SendApi + 'static {}
}
mod nft {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use elrond_wasm::elrond_codec;
    use elrond_wasm::elrond_codec::elrond_codec_derive::{
        NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode, TopEncodeOrDefault,
    };
    use elrond_wasm_derive::TypeAbi;
    use super::storage;
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    impl elrond_codec::TopEncode for Color {
        fn top_encode<O: elrond_codec::TopEncodeOutput>(
            &self,
            output: O,
        ) -> core::result::Result<(), elrond_codec::EncodeError> {
            let mut buffer = elrond_codec::Vec::<u8>::new();
            let dest = &mut buffer;
            elrond_codec::NestedEncode::dep_encode(&self.r, dest)?;
            elrond_codec::NestedEncode::dep_encode(&self.g, dest)?;
            elrond_codec::NestedEncode::dep_encode(&self.b, dest)?;
            output.set_slice_u8(&buffer[..]);
            core::result::Result::Ok(())
        }
        fn top_encode_or_exit<O: elrond_codec::TopEncodeOutput, ExitCtx: Clone>(
            &self,
            output: O,
            c: ExitCtx,
            exit: fn(ExitCtx, elrond_codec::EncodeError) -> !,
        ) {
            let mut buffer = elrond_codec::Vec::<u8>::new();
            let dest = &mut buffer;
            elrond_codec::NestedEncode::dep_encode_or_exit(&self.r, dest, c.clone(), exit);
            elrond_codec::NestedEncode::dep_encode_or_exit(&self.g, dest, c.clone(), exit);
            elrond_codec::NestedEncode::dep_encode_or_exit(&self.b, dest, c.clone(), exit);
            output.set_slice_u8(&buffer[..]);
        }
    }
    impl elrond_codec::TopDecode for Color {
        fn top_decode<I: elrond_codec::TopDecodeInput>(
            top_input: I,
        ) -> core::result::Result<Self, elrond_codec::DecodeError> {
            let bytes = top_input.into_boxed_slice_u8();
            let input = &mut &*bytes;
            let result = Color {
                r: <u8 as elrond_codec::NestedDecode>::dep_decode(input)?,
                g: <u8 as elrond_codec::NestedDecode>::dep_decode(input)?,
                b: <u8 as elrond_codec::NestedDecode>::dep_decode(input)?,
            };
            if !input.is_empty() {
                return core::result::Result::Err(elrond_codec::DecodeError::INPUT_TOO_LONG);
            }
            core::result::Result::Ok(result)
        }
        fn top_decode_or_exit<I: elrond_codec::TopDecodeInput, ExitCtx: Clone>(
            top_input: I,
            c: ExitCtx,
            exit: fn(ExitCtx, elrond_codec::DecodeError) -> !,
        ) -> Self {
            let bytes = top_input.into_boxed_slice_u8();
            let input = &mut &*bytes;
            let result = Color {
                r: <u8 as elrond_codec::NestedDecode>::dep_decode_or_exit(input, c.clone(), exit),
                g: <u8 as elrond_codec::NestedDecode>::dep_decode_or_exit(input, c.clone(), exit),
                b: <u8 as elrond_codec::NestedDecode>::dep_decode_or_exit(input, c.clone(), exit),
            };
            if !input.is_empty() {
                exit(c, elrond_codec::DecodeError::INPUT_TOO_LONG);
            }
            result
        }
    }
    impl elrond_wasm::abi::TypeAbi for Color {
        fn type_name() -> elrond_wasm::String {
            "Color".into()
        }
        fn provide_type_descriptions<TDC: elrond_wasm::abi::TypeDescriptionContainer>(
            accumulator: &mut TDC,
        ) {
            let type_name = Self::type_name();
            if !accumulator.contains_type(&type_name) {
                accumulator.reserve_type_name(type_name.clone());
                let mut field_descriptions = elrond_wasm::Vec::new();
                field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                    docs: &[],
                    name: "r",
                    field_type: <u8>::type_name(),
                });
                <u8>::provide_type_descriptions(accumulator);
                field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                    docs: &[],
                    name: "g",
                    field_type: <u8>::type_name(),
                });
                <u8>::provide_type_descriptions(accumulator);
                field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                    docs: &[],
                    name: "b",
                    field_type: <u8>::type_name(),
                });
                <u8>::provide_type_descriptions(accumulator);
                accumulator.insert(
                    type_name.clone(),
                    elrond_wasm::abi::TypeDescription {
                        docs: &[],
                        name: type_name,
                        contents: elrond_wasm::abi::TypeContents::Struct(field_descriptions),
                    },
                );
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub trait ForwarderNftModule:
        elrond_wasm::api::ContractBase + Sized + storage::ForwarderStorageModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn get_nft_balance(&self, token_identifier: &TokenIdentifier, nonce: u64) -> Self::BigUint {
            self.blockchain().get_esdt_balance(
                &self.blockchain().get_sc_address(),
                token_identifier,
                nonce,
            )
        }
        fn buy_nft(
            &self,
            nft_id: TokenIdentifier,
            nft_nonce: u64,
            nft_amount: Self::BigUint,
        ) -> Self::BigUint {
            let (payment_amount, payment_token) = self.call_value().payment_token_pair();
            let payment_nonce = self.call_value().esdt_token_nonce();
            self.send().sell_nft(
                &nft_id,
                nft_nonce,
                &nft_amount,
                &self.blockchain().get_caller(),
                &payment_token,
                payment_nonce,
                &payment_amount,
            )
        }
        fn nft_issue(
            &self,
            issue_cost: Self::BigUint,
            token_display_name: BoxedBytes,
            token_ticker: BoxedBytes,
        ) -> AsyncCall<Self::SendApi> {
            let caller = self.blockchain().get_caller();
            ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
                .issue_non_fungible(
                    issue_cost,
                    &token_display_name,
                    &token_ticker,
                    NonFungibleTokenProperties {
                        can_freeze: true,
                        can_wipe: true,
                        can_pause: true,
                        can_change_owner: true,
                        can_upgrade: true,
                        can_add_special_roles: true,
                    },
                )
                .async_call()
                .with_callback(self.callbacks().nft_issue_callback(&caller))
        }
        fn nft_issue_callback(&self, caller: &Address, result: AsyncCallResult<TokenIdentifier>) {
            match result {
                AsyncCallResult::Ok(token_identifier) => {
                    self.last_issued_token().set(&token_identifier);
                    self.last_error_message().clear();
                }
                AsyncCallResult::Err(message) => {
                    let (returned_tokens, token_identifier) =
                        self.call_value().payment_token_pair();
                    if token_identifier.is_egld() && returned_tokens > 0 {
                        self.send().direct_egld(caller, &returned_tokens, &[]);
                    }
                    self.last_error_message().set(&message.err_msg);
                }
            }
        }
        #[allow(clippy::too_many_arguments)]
        fn nft_create(
            &self,
            token_identifier: TokenIdentifier,
            amount: Self::BigUint,
            name: BoxedBytes,
            royalties: Self::BigUint,
            hash: BoxedBytes,
            color: Color,
            uri: BoxedBytes,
        ) -> u64 {
            let token_nonce = self.send().esdt_nft_create::<Color>(
                &token_identifier,
                &amount,
                &name,
                &royalties,
                &hash,
                &color,
                &[uri],
            );
            self.create_event(&token_identifier, token_nonce, &amount);
            token_nonce
        }
        fn nft_add_quantity(
            &self,
            token_identifier: TokenIdentifier,
            nonce: u64,
            amount: Self::BigUint,
        ) {
            self.send()
                .esdt_local_mint(&token_identifier, nonce, &amount);
        }
        fn nft_burn(&self, token_identifier: TokenIdentifier, nonce: u64, amount: Self::BigUint) {
            self.send()
                .esdt_local_burn(&token_identifier, nonce, &amount);
        }
        fn transfer_nft_via_async_call(
            &self,
            to: Address,
            token_identifier: TokenIdentifier,
            nonce: u64,
            amount: Self::BigUint,
            data: BoxedBytes,
        ) {
            self.send().transfer_esdt_via_async_call(
                &to,
                &token_identifier,
                nonce,
                &amount,
                data.as_slice(),
            );
        }
        fn transfer_nft_and_execute(
            &self,
            to: Address,
            token_identifier: TokenIdentifier,
            nonce: u64,
            amount: Self::BigUint,
            function: BoxedBytes,
            arguments: VarArgs<BoxedBytes>,
        ) {
            let mut arg_buffer = ArgBuffer::new();
            for arg in arguments.into_vec() {
                arg_buffer.push_argument_bytes(arg.as_slice());
            }
            let _ = self.send().direct_esdt_nft_execute(
                &to,
                &token_identifier,
                nonce,
                &amount,
                self.blockchain().get_gas_left(),
                function.as_slice(),
                &arg_buffer,
            );
        }
        fn create_and_send(
            &self,
            to: Address,
            token_identifier: TokenIdentifier,
            amount: Self::BigUint,
            name: BoxedBytes,
            royalties: Self::BigUint,
            hash: BoxedBytes,
            color: Color,
            uri: BoxedBytes,
        ) {
            let token_nonce = self.nft_create(
                token_identifier.clone(),
                amount.clone(),
                name,
                royalties,
                hash,
                color,
                uri,
            );
            self.send().direct(
                &to,
                &token_identifier,
                token_nonce,
                &amount,
                b"NFT transfer",
            );
            self.send_event(&to, &token_identifier, token_nonce, &amount);
        }
        fn create_event(
            &self,
            token_id: &TokenIdentifier,
            token_nonce: u64,
            amount: &Self::BigUint,
        );
        fn send_event(
            &self,
            to: &Address,
            token_id: &TokenIdentifier,
            token_nonce: u64,
            amount: &Self::BigUint,
        );
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ForwarderNftModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl + storage::ForwarderStorageModule,
    {
        fn create_event(
            &self,
            token_id: &TokenIdentifier,
            token_nonce: u64,
            amount: &Self::BigUint,
        ) {
            let mut ___topic_buffer___ = elrond_wasm::types::ArgBuffer::new();
            ___topic_buffer___.push_argument_bytes(&[99u8, 114u8, 101u8, 97u8, 116u8, 101u8][..]);
            elrond_wasm::io::serialize_contract_call_arg(
                token_id,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_nonce,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            self.log_api_raw().write_event_log(&___topic_buffer___, &[]);
        }
        fn send_event(
            &self,
            to: &Address,
            token_id: &TokenIdentifier,
            token_nonce: u64,
            amount: &Self::BigUint,
        ) {
            let mut ___topic_buffer___ = elrond_wasm::types::ArgBuffer::new();
            ___topic_buffer___.push_argument_bytes(&[115u8, 101u8, 110u8, 100u8][..]);
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_id,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_nonce,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                &mut ___topic_buffer___,
                self.log_api_raw(),
            );
            self.log_api_raw().write_event_log(&___topic_buffer___, &[]);
        }
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi> {
            < self :: CallbackProxyObj < Self :: SendApi > as elrond_wasm :: api :: CallbackProxyObjApi > :: new_cb_proxy_obj (self . send ())
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + ForwarderNftModule + storage::EndpointWrappers
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_get_nft_balance(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 2i32);
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"nonce"[..]),
            );
            let result = self.get_nft_balance(&token_identifier, nonce);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_buy_nft(&self) {
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 3i32);
            let nft_id = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"nft_id"[..]),
            );
            let nft_nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"nft_nonce"[..]),
            );
            let nft_amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"nft_amount"[..]),
            );
            let result = self.buy_nft(nft_id, nft_nonce, nft_amount);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_nft_issue(&self) {
            let issue_cost = self.call_value().require_egld();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 2i32);
            let token_display_name = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_display_name"[..]),
            );
            let token_ticker = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"token_ticker"[..]),
            );
            let result = self.nft_issue(issue_cost, token_display_name, token_ticker);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_nft_create(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 7i32);
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"amount"[..]),
            );
            let name = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"name"[..]),
            );
            let royalties = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                3i32,
                ArgId::from(&b"royalties"[..]),
            );
            let hash = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                4i32,
                ArgId::from(&b"hash"[..]),
            );
            let color = elrond_wasm::load_single_arg::<Self::ArgumentApi, Color>(
                self.argument_api(),
                5i32,
                ArgId::from(&b"color"[..]),
            );
            let uri = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                6i32,
                ArgId::from(&b"uri"[..]),
            );
            let result =
                self.nft_create(token_identifier, amount, name, royalties, hash, color, uri);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_nft_add_quantity(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 3i32);
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"nonce"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"amount"[..]),
            );
            self.nft_add_quantity(token_identifier, nonce, amount);
        }
        #[inline]
        fn call_nft_burn(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 3i32);
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"nonce"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"amount"[..]),
            );
            self.nft_burn(token_identifier, nonce, amount);
        }
        #[inline]
        fn call_transfer_nft_via_async_call(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 5i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"nonce"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                3i32,
                ArgId::from(&b"amount"[..]),
            );
            let data = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                4i32,
                ArgId::from(&b"data"[..]),
            );
            self.transfer_nft_via_async_call(to, token_identifier, nonce, amount, data);
        }
        #[inline]
        fn call_transfer_nft_and_execute(&self) {
            self.call_value().check_not_payable();
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let to: Address =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"to"[..]));
            let token_identifier: TokenIdentifier = elrond_wasm::load_dyn_arg(
                &mut ___arg_loader,
                ArgId::from(&b"token_identifier"[..]),
            );
            let nonce: u64 =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"nonce"[..]));
            let amount: Self::BigUint =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"amount"[..]));
            let function: BoxedBytes =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"function"[..]));
            let arguments: VarArgs<BoxedBytes> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"arguments"[..]));
            ___arg_loader.assert_no_more_args();
            self.transfer_nft_and_execute(to, token_identifier, nonce, amount, function, arguments);
        }
        #[inline]
        fn call_create_and_send(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 8i32);
            let to = elrond_wasm::load_single_arg::<Self::ArgumentApi, Address>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"to"[..]),
            );
            let token_identifier = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"token_identifier"[..]),
            );
            let amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                2i32,
                ArgId::from(&b"amount"[..]),
            );
            let name = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                3i32,
                ArgId::from(&b"name"[..]),
            );
            let royalties = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
                self.argument_api(),
                4i32,
                ArgId::from(&b"royalties"[..]),
            );
            let hash = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                5i32,
                ArgId::from(&b"hash"[..]),
            );
            let color = elrond_wasm::load_single_arg::<Self::ArgumentApi, Color>(
                self.argument_api(),
                6i32,
                ArgId::from(&b"color"[..]),
            );
            let uri = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                7i32,
                ArgId::from(&b"uri"[..]),
            );
            self.create_and_send(
                to,
                token_identifier,
                amount,
                name,
                royalties,
                hash,
                color,
                uri,
            );
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [103u8, 101u8, 116u8, 95u8, 110u8, 102u8, 116u8, 95u8, 98u8, 97u8, 108u8, 97u8, 110u8, 99u8, 101u8] =>
                {
                    self.call_get_nft_balance();
                    true
                }
                [98u8, 117u8, 121u8, 95u8, 110u8, 102u8, 116u8] => {
                    self.call_buy_nft();
                    true
                }
                [110u8, 102u8, 116u8, 95u8, 105u8, 115u8, 115u8, 117u8, 101u8] => {
                    self.call_nft_issue();
                    true
                }
                [110u8, 102u8, 116u8, 95u8, 99u8, 114u8, 101u8, 97u8, 116u8, 101u8] => {
                    self.call_nft_create();
                    true
                }
                [110u8, 102u8, 116u8, 95u8, 97u8, 100u8, 100u8, 95u8, 113u8, 117u8, 97u8, 110u8, 116u8, 105u8, 116u8, 121u8] =>
                {
                    self.call_nft_add_quantity();
                    true
                }
                [110u8, 102u8, 116u8, 95u8, 98u8, 117u8, 114u8, 110u8] => {
                    self.call_nft_burn();
                    true
                }
                [116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 101u8, 114u8, 95u8, 110u8, 102u8, 116u8, 95u8, 118u8, 105u8, 97u8, 95u8, 97u8, 115u8, 121u8, 110u8, 99u8, 95u8, 99u8, 97u8, 108u8, 108u8] =>
                {
                    self.call_transfer_nft_via_async_call();
                    true
                }
                [116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 101u8, 114u8, 95u8, 110u8, 102u8, 116u8, 95u8, 97u8, 110u8, 100u8, 95u8, 101u8, 120u8, 101u8, 99u8, 117u8, 116u8, 101u8] =>
                {
                    self.call_transfer_nft_and_execute();
                    true
                }
                [99u8, 114u8, 101u8, 97u8, 116u8, 101u8, 95u8, 97u8, 110u8, 100u8, 95u8, 115u8, 101u8, 110u8, 100u8] =>
                {
                    self.call_create_and_send();
                    true
                }
                other => false,
            } {
                return true;
            }
            if storage::EndpointWrappers::call(self, fn_name) {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            let mut ___call_result_loader___ = EndpointDynArgLoader::new(self.argument_api());
            match ___cb_data_deserializer___.get_func_name() {
                [] => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                [110u8, 102u8, 116u8, 95u8, 105u8, 115u8, 115u8, 117u8, 101u8, 95u8, 99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8] =>
                {
                    let mut ___cb_closure_loader___ =
                        CallDataArgLoader::new(___cb_data_deserializer___, self.error_api());
                    let caller: &Address = &elrond_wasm::load_dyn_arg(
                        &mut ___cb_closure_loader___,
                        ArgId::from(&b"caller"[..]),
                    );
                    let result: AsyncCallResult<TokenIdentifier> = elrond_wasm::load_dyn_arg(
                        &mut ___call_result_loader___,
                        ArgId::from(&b"result"[..]),
                    );
                    ___cb_closure_loader___.assert_no_more_args();
                    ___call_result_loader___.assert_no_more_args();
                    self.nft_issue_callback(&caller, result);
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                _ => {}
            }
            match storage::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
                elrond_wasm::types::CallbackSelectorResult::Processed => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                    ___cb_data_deserializer___ = recovered_deser;
                }
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {
            let ___tx_hash___ = elrond_wasm::api::BlockchainApi::get_tx_hash(&self.blockchain());
            let ___cb_data_raw___ = elrond_wasm::api::StorageReadApi::storage_load_boxed_bytes(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
            );
            elrond_wasm::api::StorageWriteApi::storage_store_slice_u8(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
                &[],
            );
            let mut ___cb_data_deserializer___ =
                elrond_wasm::hex_call_data::HexCallDataDeserializer::new(
                    ___cb_data_raw___.as_slice(),
                );
            if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                self::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)
            {
                self.error_api().signal_error(err_msg::CALLBACK_BAD_FUNC);
            }
        }
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ForwarderNftModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "get_nft_balance",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<&TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<&TokenIdentifier>();
            endpoint_abi.add_input::<u64>("nonce");
            contract_abi.add_type_descriptions::<u64>();
            endpoint_abi.add_output::<Self::BigUint>(&[]);
            contract_abi.add_type_descriptions::<Self::BigUint>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "buy_nft",
                only_owner: false,
                payable_in_tokens: &["*"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<TokenIdentifier>("nft_id");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<u64>("nft_nonce");
            contract_abi.add_type_descriptions::<u64>();
            endpoint_abi.add_input::<Self::BigUint>("nft_amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_output::<Self::BigUint>(&[]);
            contract_abi.add_type_descriptions::<Self::BigUint>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "nft_issue",
                only_owner: false,
                payable_in_tokens: &["EGLD"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<BoxedBytes>("token_display_name");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<BoxedBytes>("token_ticker");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "nft_create",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_input::<BoxedBytes>("name");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<Self::BigUint>("royalties");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_input::<BoxedBytes>("hash");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<Color>("color");
            contract_abi.add_type_descriptions::<Color>();
            endpoint_abi.add_input::<BoxedBytes>("uri");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_output::<u64>(&[]);
            contract_abi.add_type_descriptions::<u64>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "nft_add_quantity",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<u64>("nonce");
            contract_abi.add_type_descriptions::<u64>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "nft_burn",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<u64>("nonce");
            contract_abi.add_type_descriptions::<u64>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "transfer_nft_via_async_call",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<u64>("nonce");
            contract_abi.add_type_descriptions::<u64>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_input::<BoxedBytes>("data");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "transfer_nft_and_execute",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<u64>("nonce");
            contract_abi.add_type_descriptions::<u64>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_input::<BoxedBytes>("function");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<VarArgs<BoxedBytes>>("arguments");
            contract_abi.add_type_descriptions::<VarArgs<BoxedBytes>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "create_and_send",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("to");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<Self::BigUint>("amount");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_input::<BoxedBytes>("name");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<Self::BigUint>("royalties");
            contract_abi.add_type_descriptions::<Self::BigUint>();
            endpoint_abi.add_input::<BoxedBytes>("hash");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<Color>("color");
            contract_abi.add_type_descriptions::<Color>();
            endpoint_abi.add_input::<BoxedBytes>("uri");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> storage::AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> storage::EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized + storage::ProxyTrait {
        #[allow(clippy::too_many_arguments)]
        fn get_nft_balance(
            self,
            token_identifier: &TokenIdentifier,
            nonce: u64,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <Self::BigUint as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"get_nft_balance"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nonce,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn buy_nft(
            self,
            nft_id: TokenIdentifier,
            nft_nonce: u64,
            nft_amount: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <Self::BigUint as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"buy_nft"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nft_id,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nft_nonce,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nft_amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn nft_issue(
            self,
            issue_cost: Self::BigUint,
            token_display_name: BoxedBytes,
            token_ticker: BoxedBytes,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                issue_cost,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"nft_issue"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_display_name,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_ticker,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn nft_create(
            self,
            token_identifier: TokenIdentifier,
            amount: Self::BigUint,
            name: BoxedBytes,
            royalties: Self::BigUint,
            hash: BoxedBytes,
            color: Color,
            uri: BoxedBytes,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <u64 as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"nft_create"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                name,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                royalties,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                hash,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                color,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                uri,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn nft_add_quantity(
            self,
            token_identifier: TokenIdentifier,
            nonce: u64,
            amount: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"nft_add_quantity"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nonce,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn nft_burn(
            self,
            token_identifier: TokenIdentifier,
            nonce: u64,
            amount: Self::BigUint,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"nft_burn"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nonce,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn transfer_nft_via_async_call(
            self,
            to: Address,
            token_identifier: TokenIdentifier,
            nonce: u64,
            amount: Self::BigUint,
            data: BoxedBytes,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"transfer_nft_via_async_call"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nonce,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                data,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn transfer_nft_and_execute(
            self,
            to: Address,
            token_identifier: TokenIdentifier,
            nonce: u64,
            amount: Self::BigUint,
            function: BoxedBytes,
            arguments: VarArgs<BoxedBytes>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"transfer_nft_and_execute"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                nonce,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                function,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                arguments,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn create_and_send(
            self,
            to: Address,
            token_identifier: TokenIdentifier,
            amount: Self::BigUint,
            name: BoxedBytes,
            royalties: Self::BigUint,
            hash: BoxedBytes,
            color: Color,
            uri: BoxedBytes,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <() as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"create_and_send"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                to,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                amount,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                name,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                royalties,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                hash,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                color,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                uri,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
    pub struct CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        pub api: SA,
    }
    impl<SA> elrond_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        type BigUint = SA::AmountType;
        type BigInt = SA::ProxyBigInt;
        type EllipticCurve = SA::ProxyEllipticCurve;
        type Storage = SA::ProxyStorage;
        type SendApi = SA;
        type ErrorApi = SA;
        fn new_cb_proxy_obj(api: SA) -> Self {
            CallbackProxyObj { api }
        }
        fn into_api(self) -> Self::ErrorApi {
            self.api
        }
    }
    pub trait CallbackProxy: elrond_wasm::api::CallbackProxyObjApi + Sized {
        fn nft_issue_callback(self, caller: &Address) -> elrond_wasm::types::CallbackCall {
            let ___api___ = self.into_api();
            let mut ___closure_arg_buffer___ = elrond_wasm::types::ArgBuffer::new();
            elrond_wasm::io::serialize_contract_call_arg(
                caller,
                &mut ___closure_arg_buffer___,
                ___api___.clone(),
            );
            elrond_wasm::types::CallbackCall::from_arg_buffer(
                &b"nft_issue_callback"[..],
                &___closure_arg_buffer___,
            )
        }
    }
    impl<SA> self::CallbackProxy for CallbackProxyObj<SA> where SA: elrond_wasm::api::SendApi + 'static {}
}
mod roles {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use super::storage;
    pub trait ForwarderRolesModule:
        elrond_wasm::api::ContractBase + Sized + storage::ForwarderStorageModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn set_local_roles(
            &self,
            address: Address,
            token_identifier: TokenIdentifier,
            roles: VarArgs<EsdtLocalRole>,
        ) -> AsyncCall<Self::SendApi> {
            ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
                .set_special_roles(&address, &token_identifier, roles.as_slice())
                .async_call()
                .with_callback(self.callbacks().change_roles_callback())
        }
        fn unset_local_roles(
            &self,
            address: Address,
            token_identifier: TokenIdentifier,
            roles: VarArgs<EsdtLocalRole>,
        ) -> AsyncCall<Self::SendApi> {
            ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
                .unset_special_roles(&address, &token_identifier, roles.as_slice())
                .async_call()
                .with_callback(self.callbacks().change_roles_callback())
        }
        fn change_roles_callback(&self, result: AsyncCallResult<()>) {
            match result {
                AsyncCallResult::Ok(()) => {
                    self.last_error_message().clear();
                }
                AsyncCallResult::Err(message) => {
                    self.last_error_message().set(&message.err_msg);
                }
            }
        }
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ForwarderRolesModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl + storage::ForwarderStorageModule,
    {
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi> {
            < self :: CallbackProxyObj < Self :: SendApi > as elrond_wasm :: api :: CallbackProxyObjApi > :: new_cb_proxy_obj (self . send ())
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + ForwarderRolesModule + storage::EndpointWrappers
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_set_local_roles(&self) {
            self.call_value().check_not_payable();
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let address: Address =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"address"[..]));
            let token_identifier: TokenIdentifier = elrond_wasm::load_dyn_arg(
                &mut ___arg_loader,
                ArgId::from(&b"token_identifier"[..]),
            );
            let roles: VarArgs<EsdtLocalRole> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"roles"[..]));
            ___arg_loader.assert_no_more_args();
            let result = self.set_local_roles(address, token_identifier, roles);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_unset_local_roles(&self) {
            self.call_value().check_not_payable();
            let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
            let address: Address =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"address"[..]));
            let token_identifier: TokenIdentifier = elrond_wasm::load_dyn_arg(
                &mut ___arg_loader,
                ArgId::from(&b"token_identifier"[..]),
            );
            let roles: VarArgs<EsdtLocalRole> =
                elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"roles"[..]));
            ___arg_loader.assert_no_more_args();
            let result = self.unset_local_roles(address, token_identifier, roles);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [115u8, 101u8, 116u8, 76u8, 111u8, 99u8, 97u8, 108u8, 82u8, 111u8, 108u8, 101u8, 115u8] =>
                {
                    self.call_set_local_roles();
                    true
                }
                [117u8, 110u8, 115u8, 101u8, 116u8, 76u8, 111u8, 99u8, 97u8, 108u8, 82u8, 111u8, 108u8, 101u8, 115u8] =>
                {
                    self.call_unset_local_roles();
                    true
                }
                other => false,
            } {
                return true;
            }
            if storage::EndpointWrappers::call(self, fn_name) {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            let mut ___call_result_loader___ = EndpointDynArgLoader::new(self.argument_api());
            match ___cb_data_deserializer___.get_func_name() {
                [] => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                [99u8, 104u8, 97u8, 110u8, 103u8, 101u8, 95u8, 114u8, 111u8, 108u8, 101u8, 115u8, 95u8, 99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8] =>
                {
                    let mut ___cb_closure_loader___ =
                        CallDataArgLoader::new(___cb_data_deserializer___, self.error_api());
                    let result: AsyncCallResult<()> = elrond_wasm::load_dyn_arg(
                        &mut ___call_result_loader___,
                        ArgId::from(&b"result"[..]),
                    );
                    ___cb_closure_loader___.assert_no_more_args();
                    ___call_result_loader___.assert_no_more_args();
                    self.change_roles_callback(result);
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                _ => {}
            }
            match storage::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
                elrond_wasm::types::CallbackSelectorResult::Processed => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                    ___cb_data_deserializer___ = recovered_deser;
                }
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {
            let ___tx_hash___ = elrond_wasm::api::BlockchainApi::get_tx_hash(&self.blockchain());
            let ___cb_data_raw___ = elrond_wasm::api::StorageReadApi::storage_load_boxed_bytes(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
            );
            elrond_wasm::api::StorageWriteApi::storage_store_slice_u8(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
                &[],
            );
            let mut ___cb_data_deserializer___ =
                elrond_wasm::hex_call_data::HexCallDataDeserializer::new(
                    ___cb_data_raw___.as_slice(),
                );
            if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                self::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)
            {
                self.error_api().signal_error(err_msg::CALLBACK_BAD_FUNC);
            }
        }
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ForwarderRolesModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "setLocalRoles",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("address");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<VarArgs<EsdtLocalRole>>("roles");
            contract_abi.add_type_descriptions::<VarArgs<EsdtLocalRole>>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "unsetLocalRoles",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<Address>("address");
            contract_abi.add_type_descriptions::<Address>();
            endpoint_abi.add_input::<TokenIdentifier>("token_identifier");
            contract_abi.add_type_descriptions::<TokenIdentifier>();
            endpoint_abi.add_input::<VarArgs<EsdtLocalRole>>("roles");
            contract_abi.add_type_descriptions::<VarArgs<EsdtLocalRole>>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> storage::AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> storage::EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized + storage::ProxyTrait {
        #[allow(clippy::too_many_arguments)]
        fn set_local_roles(
            self,
            address: Address,
            token_identifier: TokenIdentifier,
            roles: VarArgs<EsdtLocalRole>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"setLocalRoles"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                address,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                roles,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]
        fn unset_local_roles(
            self,
            address: Address,
            token_identifier: TokenIdentifier,
            roles: VarArgs<EsdtLocalRole>,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"unsetLocalRoles"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                address,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_identifier,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                roles,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
    pub struct CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        pub api: SA,
    }
    impl<SA> elrond_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        type BigUint = SA::AmountType;
        type BigInt = SA::ProxyBigInt;
        type EllipticCurve = SA::ProxyEllipticCurve;
        type Storage = SA::ProxyStorage;
        type SendApi = SA;
        type ErrorApi = SA;
        fn new_cb_proxy_obj(api: SA) -> Self {
            CallbackProxyObj { api }
        }
        fn into_api(self) -> Self::ErrorApi {
            self.api
        }
    }
    pub trait CallbackProxy: elrond_wasm::api::CallbackProxyObjApi + Sized {
        fn change_roles_callback(self) -> elrond_wasm::types::CallbackCall {
            let ___api___ = self.into_api();
            let mut ___closure_arg_buffer___ = elrond_wasm::types::ArgBuffer::new();
            elrond_wasm::types::CallbackCall::from_arg_buffer(
                &b"change_roles_callback"[..],
                &___closure_arg_buffer___,
            )
        }
    }
    impl<SA> self::CallbackProxy for CallbackProxyObj<SA> where SA: elrond_wasm::api::SendApi + 'static {}
}
mod sft {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    use super::storage;
    pub trait ForwarderSftModule:
        elrond_wasm::api::ContractBase + Sized + storage::ForwarderStorageModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn sft_issue(
            &self,
            issue_cost: Self::BigUint,
            token_display_name: BoxedBytes,
            token_ticker: BoxedBytes,
        ) -> AsyncCall<Self::SendApi> {
            let caller = self.blockchain().get_caller();
            ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
                .issue_semi_fungible(
                    issue_cost,
                    &token_display_name,
                    &token_ticker,
                    SemiFungibleTokenProperties {
                        can_freeze: true,
                        can_wipe: true,
                        can_pause: true,
                        can_change_owner: true,
                        can_upgrade: true,
                        can_add_special_roles: true,
                    },
                )
                .async_call()
                .with_callback(self.callbacks().sft_issue_callback(&caller))
        }
        fn sft_issue_callback(&self, caller: &Address, result: AsyncCallResult<TokenIdentifier>) {
            match result {
                AsyncCallResult::Ok(token_identifier) => {
                    self.last_issued_token().set(&token_identifier);
                    self.last_error_message().clear();
                }
                AsyncCallResult::Err(message) => {
                    let (returned_tokens, token_identifier) =
                        self.call_value().payment_token_pair();
                    if token_identifier.is_egld() && returned_tokens > 0 {
                        self.send().direct_egld(caller, &returned_tokens, &[]);
                    }
                    self.last_error_message().set(&message.err_msg);
                }
            }
        }
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ForwarderSftModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl + storage::ForwarderStorageModule,
    {
        fn callbacks(&self) -> self::CallbackProxyObj<Self::SendApi> {
            < self :: CallbackProxyObj < Self :: SendApi > as elrond_wasm :: api :: CallbackProxyObjApi > :: new_cb_proxy_obj (self . send ())
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + ForwarderSftModule + storage::EndpointWrappers
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_sft_issue(&self) {
            let issue_cost = self.call_value().require_egld();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 2i32);
            let token_display_name = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                0i32,
                ArgId::from(&b"token_display_name"[..]),
            );
            let token_ticker = elrond_wasm::load_single_arg::<Self::ArgumentApi, BoxedBytes>(
                self.argument_api(),
                1i32,
                ArgId::from(&b"token_ticker"[..]),
            );
            let result = self.sft_issue(issue_cost, token_display_name, token_ticker);
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [115u8, 102u8, 116u8, 95u8, 105u8, 115u8, 115u8, 117u8, 101u8] => {
                    self.call_sft_issue();
                    true
                }
                other => false,
            } {
                return true;
            }
            if storage::EndpointWrappers::call(self, fn_name) {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            let mut ___call_result_loader___ = EndpointDynArgLoader::new(self.argument_api());
            match ___cb_data_deserializer___.get_func_name() {
                [] => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                [115u8, 102u8, 116u8, 95u8, 105u8, 115u8, 115u8, 117u8, 101u8, 95u8, 99u8, 97u8, 108u8, 108u8, 98u8, 97u8, 99u8, 107u8] =>
                {
                    let mut ___cb_closure_loader___ =
                        CallDataArgLoader::new(___cb_data_deserializer___, self.error_api());
                    let caller: &Address = &elrond_wasm::load_dyn_arg(
                        &mut ___cb_closure_loader___,
                        ArgId::from(&b"caller"[..]),
                    );
                    let result: AsyncCallResult<TokenIdentifier> = elrond_wasm::load_dyn_arg(
                        &mut ___call_result_loader___,
                        ArgId::from(&b"result"[..]),
                    );
                    ___cb_closure_loader___.assert_no_more_args();
                    ___call_result_loader___.assert_no_more_args();
                    self.sft_issue_callback(&caller, result);
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                _ => {}
            }
            match storage::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
                elrond_wasm::types::CallbackSelectorResult::Processed => {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                    ___cb_data_deserializer___ = recovered_deser;
                }
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {
            let ___tx_hash___ = elrond_wasm::api::BlockchainApi::get_tx_hash(&self.blockchain());
            let ___cb_data_raw___ = elrond_wasm::api::StorageReadApi::storage_load_boxed_bytes(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
            );
            elrond_wasm::api::StorageWriteApi::storage_store_slice_u8(
                &self.get_storage_raw(),
                &___tx_hash___.as_bytes(),
                &[],
            );
            let mut ___cb_data_deserializer___ =
                elrond_wasm::hex_call_data::HexCallDataDeserializer::new(
                    ___cb_data_raw___.as_slice(),
                );
            if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                self::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)
            {
                self.error_api().signal_error(err_msg::CALLBACK_BAD_FUNC);
            }
        }
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ForwarderSftModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "sft_issue",
                only_owner: false,
                payable_in_tokens: &["EGLD"],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_input::<BoxedBytes>("token_display_name");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_input::<BoxedBytes>("token_ticker");
            contract_abi.add_type_descriptions::<BoxedBytes>();
            endpoint_abi.add_output::<AsyncCall<Self::SendApi>>(&[]);
            contract_abi.add_type_descriptions::<AsyncCall<Self::SendApi>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> storage::AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> storage::EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized + storage::ProxyTrait {
        #[allow(clippy::too_many_arguments)]
        fn sft_issue(
            self,
            issue_cost: Self::BigUint,
            token_display_name: BoxedBytes,
            token_ticker: BoxedBytes,
        ) -> elrond_wasm::types::ContractCall<
            Self::SendApi,
            <AsyncCall<Self::SendApi> as elrond_wasm::io::EndpointResult>::DecodeAs,
        > {
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                issue_cost,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"sft_issue"[..]),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_display_name,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            elrond_wasm::io::serialize_contract_call_arg(
                token_ticker,
                ___contract_call___.get_mut_arg_buffer(),
                ___api___.clone(),
            );
            ___contract_call___
        }
    }
    pub struct CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        pub api: SA,
    }
    impl<SA> elrond_wasm::api::CallbackProxyObjApi for CallbackProxyObj<SA>
    where
        SA: elrond_wasm::api::SendApi + 'static,
    {
        type BigUint = SA::AmountType;
        type BigInt = SA::ProxyBigInt;
        type EllipticCurve = SA::ProxyEllipticCurve;
        type Storage = SA::ProxyStorage;
        type SendApi = SA;
        type ErrorApi = SA;
        fn new_cb_proxy_obj(api: SA) -> Self {
            CallbackProxyObj { api }
        }
        fn into_api(self) -> Self::ErrorApi {
            self.api
        }
    }
    pub trait CallbackProxy: elrond_wasm::api::CallbackProxyObjApi + Sized {
        fn sft_issue_callback(self, caller: &Address) -> elrond_wasm::types::CallbackCall {
            let ___api___ = self.into_api();
            let mut ___closure_arg_buffer___ = elrond_wasm::types::ArgBuffer::new();
            elrond_wasm::io::serialize_contract_call_arg(
                caller,
                &mut ___closure_arg_buffer___,
                ___api___.clone(),
            );
            elrond_wasm::types::CallbackCall::from_arg_buffer(
                &b"sft_issue_callback"[..],
                &___closure_arg_buffer___,
            )
        }
    }
    impl<SA> self::CallbackProxy for CallbackProxyObj<SA> where SA: elrond_wasm::api::SendApi + 'static {}
}
mod storage {
    use core::ops::{Add, Div, Mul, Rem, Sub};
    use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
    use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    use elrond_wasm::api::{
        BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi,
        EllipticCurveApi, ProxyObjApi, SendApi,
    };
    use elrond_wasm::api::{ErrorApi, LogApi};
    use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
    use elrond_wasm::err_msg;
    use elrond_wasm::esdt::*;
    use elrond_wasm::io::*;
    use elrond_wasm::non_zero_util::*;
    use elrond_wasm::storage::mappers::*;
    use elrond_wasm::types::*;
    use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
    use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
    use elrond_wasm::{Box, Vec};
    pub trait ForwarderStorageModule: elrond_wasm::api::ContractBase + Sized
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        fn last_issued_token(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
        fn last_error_message(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;
    }
    pub trait AutoImpl: elrond_wasm::api::ContractBase {}
    impl<C> ForwarderStorageModule for C
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        C: AutoImpl,
    {
        fn last_issued_token(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier> {
            let key: &'static [u8] = b"lastIssuedToken";
            < SingleValueMapper < Self :: Storage , TokenIdentifier > as elrond_wasm :: storage :: mappers :: StorageMapper < Self :: Storage > > :: new (self . get_storage_raw () , elrond_wasm :: types :: BoxedBytes :: from (key))
        }
        fn last_error_message(&self) -> SingleValueMapper<Self::Storage, BoxedBytes> {
            let key: &'static [u8] = b"lastErrorMessage";
            < SingleValueMapper < Self :: Storage , BoxedBytes > as elrond_wasm :: storage :: mappers :: StorageMapper < Self :: Storage > > :: new (self . get_storage_raw () , elrond_wasm :: types :: BoxedBytes :: from (key))
        }
    }
    pub trait EndpointWrappers:
        elrond_wasm::api::ContractPrivateApi + ForwarderStorageModule
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    {
        #[inline]
        fn call_last_issued_token(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
            let result = self.last_issued_token();
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        #[inline]
        fn call_last_error_message(&self) {
            self.call_value().check_not_payable();
            elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
            let result = self.last_error_message();
            elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
        }
        fn call(&self, fn_name: &[u8]) -> bool {
            if match fn_name {
                b"callBack" => {
                    self::EndpointWrappers::callback(self);
                    return true;
                }
                [108u8, 97u8, 115u8, 116u8, 73u8, 115u8, 115u8, 117u8, 101u8, 100u8, 84u8, 111u8, 107u8, 101u8, 110u8] =>
                {
                    self.call_last_issued_token();
                    true
                }
                [108u8, 97u8, 115u8, 116u8, 69u8, 114u8, 114u8, 111u8, 114u8, 77u8, 101u8, 115u8, 115u8, 97u8, 103u8, 101u8] =>
                {
                    self.call_last_error_message();
                    true
                }
                other => false,
            } {
                return true;
            }
            false
        }
        fn callback_selector<'a>(
            &self,
            mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
        ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
        }
        fn callback(&self) {}
    }
    pub struct AbiProvider {}
    impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
        type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
        type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
        type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
        type Storage = elrond_wasm::api::uncallable::UncallableApi;
        type SendApi = elrond_wasm::api::uncallable::UncallableApi;
        fn abi() -> elrond_wasm::abi::ContractAbi {
            let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & [] , name : "ForwarderStorageModule" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "lastIssuedToken",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_output::<SingleValueMapper<Self::Storage, TokenIdentifier>>(&[]);
            contract_abi
                .add_type_descriptions::<SingleValueMapper<Self::Storage, TokenIdentifier>>();
            contract_abi.endpoints.push(endpoint_abi);
            let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
                docs: &[],
                name: "lastErrorMessage",
                only_owner: false,
                payable_in_tokens: &[],
                inputs: Vec::new(),
                outputs: Vec::new(),
            };
            endpoint_abi.add_output::<SingleValueMapper<Self::Storage, BoxedBytes>>(&[]);
            contract_abi.add_type_descriptions::<SingleValueMapper<Self::Storage, BoxedBytes>>();
            contract_abi.endpoints.push(endpoint_abi);
            contract_abi
        }
    }
    pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
        api: A,
    }
    impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type BigUint = A::BigUint;
        type BigInt = A::BigInt;
        type EllipticCurve = A::EllipticCurve;
        type Storage = A::Storage;
        type CallValue = A::CallValue;
        type SendApi = A::SendApi;
        type BlockchainApi = A::BlockchainApi;
        type CryptoApi = A::CryptoApi;
        type LogApi = A::LogApi;
        type ErrorApi = A::ErrorApi;
        #[inline]
        fn get_storage_raw(&self) -> Self::Storage {
            self.api.get_storage_raw()
        }
        #[inline]
        fn call_value(&self) -> Self::CallValue {
            self.api.call_value()
        }
        #[inline]
        fn send(&self) -> Self::SendApi {
            self.api.send()
        }
        #[inline]
        fn blockchain(&self) -> Self::BlockchainApi {
            self.api.blockchain()
        }
        #[inline]
        fn crypto(&self) -> Self::CryptoApi {
            self.api.crypto()
        }
        #[inline]
        fn log_api_raw(&self) -> Self::LogApi {
            self.api.log_api_raw()
        }
        #[inline]
        fn error_api(&self) -> Self::ErrorApi {
            self.api.error_api()
        }
    }
    impl<A> AutoImpl for ContractObj<A> where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static
    {
    }
    impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
    where
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        type ArgumentApi = A;
        type FinishApi = A;
        #[inline]
        fn argument_api(&self) -> Self::ArgumentApi {
            self.api.clone()
        }
        #[inline]
        fn finish_api(&self) -> Self::FinishApi {
            self.api.clone()
        }
    }
    impl<A> EndpointWrappers for ContractObj<A>
    where
        Self::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
        for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
        for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
        for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
        Self::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
        for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
        for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
        for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
    }
    impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        fn call(&self, fn_name: &[u8]) -> bool {
            EndpointWrappers::call(self, fn_name)
        }
        fn into_api(self: Box<Self>) -> A {
            self.api
        }
    }
    pub fn contract_obj<A>(api: A) -> ContractObj<A>
    where
        A::BigUint: elrond_wasm::api::BigUintApi,
        for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
        for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
        for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
        for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
        for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
        A::BigInt: elrond_wasm::api::BigIntApi,
        for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
        for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
        for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
        for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
        A: elrond_wasm::api::ContractBase
            + elrond_wasm::api::ErrorApi
            + elrond_wasm::api::EndpointArgumentApi
            + elrond_wasm::api::EndpointFinishApi
            + Clone
            + 'static,
    {
        ContractObj { api }
    }
    pub trait ProxyTrait: elrond_wasm::api::ProxyObjApi + Sized {
        #[allow(clippy::too_many_arguments)]        fn last_issued_token (self) -> elrond_wasm :: types :: ContractCall < Self :: SendApi , < SingleValueMapper < Self :: Storage , TokenIdentifier > as elrond_wasm :: io :: EndpointResult > :: DecodeAs >{
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"lastIssuedToken"[..]),
            );
            ___contract_call___
        }
        #[allow(clippy::too_many_arguments)]        fn last_error_message (self) -> elrond_wasm :: types :: ContractCall < Self :: SendApi , < SingleValueMapper < Self :: Storage , BoxedBytes > as elrond_wasm :: io :: EndpointResult > :: DecodeAs >{
            let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
                self.into_fields();
            let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
                ___api___.clone(),
                ___address___,
                ___token___,
                ___payment___,
                ___nonce___,
                elrond_wasm::types::BoxedBytes::from(&b"lastErrorMessage"[..]),
            );
            ___contract_call___
        }
    }
}
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use elrond_wasm::api::{
    BigIntApi, BigUintApi, BlockchainApi, CallValueApi, ContractBase, CryptoApi, EllipticCurveApi,
    ProxyObjApi, SendApi,
};
use elrond_wasm::api::{ErrorApi, LogApi};
use elrond_wasm::elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
use elrond_wasm::err_msg;
use elrond_wasm::esdt::*;
use elrond_wasm::io::*;
use elrond_wasm::non_zero_util::*;
use elrond_wasm::storage::mappers::*;
use elrond_wasm::types::*;
use elrond_wasm::types::{SCResult::Err, SCResult::Ok};
use elrond_wasm::{non_zero_usize, only_owner, require, sc_error};
use elrond_wasm::{Box, Vec};
/// Test contract for investigating contract calls.
pub trait Forwarder:
    elrond_wasm::api::ContractBase
    + Sized
    + call_sync::ForwarderSyncCallModule
    + call_async::ForwarderAsyncCallModule
    + call_transf_exec::ForwarderTransferExecuteModule
    + contract_change_owner::ChangeOwnerModule
    + contract_deploy::DeployContractModule
    + contract_update::UpgradeContractModule
    + esdt::ForwarderEsdtModule
    + sft::ForwarderSftModule
    + nft::ForwarderNftModule
    + roles::ForwarderRolesModule
    + storage::ForwarderStorageModule
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
{
    fn init(&self) {}
    fn send_egld(&self, to: &Address, amount: &Self::BigUint, opt_data: OptionalArg<BoxedBytes>) {
        let data = match &opt_data {
            OptionalArg::Some(data) => data.as_slice(),
            OptionalArg::None => &[],
        };
        self.send().direct_egld(to, amount, data);
    }
    fn accept_egld_transfer_nft(
        &self,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
        nft_amount: Self::BigUint,
    ) -> SCResult<()> {
        if (!(nft_amount != 0)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Cannot transfer zero amount",
            ))
            .into();
        };
        SCResult::from_result(self.send().direct_esdt_nft_execute(
            &self.blockchain().get_caller(),
            &nft_id,
            nft_nonce,
            &nft_amount,
            0,
            &[],
            &ArgBuffer::new(),
        ))
    }
    fn accept_anything_transfer_nft(
        &self,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
        nft_amount: Self::BigUint,
    ) -> SCResult<()> {
        if (!(nft_amount != 0)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Cannot transfer zero amount",
            ))
            .into();
        };
        let balance = self.blockchain().get_esdt_balance(
            &self.blockchain().get_sc_address(),
            &nft_id,
            nft_nonce,
        );
        if (!(balance >= nft_amount)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Not enough NFT balance",
            ))
            .into();
        };
        SCResult::from_result(self.send().direct_esdt_nft_execute(
            &self.blockchain().get_caller(),
            &nft_id,
            nft_nonce,
            &nft_amount,
            0,
            &[],
            &ArgBuffer::new(),
        ))
    }
    fn accept_anything_with_payable_annotations_transfer_nft(
        &self,
        _payment_token: TokenIdentifier,
        _payment_nonce: u64,
        _payment_amount: Self::BigUint,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
        nft_amount: Self::BigUint,
    ) -> SCResult<()> {
        if (!(nft_amount != 0)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Cannot transfer zero amount",
            ))
            .into();
        };
        let balance = self.blockchain().get_esdt_balance(
            &self.blockchain().get_sc_address(),
            &nft_id,
            nft_nonce,
        );
        if (!(balance >= nft_amount)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Not enough NFT balance",
            ))
            .into();
        };
        SCResult::from_result(self.send().direct_esdt_nft_execute(
            &self.blockchain().get_caller(),
            &nft_id,
            nft_nonce,
            &nft_amount,
            0,
            &[],
            &ArgBuffer::new(),
        ))
    }
    fn accept_egld_check_annotations_call_value(
        &self,
        annotation_payment_token: TokenIdentifier,
        annotation_payment_nonce: u64,
        annotation_payment_amount: Self::BigUint,
    ) -> SCResult<()> {
        let (call_value_payment_amount, call_value_payment_token) =
            self.call_value().payment_token_pair();
        let call_value_payment_nonce = self.call_value().esdt_token_nonce();
        if (!(call_value_payment_token == TokenIdentifier::egld())) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Call value payment token is not EGLD",
            ))
            .into();
        };
        if (!(annotation_payment_token == TokenIdentifier::egld())) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Annotation payment token is not EGLD",
            ))
            .into();
        };
        if (!(call_value_payment_nonce == 0)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Call value payment nonce is not 0",
            ))
            .into();
        };
        if (!(annotation_payment_nonce == 0)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Annotation payment nonce is not 0",
            ))
            .into();
        };
        if (!(annotation_payment_amount == call_value_payment_amount)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Payment amounts differ",
            ))
            .into();
        };
        Ok(())
    }
    fn accept_anything_check_annotations_call_value(
        &self,
        annotation_payment_token: TokenIdentifier,
        annotation_payment_nonce: u64,
        annotation_payment_amount: Self::BigUint,
    ) -> SCResult<()> {
        let (call_value_payment_amount, call_value_payment_token) =
            self.call_value().payment_token_pair();
        let call_value_payment_nonce = self.call_value().esdt_token_nonce();
        if (!(call_value_payment_token == annotation_payment_token)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Payment tokens differs",
            ))
            .into();
        };
        if (!(call_value_payment_nonce == annotation_payment_nonce)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Payment nonces differs",
            ))
            .into();
        };
        if (!(call_value_payment_amount == annotation_payment_amount)) {
            return elrond_wasm::types::SCResult::Err(elrond_wasm::types::SCError::from(
                "Payment amounts differs",
            ))
            .into();
        };
        Ok(())
    }
}
pub trait AutoImpl: elrond_wasm::api::ContractBase {}
impl<C> Forwarder for C
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    C: AutoImpl
        + call_sync::ForwarderSyncCallModule
        + call_async::ForwarderAsyncCallModule
        + call_transf_exec::ForwarderTransferExecuteModule
        + contract_change_owner::ChangeOwnerModule
        + contract_deploy::DeployContractModule
        + contract_update::UpgradeContractModule
        + esdt::ForwarderEsdtModule
        + sft::ForwarderSftModule
        + nft::ForwarderNftModule
        + roles::ForwarderRolesModule
        + storage::ForwarderStorageModule,
{
}
pub trait EndpointWrappers:
    elrond_wasm::api::ContractPrivateApi
    + Forwarder
    + call_sync::EndpointWrappers
    + call_async::EndpointWrappers
    + call_transf_exec::EndpointWrappers
    + contract_change_owner::EndpointWrappers
    + contract_deploy::EndpointWrappers
    + contract_update::EndpointWrappers
    + esdt::EndpointWrappers
    + sft::EndpointWrappers
    + nft::EndpointWrappers
    + roles::EndpointWrappers
    + storage::EndpointWrappers
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
{
    #[inline]
    fn call_init(&self) {
        self.call_value().check_not_payable();
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
        self.init();
    }
    #[inline]
    fn call_send_egld(&self) {
        self.call_value().check_not_payable();
        let mut ___arg_loader = EndpointDynArgLoader::new(self.argument_api());
        let to: &Address = &elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"to"[..]));
        let amount: &Self::BigUint =
            &elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"amount"[..]));
        let opt_data: OptionalArg<BoxedBytes> =
            elrond_wasm::load_dyn_arg(&mut ___arg_loader, ArgId::from(&b"opt_data"[..]));
        ___arg_loader.assert_no_more_args();
        self.send_egld(&to, &amount, opt_data);
    }
    #[inline]
    fn call_accept_egld_transfer_nft(&self) {
        let _ = self.call_value().require_egld();
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 3i32);
        let nft_id = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
            self.argument_api(),
            0i32,
            ArgId::from(&b"nft_id"[..]),
        );
        let nft_nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
            self.argument_api(),
            1i32,
            ArgId::from(&b"nft_nonce"[..]),
        );
        let nft_amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
            self.argument_api(),
            2i32,
            ArgId::from(&b"nft_amount"[..]),
        );
        let result = self.accept_egld_transfer_nft(nft_id, nft_nonce, nft_amount);
        elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
    }
    #[inline]
    fn call_accept_anything_transfer_nft(&self) {
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 3i32);
        let nft_id = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
            self.argument_api(),
            0i32,
            ArgId::from(&b"nft_id"[..]),
        );
        let nft_nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
            self.argument_api(),
            1i32,
            ArgId::from(&b"nft_nonce"[..]),
        );
        let nft_amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
            self.argument_api(),
            2i32,
            ArgId::from(&b"nft_amount"[..]),
        );
        let result = self.accept_anything_transfer_nft(nft_id, nft_nonce, nft_amount);
        elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
    }
    #[inline]
    fn call_accept_anything_with_payable_annotations_transfer_nft(&self) {
        let (_payment_amount, _payment_token) = self.call_value().payment_token_pair();
        let _payment_nonce = self.call_value().esdt_token_nonce();
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 3i32);
        let nft_id = elrond_wasm::load_single_arg::<Self::ArgumentApi, TokenIdentifier>(
            self.argument_api(),
            0i32,
            ArgId::from(&b"nft_id"[..]),
        );
        let nft_nonce = elrond_wasm::load_single_arg::<Self::ArgumentApi, u64>(
            self.argument_api(),
            1i32,
            ArgId::from(&b"nft_nonce"[..]),
        );
        let nft_amount = elrond_wasm::load_single_arg::<Self::ArgumentApi, Self::BigUint>(
            self.argument_api(),
            2i32,
            ArgId::from(&b"nft_amount"[..]),
        );
        let result = self.accept_anything_with_payable_annotations_transfer_nft(
            _payment_token,
            _payment_nonce,
            _payment_amount,
            nft_id,
            nft_nonce,
            nft_amount,
        );
        elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
    }
    #[inline]
    fn call_accept_egld_check_annotations_call_value(&self) {
        let annotation_payment_amount = self.call_value().require_egld();
        let annotation_payment_token = TokenIdentifier::egld();
        let annotation_payment_nonce = 0u64;
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
        let result = self.accept_egld_check_annotations_call_value(
            annotation_payment_token,
            annotation_payment_nonce,
            annotation_payment_amount,
        );
        elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
    }
    #[inline]
    fn call_accept_anything_check_annotations_call_value(&self) {
        let (annotation_payment_amount, annotation_payment_token) =
            self.call_value().payment_token_pair();
        let annotation_payment_nonce = self.call_value().esdt_token_nonce();
        elrond_wasm::api::EndpointArgumentApi::check_num_arguments(&self.argument_api(), 0i32);
        let result = self.accept_anything_check_annotations_call_value(
            annotation_payment_token,
            annotation_payment_nonce,
            annotation_payment_amount,
        );
        elrond_wasm::io::EndpointResult::finish(&result, self.finish_api());
    }
    fn call(&self, fn_name: &[u8]) -> bool {
        if match fn_name {
            b"callBack" => {
                self::EndpointWrappers::callback(self);
                return true;
            }
            [105u8, 110u8, 105u8, 116u8] => {
                self.call_init();
                true
            }
            [115u8, 101u8, 110u8, 100u8, 95u8, 101u8, 103u8, 108u8, 100u8] => {
                self.call_send_egld();
                true
            }
            [97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 101u8, 103u8, 108u8, 100u8, 95u8, 116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 101u8, 114u8, 95u8, 110u8, 102u8, 116u8] =>
            {
                self.call_accept_egld_transfer_nft();
                true
            }
            [97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 97u8, 110u8, 121u8, 116u8, 104u8, 105u8, 110u8, 103u8, 95u8, 116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 101u8, 114u8, 95u8, 110u8, 102u8, 116u8] =>
            {
                self.call_accept_anything_transfer_nft();
                true
            }
            [97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 97u8, 110u8, 121u8, 116u8, 104u8, 105u8, 110u8, 103u8, 95u8, 119u8, 105u8, 116u8, 104u8, 95u8, 112u8, 97u8, 121u8, 97u8, 98u8, 108u8, 101u8, 95u8, 97u8, 110u8, 110u8, 111u8, 116u8, 97u8, 116u8, 105u8, 111u8, 110u8, 115u8, 95u8, 116u8, 114u8, 97u8, 110u8, 115u8, 102u8, 101u8, 114u8, 95u8, 110u8, 102u8, 116u8] =>
            {
                self.call_accept_anything_with_payable_annotations_transfer_nft();
                true
            }
            [97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 101u8, 103u8, 108u8, 100u8, 95u8, 99u8, 104u8, 101u8, 99u8, 107u8, 95u8, 97u8, 110u8, 110u8, 111u8, 116u8, 97u8, 116u8, 105u8, 111u8, 110u8, 115u8, 95u8, 99u8, 97u8, 108u8, 108u8, 95u8, 118u8, 97u8, 108u8, 117u8, 101u8] =>
            {
                self.call_accept_egld_check_annotations_call_value();
                true
            }
            [97u8, 99u8, 99u8, 101u8, 112u8, 116u8, 95u8, 97u8, 110u8, 121u8, 116u8, 104u8, 105u8, 110u8, 103u8, 95u8, 99u8, 104u8, 101u8, 99u8, 107u8, 95u8, 97u8, 110u8, 110u8, 111u8, 116u8, 97u8, 116u8, 105u8, 111u8, 110u8, 115u8, 95u8, 99u8, 97u8, 108u8, 108u8, 95u8, 118u8, 97u8, 108u8, 117u8, 101u8] =>
            {
                self.call_accept_anything_check_annotations_call_value();
                true
            }
            other => false,
        } {
            return true;
        }
        if call_sync::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if call_async::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if call_transf_exec::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if contract_change_owner::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if contract_deploy::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if contract_update::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if esdt::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if sft::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if nft::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if roles::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if storage::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        false
    }
    fn callback_selector<'a>(
        &self,
        mut ___cb_data_deserializer___: elrond_wasm::hex_call_data::HexCallDataDeserializer<'a>,
    ) -> elrond_wasm::types::CallbackSelectorResult<'a> {
        let mut ___call_result_loader___ = EndpointDynArgLoader::new(self.argument_api());
        match ___cb_data_deserializer___.get_func_name() {
            [] => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            _ => {}
        }
        match call_sync::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match call_async::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match call_transf_exec::EndpointWrappers::callback_selector(
            self,
            ___cb_data_deserializer___,
        ) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match contract_change_owner::EndpointWrappers::callback_selector(
            self,
            ___cb_data_deserializer___,
        ) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match contract_deploy::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)
        {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match contract_update::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)
        {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match esdt::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match sft::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match nft::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match roles::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        match storage::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_deser) => {
                ___cb_data_deserializer___ = recovered_deser;
            }
        }
        elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_data_deserializer___)
    }
    fn callback(&self) {
        let ___tx_hash___ = elrond_wasm::api::BlockchainApi::get_tx_hash(&self.blockchain());
        let ___cb_data_raw___ = elrond_wasm::api::StorageReadApi::storage_load_boxed_bytes(
            &self.get_storage_raw(),
            &___tx_hash___.as_bytes(),
        );
        elrond_wasm::api::StorageWriteApi::storage_store_slice_u8(
            &self.get_storage_raw(),
            &___tx_hash___.as_bytes(),
            &[],
        );
        let mut ___cb_data_deserializer___ =
            elrond_wasm::hex_call_data::HexCallDataDeserializer::new(___cb_data_raw___.as_slice());
        if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
            self::EndpointWrappers::callback_selector(self, ___cb_data_deserializer___)
        {
            self.error_api().signal_error(err_msg::CALLBACK_BAD_FUNC);
        }
    }
}
pub struct AbiProvider {}
impl elrond_wasm::api::ContractAbiProvider for AbiProvider {
    type BigUint = elrond_wasm::api::uncallable::BigUintUncallable;
    type BigInt = elrond_wasm::api::uncallable::BigIntUncallable;
    type EllipticCurve = elrond_wasm::api::uncallable::EllipticCurveUncallable;
    type Storage = elrond_wasm::api::uncallable::UncallableApi;
    type SendApi = elrond_wasm::api::uncallable::UncallableApi;
    fn abi() -> elrond_wasm::abi::ContractAbi {
        let mut contract_abi = elrond_wasm :: abi :: ContractAbi { docs : & ["Test contract for investigating contract calls."] , name : "Forwarder" , constructor : None , endpoints : Vec :: new () , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "init",
            only_owner: false,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        contract_abi.constructor = Some(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "send_egld",
            only_owner: false,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<&Address>("to");
        contract_abi.add_type_descriptions::<&Address>();
        endpoint_abi.add_input::<&Self::BigUint>("amount");
        contract_abi.add_type_descriptions::<&Self::BigUint>();
        endpoint_abi.add_input::<OptionalArg<BoxedBytes>>("opt_data");
        contract_abi.add_type_descriptions::<OptionalArg<BoxedBytes>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "accept_egld_transfer_nft",
            only_owner: false,
            payable_in_tokens: &["EGLD"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<TokenIdentifier>("nft_id");
        contract_abi.add_type_descriptions::<TokenIdentifier>();
        endpoint_abi.add_input::<u64>("nft_nonce");
        contract_abi.add_type_descriptions::<u64>();
        endpoint_abi.add_input::<Self::BigUint>("nft_amount");
        contract_abi.add_type_descriptions::<Self::BigUint>();
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "accept_anything_transfer_nft",
            only_owner: false,
            payable_in_tokens: &["*"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<TokenIdentifier>("nft_id");
        contract_abi.add_type_descriptions::<TokenIdentifier>();
        endpoint_abi.add_input::<u64>("nft_nonce");
        contract_abi.add_type_descriptions::<u64>();
        endpoint_abi.add_input::<Self::BigUint>("nft_amount");
        contract_abi.add_type_descriptions::<Self::BigUint>();
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "accept_anything_with_payable_annotations_transfer_nft",
            only_owner: false,
            payable_in_tokens: &["*"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<TokenIdentifier>("nft_id");
        contract_abi.add_type_descriptions::<TokenIdentifier>();
        endpoint_abi.add_input::<u64>("nft_nonce");
        contract_abi.add_type_descriptions::<u64>();
        endpoint_abi.add_input::<Self::BigUint>("nft_amount");
        contract_abi.add_type_descriptions::<Self::BigUint>();
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "accept_egld_check_annotations_call_value",
            only_owner: false,
            payable_in_tokens: &["EGLD"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "accept_anything_check_annotations_call_value",
            only_owner: false,
            payable_in_tokens: &["*"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.endpoints.push(endpoint_abi);
        contract_abi
            .coalesce(<call_sync::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi());
        contract_abi
            .coalesce(<call_async::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi());
        contract_abi.coalesce(
            <call_transf_exec::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi(),
        );
        contract_abi.coalesce(
            <contract_change_owner::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi(),
        );
        contract_abi.coalesce(
            <contract_deploy::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi(),
        );
        contract_abi.coalesce(
            <contract_update::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi(),
        );
        contract_abi.coalesce(<esdt::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi());
        contract_abi.coalesce(<sft::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi());
        contract_abi.coalesce(<nft::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi());
        contract_abi.coalesce(<roles::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi());
        contract_abi
            .coalesce(<storage::AbiProvider as elrond_wasm::api::ContractAbiProvider>::abi());
        contract_abi
    }
}
pub struct ContractObj<A: elrond_wasm::api::ContractBase> {
    api: A,
}
impl<A> elrond_wasm::api::ContractBase for ContractObj<A>
where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
    type BigUint = A::BigUint;
    type BigInt = A::BigInt;
    type EllipticCurve = A::EllipticCurve;
    type Storage = A::Storage;
    type CallValue = A::CallValue;
    type SendApi = A::SendApi;
    type BlockchainApi = A::BlockchainApi;
    type CryptoApi = A::CryptoApi;
    type LogApi = A::LogApi;
    type ErrorApi = A::ErrorApi;
    #[inline]
    fn get_storage_raw(&self) -> Self::Storage {
        self.api.get_storage_raw()
    }
    #[inline]
    fn call_value(&self) -> Self::CallValue {
        self.api.call_value()
    }
    #[inline]
    fn send(&self) -> Self::SendApi {
        self.api.send()
    }
    #[inline]
    fn blockchain(&self) -> Self::BlockchainApi {
        self.api.blockchain()
    }
    #[inline]
    fn crypto(&self) -> Self::CryptoApi {
        self.api.crypto()
    }
    #[inline]
    fn log_api_raw(&self) -> Self::LogApi {
        self.api.log_api_raw()
    }
    #[inline]
    fn error_api(&self) -> Self::ErrorApi {
        self.api.error_api()
    }
}
impl<A> call_sync::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> call_async::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> call_transf_exec::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> contract_change_owner::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> contract_deploy::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> contract_update::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> esdt::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> sft::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> nft::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> roles::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> storage::AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> AutoImpl for ContractObj<A> where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static
{
}
impl<A> elrond_wasm::api::ContractPrivateApi for ContractObj<A>
where
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
    type ArgumentApi = A;
    type FinishApi = A;
    #[inline]
    fn argument_api(&self) -> Self::ArgumentApi {
        self.api.clone()
    }
    #[inline]
    fn finish_api(&self) -> Self::FinishApi {
        self.api.clone()
    }
}
impl<A> call_sync::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> call_async::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> call_transf_exec::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> contract_change_owner::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> contract_deploy::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> contract_update::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> esdt::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> sft::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> nft::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> roles::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> storage::EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> EndpointWrappers for ContractObj<A>
where
    Self::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a Self::BigUint: core::ops::Add<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Sub<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Mul<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Div<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::Rem<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::AddAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::SubAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::MulAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::DivAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::RemAssign<&'b Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitAnd<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitOr<&'b Self::BigUint, Output = Self::BigUint>,
    for<'a, 'b> &'a Self::BigUint: core::ops::BitXor<&'b Self::BigUint, Output = Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitAndAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitOrAssign<&'b Self::BigUint>,
    for<'b> Self::BigUint: core::ops::BitXorAssign<&'b Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shr<usize, Output = Self::BigUint>,
    for<'a> &'a Self::BigUint: core::ops::Shl<usize, Output = Self::BigUint>,
    Self::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a Self::BigInt: core::ops::Add<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Sub<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Mul<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Div<&'b Self::BigInt, Output = Self::BigInt>,
    for<'a, 'b> &'a Self::BigInt: core::ops::Rem<&'b Self::BigInt, Output = Self::BigInt>,
    for<'b> Self::BigInt: core::ops::AddAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::SubAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::MulAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::DivAssign<&'b Self::BigInt>,
    for<'b> Self::BigInt: core::ops::RemAssign<&'b Self::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
}
impl<A> elrond_wasm::api::CallableContract<A> for ContractObj<A>
where
    A::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
    for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
    for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
    for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
    for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
    A::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
    for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
    for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
    for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
    for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
    for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
    for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
    for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
    for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
    for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
    fn call(&self, fn_name: &[u8]) -> bool {
        EndpointWrappers::call(self, fn_name)
    }
    fn into_api(self: Box<Self>) -> A {
        self.api
    }
}
pub fn contract_obj<A>(api: A) -> ContractObj<A>
where
    A::BigUint: elrond_wasm::api::BigUintApi,
    for<'a, 'b> &'a A::BigUint: core::ops::Add<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::Sub<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::Mul<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::Div<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::Rem<&'b A::BigUint, Output = A::BigUint>,
    for<'b> A::BigUint: core::ops::AddAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::SubAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::MulAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::DivAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::RemAssign<&'b A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::BitAnd<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::BitOr<&'b A::BigUint, Output = A::BigUint>,
    for<'a, 'b> &'a A::BigUint: core::ops::BitXor<&'b A::BigUint, Output = A::BigUint>,
    for<'b> A::BigUint: core::ops::BitAndAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::BitOrAssign<&'b A::BigUint>,
    for<'b> A::BigUint: core::ops::BitXorAssign<&'b A::BigUint>,
    for<'a> &'a A::BigUint: core::ops::Shr<usize, Output = A::BigUint>,
    for<'a> &'a A::BigUint: core::ops::Shl<usize, Output = A::BigUint>,
    A::BigInt: elrond_wasm::api::BigIntApi,
    for<'a, 'b> &'a A::BigInt: core::ops::Add<&'b A::BigInt, Output = A::BigInt>,
    for<'a, 'b> &'a A::BigInt: core::ops::Sub<&'b A::BigInt, Output = A::BigInt>,
    for<'a, 'b> &'a A::BigInt: core::ops::Mul<&'b A::BigInt, Output = A::BigInt>,
    for<'a, 'b> &'a A::BigInt: core::ops::Div<&'b A::BigInt, Output = A::BigInt>,
    for<'a, 'b> &'a A::BigInt: core::ops::Rem<&'b A::BigInt, Output = A::BigInt>,
    for<'b> A::BigInt: core::ops::AddAssign<&'b A::BigInt>,
    for<'b> A::BigInt: core::ops::SubAssign<&'b A::BigInt>,
    for<'b> A::BigInt: core::ops::MulAssign<&'b A::BigInt>,
    for<'b> A::BigInt: core::ops::DivAssign<&'b A::BigInt>,
    for<'b> A::BigInt: core::ops::RemAssign<&'b A::BigInt>,
    A: elrond_wasm::api::ContractBase
        + elrond_wasm::api::ErrorApi
        + elrond_wasm::api::EndpointArgumentApi
        + elrond_wasm::api::EndpointFinishApi
        + Clone
        + 'static,
{
    ContractObj { api }
}
pub trait ProxyTrait:
    elrond_wasm::api::ProxyObjApi
    + Sized
    + call_sync::ProxyTrait
    + call_async::ProxyTrait
    + call_transf_exec::ProxyTrait
    + contract_change_owner::ProxyTrait
    + contract_deploy::ProxyTrait
    + contract_update::ProxyTrait
    + esdt::ProxyTrait
    + sft::ProxyTrait
    + nft::ProxyTrait
    + roles::ProxyTrait
    + storage::ProxyTrait
{
    #[allow(clippy::too_many_arguments)]
    fn init(self) -> elrond_wasm::types::ContractDeploy<Self::SendApi> {
        let (___api___, _, _, ___payment___, _) = self.into_fields();
        let mut ___contract_deploy___ =
            elrond_wasm::types::new_contract_deploy(___api___.clone(), ___payment___);
        ___contract_deploy___
    }
    #[allow(clippy::too_many_arguments)]
    fn send_egld(
        self,
        to: &Address,
        amount: &Self::BigUint,
        opt_data: OptionalArg<BoxedBytes>,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <() as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            ___token___,
            ___payment___,
            ___nonce___,
            elrond_wasm::types::BoxedBytes::from(&b"send_egld"[..]),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            to,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            amount,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            opt_data,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    fn accept_egld_transfer_nft(
        self,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
        nft_amount: Self::BigUint,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <SCResult<()> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            ___token___,
            ___payment___,
            ___nonce___,
            elrond_wasm::types::BoxedBytes::from(&b"accept_egld_transfer_nft"[..]),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_id,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_nonce,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_amount,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    fn accept_anything_transfer_nft(
        self,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
        nft_amount: Self::BigUint,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <SCResult<()> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            ___token___,
            ___payment___,
            ___nonce___,
            elrond_wasm::types::BoxedBytes::from(&b"accept_anything_transfer_nft"[..]),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_id,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_nonce,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_amount,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    fn accept_anything_with_payable_annotations_transfer_nft(
        self,
        _payment_token: TokenIdentifier,
        _payment_nonce: u64,
        _payment_amount: Self::BigUint,
        nft_id: TokenIdentifier,
        nft_nonce: u64,
        nft_amount: Self::BigUint,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <SCResult<()> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            _payment_token,
            _payment_amount,
            _payment_nonce,
            elrond_wasm::types::BoxedBytes::from(
                &b"accept_anything_with_payable_annotations_transfer_nft"[..],
            ),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_id,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_nonce,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        elrond_wasm::io::serialize_contract_call_arg(
            nft_amount,
            ___contract_call___.get_mut_arg_buffer(),
            ___api___.clone(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    fn accept_egld_check_annotations_call_value(
        self,
        annotation_payment_token: TokenIdentifier,
        annotation_payment_nonce: u64,
        annotation_payment_amount: Self::BigUint,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <SCResult<()> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            annotation_payment_token,
            annotation_payment_amount,
            annotation_payment_nonce,
            elrond_wasm::types::BoxedBytes::from(&b"accept_egld_check_annotations_call_value"[..]),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    fn accept_anything_check_annotations_call_value(
        self,
        annotation_payment_token: TokenIdentifier,
        annotation_payment_nonce: u64,
        annotation_payment_amount: Self::BigUint,
    ) -> elrond_wasm::types::ContractCall<
        Self::SendApi,
        <SCResult<()> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let (___api___, ___address___, ___token___, ___payment___, ___nonce___) =
            self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___api___.clone(),
            ___address___,
            annotation_payment_token,
            annotation_payment_amount,
            annotation_payment_nonce,
            elrond_wasm::types::BoxedBytes::from(
                &b"accept_anything_check_annotations_call_value"[..],
            ),
        );
        ___contract_call___
    }
}
pub struct Proxy<SA>
where
    SA: elrond_wasm::api::SendApi + 'static,
{
    pub api: SA,
    pub address: Address,
    pub payment_token: elrond_wasm::types::TokenIdentifier,
    pub payment_amount: SA::AmountType,
    pub payment_nonce: u64,
}
impl<SA> elrond_wasm::api::ProxyObjApi for Proxy<SA>
where
    SA: elrond_wasm::api::SendApi + 'static,
{
    type BigUint = SA::AmountType;
    type BigInt = SA::ProxyBigInt;
    type EllipticCurve = SA::ProxyEllipticCurve;
    type Storage = SA::ProxyStorage;
    type SendApi = SA;
    fn new_proxy_obj(api: SA) -> Self {
        Proxy {
            api,
            address: Address::zero(),
            payment_token: elrond_wasm::types::TokenIdentifier::egld(),
            payment_amount: Self::BigUint::zero(),
            payment_nonce: 0,
        }
    }
    #[inline]
    fn contract(mut self, address: Address) -> Self {
        self.address = address;
        self
    }
    fn with_token_transfer(mut self, token: TokenIdentifier, payment: Self::BigUint) -> Self {
        self.payment_token = token;
        self.payment_amount = payment;
        self
    }
    #[inline]
    fn with_nft_nonce(mut self, nonce: u64) -> Self {
        self.payment_nonce = nonce;
        self
    }
    #[inline]
    fn into_fields(self) -> (Self::SendApi, Address, TokenIdentifier, Self::BigUint, u64) {
        (
            self.api,
            self.address,
            self.payment_token,
            self.payment_amount,
            self.payment_nonce,
        )
    }
}
impl<SA> call_sync::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> call_async::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> call_transf_exec::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> contract_change_owner::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> contract_deploy::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> contract_update::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> esdt::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> sft::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> nft::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> roles::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> storage::ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
impl<SA> ProxyTrait for Proxy<SA> where SA: elrond_wasm::api::SendApi {}
