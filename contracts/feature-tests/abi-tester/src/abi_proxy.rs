////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct AbiTesterProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for AbiTesterProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = AbiTesterProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        AbiTesterProxyMethods { wrapped_tx: tx }
    }
}

pub struct AbiTesterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> AbiTesterProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    /// Contract constructor. 
    pub fn init<
        Arg0: CodecInto<i32>,
        Arg1: CodecInto<OnlyShowsUpInConstructor>,
    >(
        self,
        _constructor_arg_1: Arg0,
        _constructor_arg_2: Arg1,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&_constructor_arg_1)
            .argument(&_constructor_arg_2)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> AbiTesterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Example endpoint docs. 
    pub fn echo_abi_test_type<
        Arg0: CodecInto<AbiTestType>,
    >(
        self,
        att: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, AbiTestType> {
        self.wrapped_tx
            .raw_call()
            .function_name("echo_abi_test_type")
            .argument(&att)
            .original_result()
    }

    pub fn echo_enum<
        Arg0: CodecInto<AbiEnum>,
    >(
        self,
        e: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, AbiEnum> {
        self.wrapped_tx
            .raw_call()
            .function_name("echo_enum")
            .argument(&e)
            .original_result()
    }

    pub fn take_managed_type<
        Arg0: CodecInto<AbiManagedType<Env::Api>>,
    >(
        self,
        _arg: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("take_managed_type")
            .argument(&_arg)
            .original_result()
    }

    pub fn multi_result_3(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue3<i32, [u8; 3], BoxedBytes>> {
        self.wrapped_tx
            .raw_call()
            .function_name("multi_result_3")
            .original_result()
    }

    pub fn multi_result_4(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue4<i32, [u8; 3], BoxedBytes, OnlyShowsUpAsNested03>> {
        self.wrapped_tx
            .raw_call()
            .function_name("multi_result_4")
            .original_result()
    }

    pub fn var_args<
        Arg0: CodecInto<u32>,
        Arg1: CodecInto<MultiValueVec<MultiValue2<OnlyShowsUpAsNested04, i32>>>,
    >(
        self,
        _simple_arg: Arg0,
        _var_args: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("var_args")
            .argument(&_simple_arg)
            .argument(&_var_args)
            .original_result()
    }

    pub fn multi_result_vec(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValueVec<MultiValue3<OnlyShowsUpAsNested05, bool, ()>>> {
        self.wrapped_tx
            .raw_call()
            .function_name("multi_result_vec")
            .original_result()
    }

    pub fn optional_arg<
        Arg0: CodecInto<u32>,
        Arg1: CodecInto<OptionalValue<OnlyShowsUpAsNested06>>,
    >(
        self,
        _simple_arg: Arg0,
        _opt_args: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("optional_arg")
            .argument(&_simple_arg)
            .argument(&_opt_args)
            .original_result()
    }

    pub fn optional_result(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, OptionalValue<OnlyShowsUpAsNested07>> {
        self.wrapped_tx
            .raw_call()
            .function_name("optional_result")
            .original_result()
    }

    pub fn address_vs_h256<
        Arg0: CodecInto<Address>,
        Arg1: CodecInto<H256>,
    >(
        self,
        address: Arg0,
        h256: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<Address, H256>> {
        self.wrapped_tx
            .raw_call()
            .function_name("address_vs_h256")
            .argument(&address)
            .argument(&h256)
            .original_result()
    }

    pub fn managed_address_vs_byte_array<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedByteArray<Env::Api, 32usize>>,
    >(
        self,
        address: Arg0,
        byte_array: Arg1,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<ManagedAddress<Env::Api>, ManagedByteArray<Env::Api, 32usize>>> {
        self.wrapped_tx
            .raw_call()
            .function_name("managed_address_vs_byte_array")
            .argument(&address)
            .argument(&byte_array)
            .original_result()
    }

    pub fn esdt_local_role(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtLocalRole> {
        self.wrapped_tx
            .raw_call()
            .function_name("esdt_local_role")
            .original_result()
    }

    pub fn esdt_token_payment(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenPayment<Env::Api>> {
        self.wrapped_tx
            .raw_call()
            .function_name("esdt_token_payment")
            .original_result()
    }

    pub fn esdt_token_data(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, EsdtTokenData<Env::Api>> {
        self.wrapped_tx
            .raw_call()
            .function_name("esdt_token_data")
            .original_result()
    }

    pub fn sample_storage_mapper(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, OnlyShowsUpAsNestedInSingleValueMapper> {
        self.wrapped_tx
            .raw_call()
            .function_name("sample_storage_mapper")
            .original_result()
    }

    pub fn item_for_vec(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, Vec<OnlyShowsUpAsNestedInVec>> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_vec")
            .original_result()
    }

    pub fn item_for_array_vec(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ArrayVec<OnlyShowsUpAsNestedInArrayVec, 3usize>> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_array_vec")
            .original_result()
    }

    pub fn item_for_managed_vec(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ManagedVec<Env::Api, AbiManagedVecItem>> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_managed_vec")
            .original_result()
    }

    pub fn item_for_array<
        Arg0: CodecInto<[OnlyShowsUpAsNestedInArray; 5]>,
    >(
        self,
        _array: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_array")
            .argument(&_array)
            .original_result()
    }

    pub fn item_for_box(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, Box<OnlyShowsUpAsNestedInBox>> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_box")
            .original_result()
    }

    pub fn item_for_boxed_slice(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, Box<[OnlyShowsUpAsNestedInBoxedSlice]>> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_boxed_slice")
            .original_result()
    }

    pub fn item_for_ref<
        Arg0: CodecInto<OnlyShowsUpAsNestedInRef>,
    >(
        self,
        _ref: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_ref")
            .argument(&_ref)
            .original_result()
    }

    pub fn item_for_slice<
        Arg0: CodecInto<Box<[OnlyShowsUpAsNestedInSlice]>>,
    >(
        self,
        _ref: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_slice")
            .argument(&_ref)
            .original_result()
    }

    pub fn item_for_option(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, Option<OnlyShowsUpAsNestedInOption>> {
        self.wrapped_tx
            .raw_call()
            .function_name("item_for_option")
            .original_result()
    }

    pub fn payable_egld(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("payable_egld")
            .original_result()
    }

    pub fn payable_some_token(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("payable_some_token")
            .original_result()
    }

    pub fn payable_any_token(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("payable_any_token")
            .original_result()
    }

    pub fn external_view(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("external_view")
            .original_result()
    }

    pub fn label_a(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("label_a")
            .original_result()
    }

    pub fn label_b(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call()
            .function_name("label_b")
            .original_result()
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpInConstructor {
    pub something: (),
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct AbiTestType {
    pub nested: OnlyShowsUpAsNested01,
    pub next: Option<Box<AbiTestType>>,
    pub tuple_madness: (OnlyShowsUpAsNested02, Option<Box<AbiTestType>>),
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested01 {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested02 {
    pub something: [u8; 0],
}

#[derive(TopEncode, TopDecode)]
pub enum AbiEnum {
    Nothing,
    Something(i32),
    SomethingMore(u8, OnlyShowsUpAsNested08),
    SomeStruct { a: u16, b: OnlyShowsUpAsNested09 },
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested08 {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested09 {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct AbiManagedType<Api>
where
    Api: ManagedTypeApi,
{
    pub big_uint: BigUint<Api>,
    pub integer: i32,
    pub managed_buffer: ManagedBuffer<Api>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested03 {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested04 {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested05 {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested06 {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested07 {}

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug, Copy)]
pub enum EsdtLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
    NftAddUri,
    NftUpdateAttributes,
    Transfer,
}

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug, ManagedVecItem,
)]
pub enum EsdtTokenType {
    Fungible,
    NonFungible,
    SemiFungible,
    Meta,
    Invalid,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInSingleValueMapper {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInVec {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInArrayVec {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, ManagedVecItem)]
pub struct AbiManagedVecItem {
    pub value1: u32,
    pub value2: u32,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInArray {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInBox {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInBoxedSlice {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInRef {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInSlice {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNestedInOption {}

#[derive(TopEncode, TopDecode)]
pub struct OnlyShowsUpInEsdtAttr {
    pub field: OnlyShowsUpAsNested10,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OnlyShowsUpAsNested10 {}

#[derive(TopEncode, TopDecode)]
pub enum ExplicitDiscriminant {
    Zero,
    Thirty,
    Twelve,
    Fifty,
    FiftyOne,
}

#[derive(TopEncode, TopDecode)]
pub enum ExplicitDiscriminantMixed {
    Zero,
    Unit,
    Tuple(u16),
    Five,
    Struct { a: u8, b: u16 },
}