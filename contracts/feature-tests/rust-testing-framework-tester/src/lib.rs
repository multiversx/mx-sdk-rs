#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod dummy_module;

#[derive(TopEncode, TopDecode, TypeAbi, Clone, Debug, PartialEq, Eq)]
pub struct NftDummyAttributes {
    pub creation_epoch: u64,
    pub cool_factor: u8,
}

pub struct StructWithManagedTypes<M: ManagedTypeApi> {
    pub big_uint: BaseBigUint<M>,
    pub buffer: ManagedBuffer<M>,
}

/*
#[multiversx_sc::contract]
pub trait RustTestingFrameworkTester: dummy_module::DummyModule {
    #[init]
    fn init(&self) -> ManagedBuffer {
        self.total_value().set(&BaseBigUint::from(1u32));
        b"constructor-result".into()
    }

    #[endpoint]
    fn sum(&self, first: BaseBigUint, second: BaseBigUint) -> BaseBigUint {
        first + second
    }

    #[endpoint]
    fn sum_sc_result(&self, first: BaseBigUint, second: BaseBigUint) -> BaseBigUint {
        require!(first > 0 && second > 0, "Non-zero required");
        first + second
    }

    #[endpoint]
    fn get_caller_legacy(&self) -> Address {
        #[allow(deprecated)]
        self.blockchain().get_caller_legacy()
    }

    #[endpoint]
    fn get_egld_balance(&self) -> BaseBigUint {
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
    }

    #[endpoint]
    fn get_esdt_balance(&self, token_id: TokenIdentifier, nonce: u64) -> BaseBigUint {
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(token_id), nonce)
    }

    #[payable("EGLD")]
    #[endpoint]
    fn receive_egld(&self) -> BaseBigUint {
        self.call_value().egld_value().clone_value()
    }

    #[payable("EGLD")]
    #[endpoint]
    fn recieve_egld_half(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = &*self.call_value().egld_value() / 2u32;
        self.send().direct(
            &caller,
            &EgldOrEsdtTokenIdentifier::egld(),
            0,
            &payment_amount,
        );
    }

    #[payable("*")]
    #[endpoint]
    fn receive_esdt(&self) -> (TokenIdentifier, BaseBigUint) {
        let payment = self.call_value().single_esdt();
        (payment.token_identifier, payment.amount)
    }

    #[payable("*")]
    #[endpoint]
    fn reject_payment(&self) {
        sc_panic!("No payment allowed!");
    }

    #[payable("*")]
    #[endpoint]
    fn receive_esdt_half(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        let amount = payment.amount / 2u32;

        self.send()
            .direct_esdt(&caller, &payment.token_identifier, 0, &amount);
    }

    #[payable("*")]
    #[endpoint]
    fn receive_multi_esdt(&self) -> ManagedVec<EsdtTokenPayment<CurrentApi>> {
        self.call_value().all_esdt_transfers().clone_value()
    }

    #[payable("*")]
    #[endpoint]
    fn send_nft(
        &self,
        to: ManagedAddress,
        token_id: TokenIdentifier,
        nft_nonce: u64,
        amount: BaseBigUint,
    ) {
        self.send().direct_esdt(&to, &token_id, nft_nonce, &amount);
    }

    #[endpoint]
    fn mint_esdt(&self, token_id: TokenIdentifier, nonce: u64, amount: BaseBigUint) {
        self.send().esdt_local_mint(&token_id, nonce, &amount);
    }

    #[endpoint]
    fn burn_esdt(&self, token_id: TokenIdentifier, nonce: u64, amount: BaseBigUint) {
        self.send().esdt_local_burn(&token_id, nonce, &amount);
    }

    #[endpoint]
    fn create_nft(
        &self,
        token_id: TokenIdentifier,
        amount: BaseBigUint,
        attributes: NftDummyAttributes,
    ) -> u64 {
        self.send().esdt_nft_create(
            &token_id,
            &amount,
            &ManagedBuffer::new(),
            &BaseBigUint::zero(),
            &ManagedBuffer::new(),
            &attributes,
            &ManagedVec::new(),
        )
    }

    #[endpoint]
    fn get_block_epoch(&self) -> u64 {
        self.blockchain().get_block_epoch()
    }

    #[endpoint]
    fn get_block_nonce(&self) -> u64 {
        self.blockchain().get_block_nonce()
    }

    #[endpoint]
    fn get_block_timestamp(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    #[endpoint]
    fn get_random_buffer_once(&self, len: usize) -> ManagedBuffer {
        ManagedBuffer::new_random(len)
    }

    #[endpoint]
    fn get_random_buffer_twice(&self, len1: usize, len2: usize) -> (ManagedBuffer, ManagedBuffer) {
        (
            ManagedBuffer::new_random(len1),
            ManagedBuffer::new_random(len2),
        )
    }

    #[endpoint]
    fn call_other_contract_execute_on_dest(&self, other_sc_address: ManagedAddress) -> BaseBigUint {
        let call_result = self.send_raw().execute_on_dest_context_raw(
            self.blockchain().get_gas_left(),
            &other_sc_address,
            &BaseBigUint::zero(),
            &ManagedBuffer::new_from_bytes(b"getTotalValue"),
            &ManagedArgBuffer::new(),
        );
        if let Some(raw_value) = call_result.try_get(0) {
            BaseBigUint::from_bytes_be_buffer(&raw_value)
        } else {
            BaseBigUint::zero()
        }
    }

    #[endpoint]
    fn call_other_contract_add_async_call(&self, other_sc_address: ManagedAddress, value: BaseBigUint) {
        let mut args = ManagedArgBuffer::new();
        args.push_arg(&value);

        self.send_raw().async_call_raw(
            &other_sc_address,
            &BaseBigUint::zero(),
            &ManagedBuffer::new_from_bytes(b"add"),
            &args,
        );
    }

    #[callback_raw]
    fn callback_raw(&self, _ignore: IgnoreValue) {
        self.callback_executed().set(true);
    }

    #[endpoint(getTotalValue)]
    fn get_total_value(&self) -> BaseBigUint {
        self.total_value().get()
    }

    #[endpoint]
    fn execute_on_dest_add_value(&self, other_sc_address: ManagedAddress, value: BaseBigUint) {
        let mut args = ManagedArgBuffer::new();
        args.push_arg(value);

        let _ = self.send_raw().execute_on_dest_context_raw(
            self.blockchain().get_gas_left(),
            &other_sc_address,
            &BaseBigUint::zero(),
            &ManagedBuffer::new_from_bytes(b"addValue"),
            &args,
        );
    }

    #[endpoint(addValue)]
    fn add(&self, value: BaseBigUint) {
        let caller = self.blockchain().get_caller();

        self.total_value().update(|val| *val += &value);
        self.value_per_caller(&caller).update(|val| *val += value);
    }

    #[endpoint]
    fn panic(&self) {
        sc_panic!("Oh no!");
    }

    fn get_val(&self) -> BaseBigUint {
        self.total_value().get()
    }

    #[storage_mapper("totalValue")]
    fn total_value(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("valuePerCaller")]
    fn value_per_caller(&self, caller: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[storage_mapper("callbackExecuted")]
    fn callback_executed(&self) -> SingleValueMapper<bool>;
}

 */
pub trait RustTestingFrameworkTester : multiversx_sc :: contract_base ::
ContractBase < CurrentApi > + Sized + dummy_module :: DummyModule where
{
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn init(& self,) -> multiversx_sc :: types :: ManagedBuffer < CurrentApi >
    {
        self.total_value().set(& multiversx_sc :: types :: BaseBigUint :: <
            CurrentApi > :: from(1u32)) ; b"constructor-result".into()
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn
sum(& self, first : multiversx_sc :: types :: BaseBigUint < CurrentApi >,
    second : multiversx_sc :: types :: BaseBigUint < CurrentApi >) ->
    multiversx_sc :: types :: BaseBigUint < CurrentApi > { first + second }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn
    sum_sc_result(& self, first : multiversx_sc :: types :: BaseBigUint <
        CurrentApi >, second : multiversx_sc :: types :: BaseBigUint < CurrentApi
    >) -> multiversx_sc :: types :: BaseBigUint < CurrentApi >
    {
        require! (first > 0 && second > 0, "Non-zero required") ; first +
        second
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_caller_legacy(& self,) ->
    Address { #[allow(deprecated)] self.blockchain().get_caller_legacy() }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn get_egld_balance(& self,) -> multiversx_sc :: types :: BaseBigUint <
        CurrentApi >
    {
        self.blockchain().get_sc_balance(& multiversx_sc :: types ::
        EgldOrEsdtTokenIdentifier :: < CurrentApi > :: egld(), 0)
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn
get_esdt_balance(& self, token_id : multiversx_sc :: types ::
TokenIdentifier < CurrentApi >, nonce : u64) -> multiversx_sc :: types ::
    BaseBigUint < CurrentApi >
{
    self.blockchain().get_sc_balance(& multiversx_sc :: types ::
    EgldOrEsdtTokenIdentifier :: < CurrentApi > :: esdt(token_id), nonce)
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn receive_egld(& self,) ->
    multiversx_sc :: types :: BaseBigUint < CurrentApi >
{ self.call_value().egld_value().clone_value() }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn recieve_egld_half(& self,)
    {
        let caller = self.blockchain().get_caller() ; let payment_amount = & *
        self.call_value().egld_value() / 2u32 ;
        self.send().direct(& caller, & multiversx_sc :: types ::
        EgldOrEsdtTokenIdentifier :: < CurrentApi > :: egld(), 0, &
                               payment_amount,) ;
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn receive_esdt(& self,) ->
    (multiversx_sc :: types :: TokenIdentifier < CurrentApi >, multiversx_sc
    :: types :: BaseBigUint < CurrentApi >)
{
    let payment = self.call_value().single_esdt() ;
    (payment.token_identifier, payment.amount)
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn reject_payment(& self,)
{ sc_panic! ("No payment allowed!") ; }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn receive_esdt_half(& self,)
    {
        let caller = self.blockchain().get_caller() ; let payment =
        self.call_value().single_esdt() ; let amount = payment.amount / 2u32 ;
        self.send().direct_esdt(& caller, & payment.token_identifier, 0, &
            amount) ;
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn receive_multi_esdt(& self,) ->
    ManagedVec < CurrentApi, EsdtTokenPayment < CurrentApi > >
{ self.call_value().all_esdt_transfers().clone_value() }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn
    send_nft(& self, to : multiversx_sc :: types :: ManagedAddress <
        CurrentApi >, token_id : multiversx_sc :: types :: TokenIdentifier <
        CurrentApi >, nft_nonce : u64, amount : multiversx_sc :: types ::
    BaseBigUint < CurrentApi >)
    { self.send().direct_esdt(& to, & token_id, nft_nonce, & amount) ; }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn
    mint_esdt(& self, token_id : multiversx_sc :: types :: TokenIdentifier <
        CurrentApi >, nonce : u64, amount : multiversx_sc :: types :: BaseBigUint
    < CurrentApi >)
    { self.send().esdt_local_mint(& token_id, nonce, & amount) ; }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn
    burn_esdt(& self, token_id : multiversx_sc :: types :: TokenIdentifier <
        CurrentApi >, nonce : u64, amount : multiversx_sc :: types :: BaseBigUint
    < CurrentApi >)
    { self.send().esdt_local_burn(& token_id, nonce, & amount) ; }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn
    create_nft(& self, token_id : multiversx_sc :: types :: TokenIdentifier <
        CurrentApi >, amount : multiversx_sc :: types :: BaseBigUint < CurrentApi
    >, attributes : NftDummyAttributes) -> u64
    {
        self.send().esdt_nft_create(& token_id, & amount, & multiversx_sc ::
        types :: ManagedBuffer :: < CurrentApi > :: new(), & multiversx_sc ::
        types :: BaseBigUint :: < CurrentApi > :: zero(), & multiversx_sc ::
        types :: ManagedBuffer :: < CurrentApi > :: new(), & attributes, &
                                        ManagedVec :: new(),)
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_block_epoch(& self,) -> u64
{ self.blockchain().get_block_epoch() }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn get_block_nonce(& self,) -> u64 { self.blockchain().get_block_nonce() }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn get_block_timestamp(& self,) -> u64
    { self.blockchain().get_block_timestamp() }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn get_random_buffer_once(& self, len : usize) -> multiversx_sc :: types
    :: ManagedBuffer < CurrentApi >
    {
        multiversx_sc :: types :: ManagedBuffer :: < CurrentApi > ::
        new_random(len)
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn
get_random_buffer_twice(& self, len1 : usize, len2 : usize) ->
    (multiversx_sc :: types :: ManagedBuffer < CurrentApi >, multiversx_sc ::
    types :: ManagedBuffer < CurrentApi >)
{
    (multiversx_sc :: types :: ManagedBuffer :: < CurrentApi > ::
     new_random(len1), multiversx_sc :: types :: ManagedBuffer :: <
        CurrentApi > :: new_random(len2),)
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn
call_other_contract_execute_on_dest(& self, other_sc_address :
multiversx_sc :: types :: ManagedAddress < CurrentApi >) -> multiversx_sc
    :: types :: BaseBigUint < CurrentApi >
{
    let call_result =
        self.send_raw().execute_on_dest_context_raw(self.blockchain().get_gas_left(),
                                                    & other_sc_address, & multiversx_sc :: types :: BaseBigUint :: <
                CurrentApi > :: zero(), & multiversx_sc :: types :: ManagedBuffer :: <
                CurrentApi > :: new_from_bytes(b"getTotalValue"), & ManagedArgBuffer
            :: new(),) ; if let Some(raw_value) = call_result.try_get(0)
{
    multiversx_sc :: types :: BaseBigUint :: < CurrentApi > ::
    from_bytes_be_buffer(& raw_value)
} else
{ multiversx_sc :: types :: BaseBigUint :: < CurrentApi > :: zero() }
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn
call_other_contract_add_async_call(& self, other_sc_address :
multiversx_sc :: types :: ManagedAddress < CurrentApi >, value :
                                   multiversx_sc :: types :: BaseBigUint < CurrentApi >)
{
    let mut args = ManagedArgBuffer :: new() ; args.push_arg(& value) ;
    self.send_raw().async_call_raw(& other_sc_address, & multiversx_sc ::
    types :: BaseBigUint :: < CurrentApi > :: zero(), & multiversx_sc ::
    types :: ManagedBuffer :: < CurrentApi > :: new_from_bytes(b"add"), &
                                       args,) ;
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn
callback_raw(& self, _ignore : IgnoreValue)
{ self.callback_executed().set(true) ; }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn get_total_value(& self,) -> multiversx_sc :: types :: BaseBigUint <
        CurrentApi > { self.total_value().get() }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn
    execute_on_dest_add_value(& self, other_sc_address : multiversx_sc ::
    types :: ManagedAddress < CurrentApi >, value : multiversx_sc :: types ::
    BaseBigUint < CurrentApi >)
    {
        let mut args = ManagedArgBuffer :: new() ; args.push_arg(value) ; let
        _ =
        self.send_raw().execute_on_dest_context_raw(self.blockchain().get_gas_left(),
                                                    & other_sc_address, & multiversx_sc :: types :: BaseBigUint :: <
                CurrentApi > :: zero(), & multiversx_sc :: types :: ManagedBuffer :: <
                CurrentApi > :: new_from_bytes(b"addValue"), & args,) ;
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn
add(& self, value : multiversx_sc :: types :: BaseBigUint < CurrentApi >)
{
    let caller = self.blockchain().get_caller() ;
    self.total_value().update(| val | * val += & value) ;
    self.value_per_caller(& caller).update(| val | * val += value) ;
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn panic(& self,)
{ sc_panic! ("Oh no!") ; } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_val(& self,) -> multiversx_sc
    :: types :: BaseBigUint < CurrentApi > { self.total_value().get() }
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn total_value(& self,) -> SingleValueMapper < BigUint > ;
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn
    value_per_caller(& self, caller : & multiversx_sc :: types ::
    ManagedAddress < CurrentApi >) -> SingleValueMapper < BigUint > ;
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn callback_executed(& self,) -> SingleValueMapper < bool > ;
} pub trait AutoImpl : multiversx_sc :: contract_base :: ContractBase <
    CurrentApi > {} impl < C > RustTestingFrameworkTester for C where C : AutoImpl
+ dummy_module :: DummyModule
{
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn total_value(& self,) -> SingleValueMapper < BigUint >
    {
        let mut ___key___ = multiversx_sc :: storage :: StorageKey :: <
            CurrentApi > :: new(& b"totalValue" [..],) ; < SingleValueMapper <
        BigUint > as multiversx_sc :: storage :: mappers :: StorageMapper <
        CurrentApi >> :: new(___key___)
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn
value_per_caller(& self, caller : & multiversx_sc :: types ::
ManagedAddress < CurrentApi >) -> SingleValueMapper < BigUint >
{
    let mut ___key___ = multiversx_sc :: storage :: StorageKey :: <
        CurrentApi > :: new(& b"valuePerCaller" [..],) ;
    ___key___.append_item(& caller) ; < SingleValueMapper < BigUint > as
multiversx_sc :: storage :: mappers :: StorageMapper < CurrentApi >>
:: new(___key___)
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn callback_executed(& self,) ->
    SingleValueMapper < bool >
{
    let mut ___key___ = multiversx_sc :: storage :: StorageKey :: <
        CurrentApi > :: new(& b"callbackExecuted" [..],) ; < SingleValueMapper
< bool > as multiversx_sc :: storage :: mappers :: StorageMapper <
    CurrentApi >> :: new(___key___)
}
} impl AutoImpl for multiversx_sc :: contract_base :: UniversalContractObj <
    CurrentApi > {} pub trait EndpointWrappers : multiversx_sc :: contract_base ::
ContractBase < CurrentApi > + RustTestingFrameworkTester + dummy_module ::
EndpointWrappers
{
    #[inline] fn call_init(& self)
    {
        < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
        multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
        > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
        CurrentApi, () > (()) ; let result = self.init() ; multiversx_sc :: io
    :: finish_multi :: < CurrentApi, _ > (& result) ;
    } #[inline] fn call_sum(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(first, (second, ())) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: BaseBigUint < CurrentApi >,
     (multiversx_sc :: types :: BaseBigUint < CurrentApi >, ())) >
    (("first", ("second", ()))) ; let result = self.sum(first, second) ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_sum_sc_result(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(first, (second, ())) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: BaseBigUint < CurrentApi >,
     (multiversx_sc :: types :: BaseBigUint < CurrentApi >, ())) >
    (("first", ("second", ()))) ; let result =
    self.sum_sc_result(first, second) ; multiversx_sc :: io ::
finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_get_caller_legacy(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.get_caller_legacy() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_get_egld_balance(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.get_egld_balance() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_get_esdt_balance(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(token_id, (nonce, ())) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: TokenIdentifier < CurrentApi >, (u64, ()))
> (("token_id", ("nonce", ()))) ; let result =
    self.get_esdt_balance(token_id, nonce) ; multiversx_sc :: io ::
finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_receive_egld(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: payable_egld :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.receive_egld() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_recieve_egld_half(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: payable_egld :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; self.recieve_egld_half() ;
} #[inline] fn call_receive_esdt(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: payable_any :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.receive_esdt() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_reject_payment(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: payable_any :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; self.reject_payment() ;
} #[inline] fn call_receive_esdt_half(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: payable_any :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; self.receive_esdt_half() ;
} #[inline] fn call_receive_multi_esdt(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: payable_any :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.receive_multi_esdt() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_send_nft(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: payable_any :: < CurrentApi
    > () ; let(to, (token_id, (nft_nonce, (amount, ())))) = multiversx_sc
:: io :: load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: ManagedAddress < CurrentApi >,
     (multiversx_sc :: types :: TokenIdentifier < CurrentApi >,
      (u64, (multiversx_sc :: types :: BaseBigUint < CurrentApi >, ())))) >
    (("to", ("token_id", ("nft_nonce", ("amount", ()))))) ;
    self.send_nft(to, token_id, nft_nonce, amount) ;
} #[inline] fn call_mint_esdt(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(token_id, (nonce, (amount, ()))) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: TokenIdentifier < CurrentApi >,
     (u64, (multiversx_sc :: types :: BaseBigUint < CurrentApi >, ()))) >
    (("token_id", ("nonce", ("amount", ())))) ;
    self.mint_esdt(token_id, nonce, amount) ;
} #[inline] fn call_burn_esdt(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(token_id, (nonce, (amount, ()))) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: TokenIdentifier < CurrentApi >,
     (u64, (multiversx_sc :: types :: BaseBigUint < CurrentApi >, ()))) >
    (("token_id", ("nonce", ("amount", ())))) ;
    self.burn_esdt(token_id, nonce, amount) ;
} #[inline] fn call_create_nft(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(token_id, (amount, (attributes, ()))) = multiversx_sc :: io
:: load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: TokenIdentifier < CurrentApi >,
     (multiversx_sc :: types :: BaseBigUint < CurrentApi >,
      (NftDummyAttributes, ()))) >
    (("token_id", ("amount", ("attributes", ())))) ; let result =
    self.create_nft(token_id, amount, attributes) ; multiversx_sc :: io ::
finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_get_block_epoch(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.get_block_epoch() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_get_block_nonce(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.get_block_nonce() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_get_block_timestamp(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.get_block_timestamp() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_get_random_buffer_once(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(len, ()) = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, (usize, ()) > (("len", ())) ; let result =
    self.get_random_buffer_once(len) ; multiversx_sc :: io :: finish_multi
    :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_get_random_buffer_twice(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(len1, (len2, ())) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi, (usize, (usize, ())) >
    (("len1", ("len2", ()))) ; let result =
    self.get_random_buffer_twice(len1, len2) ; multiversx_sc :: io ::
finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_call_other_contract_execute_on_dest(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(other_sc_address, ()) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: ManagedAddress < CurrentApi >, ()) >
    (("other_sc_address", ())) ; let result =
    self.call_other_contract_execute_on_dest(other_sc_address) ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_call_other_contract_add_async_call(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(other_sc_address, (value, ())) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: ManagedAddress < CurrentApi >,
     (multiversx_sc :: types :: BaseBigUint < CurrentApi >, ())) >
    (("other_sc_address", ("value", ()))) ;
    self.call_other_contract_add_async_call(other_sc_address, value) ;
} #[inline] fn call_get_total_value(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; let result = self.get_total_value() ;
    multiversx_sc :: io :: finish_multi :: < CurrentApi, _ > (& result) ;
} #[inline] fn call_execute_on_dest_add_value(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(other_sc_address, (value, ())) = multiversx_sc :: io ::
load_endpoint_args :: < CurrentApi,
    (multiversx_sc :: types :: ManagedAddress < CurrentApi >,
     (multiversx_sc :: types :: BaseBigUint < CurrentApi >, ())) >
    (("other_sc_address", ("value", ()))) ;
    self.execute_on_dest_add_value(other_sc_address, value) ;
} #[inline] fn call_add(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let(value, ()) = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, (multiversx_sc :: types :: BaseBigUint < CurrentApi >, ())
> (("value", ())) ; self.add(value) ;
} #[inline] fn call_panic(& self)
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: not_payable :: < CurrentApi
    > () ; let() = multiversx_sc :: io :: load_endpoint_args :: <
    CurrentApi, () > (()) ; self.panic() ;
} fn call(& self, fn_name : & str) -> bool
{
    if match fn_name
    {
        "callBack" =>
            { self :: EndpointWrappers :: callback(self) ; return true ; },
        "init" if < CurrentApi as multiversx_sc :: api :: VMApi > ::
        external_view_init_override() =>
            {
                multiversx_sc :: external_view_contract ::
                external_view_contract_constructor :: < CurrentApi > () ;
                return true ;
            }, "init" if! < CurrentApi as multiversx_sc :: api :: VMApi > ::
    external_view_init_override() => { self.call_init() ; true },
        "sum" => { self.call_sum() ; true }, "sum_sc_result" =>
        { self.call_sum_sc_result() ; true }, "get_caller_legacy" =>
        { self.call_get_caller_legacy() ; true }, "get_egld_balance" =>
        { self.call_get_egld_balance() ; true }, "get_esdt_balance" =>
        { self.call_get_esdt_balance() ; true }, "receive_egld" =>
        { self.call_receive_egld() ; true }, "recieve_egld_half" =>
        { self.call_recieve_egld_half() ; true }, "receive_esdt" =>
        { self.call_receive_esdt() ; true }, "reject_payment" =>
        { self.call_reject_payment() ; true }, "receive_esdt_half" =>
        { self.call_receive_esdt_half() ; true }, "receive_multi_esdt" =>
        { self.call_receive_multi_esdt() ; true }, "send_nft" =>
        { self.call_send_nft() ; true }, "mint_esdt" =>
        { self.call_mint_esdt() ; true }, "burn_esdt" =>
        { self.call_burn_esdt() ; true }, "create_nft" =>
        { self.call_create_nft() ; true }, "get_block_epoch" =>
        { self.call_get_block_epoch() ; true }, "get_block_nonce" =>
        { self.call_get_block_nonce() ; true }, "get_block_timestamp" =>
        { self.call_get_block_timestamp() ; true },
        "get_random_buffer_once" =>
            { self.call_get_random_buffer_once() ; true },
        "get_random_buffer_twice" =>
            { self.call_get_random_buffer_twice() ; true },
        "call_other_contract_execute_on_dest" =>
            { self.call_call_other_contract_execute_on_dest() ; true },
        "call_other_contract_add_async_call" =>
            { self.call_call_other_contract_add_async_call() ; true },
        "getTotalValue" => { self.call_get_total_value() ; true },
        "execute_on_dest_add_value" =>
            { self.call_execute_on_dest_add_value() ; true }, "addValue" =>
        { self.call_add() ; true }, "panic" =>
        { self.call_panic() ; true }, other => false
    } { return true ; } if dummy_module :: EndpointWrappers ::
call(self, fn_name) { return true ; } false
} fn
callback_selector(& self, mut ___cb_closure___ : multiversx_sc :: types ::
CallbackClosureForDeser < CurrentApi >) -> multiversx_sc :: types ::
    CallbackSelectorResult < CurrentApi >
{
    < CurrentApi as multiversx_sc :: api :: VMApi > :: init_static() ;
    multiversx_sc :: io :: call_value_init :: payable_any :: < CurrentApi
    > () ; let(_ignore, ()) = multiversx_sc :: io :: load_endpoint_args ::
< CurrentApi, (IgnoreValue, ()) > (("_ignore", ())) ;
    self.callback_raw(_ignore) ; multiversx_sc :: types ::
CallbackSelectorResult :: Processed
} fn callback(& self)
{
    let _ = self :: EndpointWrappers ::
    callback_selector(self, multiversx_sc :: types ::
    CallbackClosureForDeser :: no_callback(),) ;
}
} impl EndpointWrappers for multiversx_sc :: contract_base ::
UniversalContractObj < CurrentApi > where {} pub struct AbiProvider {} impl
multiversx_sc :: contract_base :: ContractAbiProvider for AbiProvider
{
    type Api = multiversx_sc :: api :: uncallable :: UncallableApi ; fn abi()
    -> multiversx_sc :: abi :: ContractAbi
{
    let mut contract_abi = multiversx_sc :: abi :: ContractAbi
    {
        build_info : multiversx_sc :: abi :: BuildInfoAbi
        {
            contract_crate : multiversx_sc :: abi :: ContractCrateBuildAbi
            {
                name : env! ("CARGO_PKG_NAME"), version : env!
            ("CARGO_PKG_VERSION"), git_version : "",
            }, framework : multiversx_sc :: abi :: FrameworkBuildAbi ::
        create(),
        }, docs : & [], name : "RustTestingFrameworkTester", constructors
    : multiversx_sc :: types :: heap :: Vec :: new(), endpoints :
    multiversx_sc :: types :: heap :: Vec :: new(), promise_callbacks
    : multiversx_sc :: types :: heap :: Vec :: new(), events :
    multiversx_sc :: types :: heap :: Vec :: new(), has_callback :
    true, type_descriptions : < multiversx_sc :: abi ::
    TypeDescriptionContainerImpl as multiversx_sc :: abi ::
    TypeDescriptionContainer > :: new(),
    } ; let mut endpoint_abi = multiversx_sc :: abi :: EndpointAbi
{
    docs : & [], name : "init", rust_method_name : "init", only_owner
: false, only_admin : false, mutability : multiversx_sc :: abi ::
EndpointMutabilityAbi :: Mutable, endpoint_type : multiversx_sc ::
abi :: EndpointTypeAbi :: Init, payable_in_tokens : & [], inputs :
multiversx_sc :: types :: heap :: Vec :: new(), outputs :
multiversx_sc :: types :: heap :: Vec :: new(), labels : & [],
} ; endpoint_abi.add_output :: < multiversx_sc :: types ::
ManagedBuffer < CurrentApi > > (& []) ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    ManagedBuffer < CurrentApi > > () ;
    contract_abi.constructors.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "sum", rust_method_name : "sum", only_owner :
    false, only_admin : false, mutability : multiversx_sc :: abi ::
    EndpointMutabilityAbi :: Mutable, endpoint_type : multiversx_sc ::
    abi :: EndpointTypeAbi :: Endpoint, payable_in_tokens : & [],
        inputs : multiversx_sc :: types :: heap :: Vec :: new(), outputs :
    multiversx_sc :: types :: heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types :: BaseBigUint
< CurrentApi > > ("first") ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    endpoint_abi.add_input :: < multiversx_sc :: types :: BaseBigUint <
        CurrentApi > > ("second") ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    endpoint_abi.add_output :: < multiversx_sc :: types :: BaseBigUint <
        CurrentApi > > (& []) ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "sum_sc_result", rust_method_name :
    "sum_sc_result", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types :: BaseBigUint
< CurrentApi > > ("first") ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    endpoint_abi.add_input :: < multiversx_sc :: types :: BaseBigUint <
        CurrentApi > > ("second") ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    endpoint_abi.add_output :: < multiversx_sc :: types :: BaseBigUint <
        CurrentApi > > (& []) ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "get_caller_legacy", rust_method_name :
    "get_caller_legacy", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: < Address > (& []) ;
    contract_abi.add_type_descriptions :: < Address > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "get_egld_balance", rust_method_name :
    "get_egld_balance", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: < multiversx_sc :: types :: BaseBigUint
< CurrentApi > > (& []) ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "get_esdt_balance", rust_method_name :
    "get_esdt_balance", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types ::
TokenIdentifier < CurrentApi > > ("token_id") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    TokenIdentifier < CurrentApi > > () ; endpoint_abi.add_input :: < u64
> ("nonce") ; contract_abi.add_type_descriptions :: < u64 > () ;
    endpoint_abi.add_output :: < multiversx_sc :: types :: BaseBigUint <
        CurrentApi > > (& []) ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "receive_egld", rust_method_name :
    "receive_egld", only_owner : false, only_admin : false, mutability
    : multiversx_sc :: abi :: EndpointMutabilityAbi :: Mutable,
        endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi ::
        Endpoint, payable_in_tokens : & ["EGLD"], inputs : multiversx_sc
    :: types :: heap :: Vec :: new(), outputs : multiversx_sc :: types
    :: heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: < multiversx_sc :: types :: BaseBigUint
< CurrentApi > > (& []) ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "recieve_egld_half", rust_method_name :
    "recieve_egld_half", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & ["EGLD"], inputs :
    multiversx_sc :: types :: heap :: Vec :: new(), outputs :
    multiversx_sc :: types :: heap :: Vec :: new(), labels : & [],
    } ; contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "receive_esdt", rust_method_name :
    "receive_esdt", only_owner : false, only_admin : false, mutability
    : multiversx_sc :: abi :: EndpointMutabilityAbi :: Mutable,
        endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi ::
        Endpoint, payable_in_tokens : & ["*"], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: <
    (multiversx_sc :: types :: TokenIdentifier < CurrentApi >,
     multiversx_sc :: types :: BaseBigUint < CurrentApi >) > (& []) ;
    contract_abi.add_type_descriptions :: <
        (multiversx_sc :: types :: TokenIdentifier < CurrentApi >,
         multiversx_sc :: types :: BaseBigUint < CurrentApi >) > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "reject_payment", rust_method_name :
    "reject_payment", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & ["*"], inputs : multiversx_sc
    :: types :: heap :: Vec :: new(), outputs : multiversx_sc :: types
    :: heap :: Vec :: new(), labels : & [],
    } ; contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "receive_esdt_half", rust_method_name :
    "receive_esdt_half", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & ["*"], inputs : multiversx_sc
    :: types :: heap :: Vec :: new(), outputs : multiversx_sc :: types
    :: heap :: Vec :: new(), labels : & [],
    } ; contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "receive_multi_esdt", rust_method_name :
    "receive_multi_esdt", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & ["*"], inputs : multiversx_sc
    :: types :: heap :: Vec :: new(), outputs : multiversx_sc :: types
    :: heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: < ManagedVec < CurrentApi,
    EsdtTokenPayment < CurrentApi > > > (& []) ;
    contract_abi.add_type_descriptions :: < ManagedVec < CurrentApi,
        EsdtTokenPayment < CurrentApi > > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "send_nft", rust_method_name : "send_nft",
        only_owner : false, only_admin : false, mutability : multiversx_sc
    :: abi :: EndpointMutabilityAbi :: Mutable, endpoint_type :
    multiversx_sc :: abi :: EndpointTypeAbi :: Endpoint,
        payable_in_tokens : & ["*"], inputs : multiversx_sc :: types ::
    heap :: Vec :: new(), outputs : multiversx_sc :: types :: heap ::
    Vec :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types ::
ManagedAddress < CurrentApi > > ("to") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    ManagedAddress < CurrentApi > > () ; endpoint_abi.add_input :: <
    multiversx_sc :: types :: TokenIdentifier < CurrentApi > >
("token_id") ; contract_abi.add_type_descriptions :: < multiversx_sc
:: types :: TokenIdentifier < CurrentApi > > () ;
    endpoint_abi.add_input :: < u64 > ("nft_nonce") ;
    contract_abi.add_type_descriptions :: < u64 > () ;
    endpoint_abi.add_input :: < multiversx_sc :: types :: BaseBigUint <
        CurrentApi > > ("amount") ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "mint_esdt", rust_method_name : "mint_esdt",
        only_owner : false, only_admin : false, mutability : multiversx_sc
    :: abi :: EndpointMutabilityAbi :: Mutable, endpoint_type :
    multiversx_sc :: abi :: EndpointTypeAbi :: Endpoint,
        payable_in_tokens : & [], inputs : multiversx_sc :: types :: heap
    :: Vec :: new(), outputs : multiversx_sc :: types :: heap :: Vec
    :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types ::
TokenIdentifier < CurrentApi > > ("token_id") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    TokenIdentifier < CurrentApi > > () ; endpoint_abi.add_input :: < u64
> ("nonce") ; contract_abi.add_type_descriptions :: < u64 > () ;
    endpoint_abi.add_input :: < multiversx_sc :: types :: BaseBigUint <
        CurrentApi > > ("amount") ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "burn_esdt", rust_method_name : "burn_esdt",
        only_owner : false, only_admin : false, mutability : multiversx_sc
    :: abi :: EndpointMutabilityAbi :: Mutable, endpoint_type :
    multiversx_sc :: abi :: EndpointTypeAbi :: Endpoint,
        payable_in_tokens : & [], inputs : multiversx_sc :: types :: heap
    :: Vec :: new(), outputs : multiversx_sc :: types :: heap :: Vec
    :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types ::
TokenIdentifier < CurrentApi > > ("token_id") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    TokenIdentifier < CurrentApi > > () ; endpoint_abi.add_input :: < u64
> ("nonce") ; contract_abi.add_type_descriptions :: < u64 > () ;
    endpoint_abi.add_input :: < multiversx_sc :: types :: BaseBigUint <
        CurrentApi > > ("amount") ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "create_nft", rust_method_name : "create_nft",
        only_owner : false, only_admin : false, mutability : multiversx_sc
    :: abi :: EndpointMutabilityAbi :: Mutable, endpoint_type :
    multiversx_sc :: abi :: EndpointTypeAbi :: Endpoint,
        payable_in_tokens : & [], inputs : multiversx_sc :: types :: heap
    :: Vec :: new(), outputs : multiversx_sc :: types :: heap :: Vec
    :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types ::
TokenIdentifier < CurrentApi > > ("token_id") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    TokenIdentifier < CurrentApi > > () ; endpoint_abi.add_input :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > ("amount") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    BaseBigUint < CurrentApi > > () ; endpoint_abi.add_input :: <
    NftDummyAttributes > ("attributes") ;
    contract_abi.add_type_descriptions :: < NftDummyAttributes > () ;
    endpoint_abi.add_output :: < u64 > (& []) ;
    contract_abi.add_type_descriptions :: < u64 > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "get_block_epoch", rust_method_name :
    "get_block_epoch", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: < u64 > (& []) ;
    contract_abi.add_type_descriptions :: < u64 > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "get_block_nonce", rust_method_name :
    "get_block_nonce", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: < u64 > (& []) ;
    contract_abi.add_type_descriptions :: < u64 > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "get_block_timestamp", rust_method_name :
    "get_block_timestamp", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: < u64 > (& []) ;
    contract_abi.add_type_descriptions :: < u64 > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "get_random_buffer_once", rust_method_name :
    "get_random_buffer_once", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < usize > ("len") ;
    contract_abi.add_type_descriptions :: < usize > () ;
    endpoint_abi.add_output :: < multiversx_sc :: types :: ManagedBuffer <
        CurrentApi > > (& []) ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: ManagedBuffer < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "get_random_buffer_twice", rust_method_name :
    "get_random_buffer_twice", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < usize > ("len1") ;
    contract_abi.add_type_descriptions :: < usize > () ;
    endpoint_abi.add_input :: < usize > ("len2") ;
    contract_abi.add_type_descriptions :: < usize > () ;
    endpoint_abi.add_output :: <
        (multiversx_sc :: types :: ManagedBuffer < CurrentApi >, multiversx_sc
        :: types :: ManagedBuffer < CurrentApi >) > (& []) ;
    contract_abi.add_type_descriptions :: <
        (multiversx_sc :: types :: ManagedBuffer < CurrentApi >, multiversx_sc
        :: types :: ManagedBuffer < CurrentApi >) > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "call_other_contract_execute_on_dest",
        rust_method_name : "call_other_contract_execute_on_dest",
        only_owner : false, only_admin : false, mutability : multiversx_sc
    :: abi :: EndpointMutabilityAbi :: Mutable, endpoint_type :
    multiversx_sc :: abi :: EndpointTypeAbi :: Endpoint,
        payable_in_tokens : & [], inputs : multiversx_sc :: types :: heap
    :: Vec :: new(), outputs : multiversx_sc :: types :: heap :: Vec
    :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types ::
ManagedAddress < CurrentApi > > ("other_sc_address") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    ManagedAddress < CurrentApi > > () ; endpoint_abi.add_output :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > (& []) ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "call_other_contract_add_async_call",
        rust_method_name : "call_other_contract_add_async_call",
        only_owner : false, only_admin : false, mutability : multiversx_sc
    :: abi :: EndpointMutabilityAbi :: Mutable, endpoint_type :
    multiversx_sc :: abi :: EndpointTypeAbi :: Endpoint,
        payable_in_tokens : & [], inputs : multiversx_sc :: types :: heap
    :: Vec :: new(), outputs : multiversx_sc :: types :: heap :: Vec
    :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types ::
ManagedAddress < CurrentApi > > ("other_sc_address") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    ManagedAddress < CurrentApi > > () ; endpoint_abi.add_input :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > ("value") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "getTotalValue", rust_method_name :
    "get_total_value", only_owner : false, only_admin : false,
        mutability : multiversx_sc :: abi :: EndpointMutabilityAbi ::
        Mutable, endpoint_type : multiversx_sc :: abi :: EndpointTypeAbi
    :: Endpoint, payable_in_tokens : & [], inputs : multiversx_sc ::
    types :: heap :: Vec :: new(), outputs : multiversx_sc :: types ::
    heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_output :: < multiversx_sc :: types :: BaseBigUint
< CurrentApi > > (& []) ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "execute_on_dest_add_value", rust_method_name
    : "execute_on_dest_add_value", only_owner : false, only_admin :
    false, mutability : multiversx_sc :: abi :: EndpointMutabilityAbi
    :: Mutable, endpoint_type : multiversx_sc :: abi ::
    EndpointTypeAbi :: Endpoint, payable_in_tokens : & [], inputs :
    multiversx_sc :: types :: heap :: Vec :: new(), outputs :
    multiversx_sc :: types :: heap :: Vec :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types ::
ManagedAddress < CurrentApi > > ("other_sc_address") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    ManagedAddress < CurrentApi > > () ; endpoint_abi.add_input :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > ("value") ;
    contract_abi.add_type_descriptions :: < multiversx_sc :: types ::
    BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "addValue", rust_method_name : "add",
        only_owner : false, only_admin : false, mutability : multiversx_sc
    :: abi :: EndpointMutabilityAbi :: Mutable, endpoint_type :
    multiversx_sc :: abi :: EndpointTypeAbi :: Endpoint,
        payable_in_tokens : & [], inputs : multiversx_sc :: types :: heap
    :: Vec :: new(), outputs : multiversx_sc :: types :: heap :: Vec
    :: new(), labels : & [],
    } ; endpoint_abi.add_input :: < multiversx_sc :: types :: BaseBigUint
< CurrentApi > > ("value") ; contract_abi.add_type_descriptions :: <
    multiversx_sc :: types :: BaseBigUint < CurrentApi > > () ;
    contract_abi.endpoints.push(endpoint_abi) ; let mut endpoint_abi =
    multiversx_sc :: abi :: EndpointAbi
    {
        docs : & [], name : "panic", rust_method_name : "panic",
        only_owner : false, only_admin : false, mutability : multiversx_sc
    :: abi :: EndpointMutabilityAbi :: Mutable, endpoint_type :
    multiversx_sc :: abi :: EndpointTypeAbi :: Endpoint,
        payable_in_tokens : & [], inputs : multiversx_sc :: types :: heap
    :: Vec :: new(), outputs : multiversx_sc :: types :: heap :: Vec
    :: new(), labels : & [],
    } ; contract_abi.endpoints.push(endpoint_abi) ;
    contract_abi.coalesce(< dummy_module :: AbiProvider as multiversx_sc
    :: contract_base :: ContractAbiProvider > :: abi()) ; contract_abi
}
} pub struct ContractObj ; impl multiversx_sc :: contract_base :: ContractBase
< CurrentApi > for ContractObj {} impl dummy_module :: AutoImpl for
ContractObj {} impl AutoImpl for ContractObj {} impl dummy_module ::
EndpointWrappers for ContractObj {} impl EndpointWrappers for ContractObj {}
impl multiversx_sc :: contract_base :: CallableContract for ContractObj
{
    fn call(& self, fn_name : & str) -> bool
    { EndpointWrappers :: call(self, fn_name) }
} pub fn contract_obj() -> ContractObj { ContractObj } pub struct
ContractBuilder ; impl multiversx_sc :: contract_base ::
CallableContractBuilder for self :: ContractBuilder
{
    fn new_contract_obj(& self,) -> multiversx_sc :: types :: heap :: Box <
        dyn multiversx_sc :: contract_base :: CallableContract >
    { multiversx_sc :: types :: heap :: Box :: new(ContractObj) }
} pub use dummy_module :: endpoints as __endpoints_0__ ;
#[allow(non_snake_case)] pub mod endpoints
{
    use super :: EndpointWrappers ; pub use super :: __endpoints_0__ :: * ;
    pub fn init()
    {
        super :: EndpointWrappers ::
        call_init(& multiversx_sc :: contract_base :: UniversalContractObj ::
        < super :: CurrentApi > :: new(),) ;
    } pub fn sum()
{
    super :: EndpointWrappers ::
    call_sum(& multiversx_sc :: contract_base :: UniversalContractObj :: <
        super :: CurrentApi > :: new(),) ;
} pub fn sum_sc_result()
{
    super :: EndpointWrappers ::
    call_sum_sc_result(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn get_caller_legacy()
{
    super :: EndpointWrappers ::
    call_get_caller_legacy(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn get_egld_balance()
{
    super :: EndpointWrappers ::
    call_get_egld_balance(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn get_esdt_balance()
{
    super :: EndpointWrappers ::
    call_get_esdt_balance(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn receive_egld()
{
    super :: EndpointWrappers ::
    call_receive_egld(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn recieve_egld_half()
{
    super :: EndpointWrappers ::
    call_recieve_egld_half(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn receive_esdt()
{
    super :: EndpointWrappers ::
    call_receive_esdt(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn reject_payment()
{
    super :: EndpointWrappers ::
    call_reject_payment(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn receive_esdt_half()
{
    super :: EndpointWrappers ::
    call_receive_esdt_half(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn receive_multi_esdt()
{
    super :: EndpointWrappers ::
    call_receive_multi_esdt(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn send_nft()
{
    super :: EndpointWrappers ::
    call_send_nft(& multiversx_sc :: contract_base :: UniversalContractObj
        :: < super :: CurrentApi > :: new(),) ;
} pub fn mint_esdt()
{
    super :: EndpointWrappers ::
    call_mint_esdt(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn burn_esdt()
{
    super :: EndpointWrappers ::
    call_burn_esdt(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn create_nft()
{
    super :: EndpointWrappers ::
    call_create_nft(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn get_block_epoch()
{
    super :: EndpointWrappers ::
    call_get_block_epoch(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn get_block_nonce()
{
    super :: EndpointWrappers ::
    call_get_block_nonce(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn get_block_timestamp()
{
    super :: EndpointWrappers ::
    call_get_block_timestamp(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn get_random_buffer_once()
{
    super :: EndpointWrappers ::
    call_get_random_buffer_once(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn get_random_buffer_twice()
{
    super :: EndpointWrappers ::
    call_get_random_buffer_twice(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn call_other_contract_execute_on_dest()
{
    super :: EndpointWrappers ::
    call_call_other_contract_execute_on_dest(& multiversx_sc ::
    contract_base :: UniversalContractObj :: < super :: CurrentApi > ::
    new(),) ;
} pub fn call_other_contract_add_async_call()
{
    super :: EndpointWrappers ::
    call_call_other_contract_add_async_call(& multiversx_sc ::
    contract_base :: UniversalContractObj :: < super :: CurrentApi > ::
    new(),) ;
} pub fn get_total_value()
{
    super :: EndpointWrappers ::
    call_get_total_value(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn execute_on_dest_add_value()
{
    super :: EndpointWrappers ::
    call_execute_on_dest_add_value(& multiversx_sc :: contract_base ::
    UniversalContractObj :: < super :: CurrentApi > :: new(),) ;
} pub fn add()
{
    super :: EndpointWrappers ::
    call_add(& multiversx_sc :: contract_base :: UniversalContractObj :: <
        super :: CurrentApi > :: new(),) ;
} pub fn panic()
{
    super :: EndpointWrappers ::
    call_panic(& multiversx_sc :: contract_base :: UniversalContractObj ::
    < super :: CurrentApi > :: new(),) ;
} pub fn callBack()
{
    super :: EndpointWrappers ::
    callback(& multiversx_sc :: contract_base :: UniversalContractObj :: <
        super :: CurrentApi > :: new(),) ;
}
} pub trait ProxyTrait < A > : multiversx_sc :: contract_base :: ProxyObjBase
< A > + Sized + dummy_module :: ProxyTrait < A > where A : multiversx_sc ::
api :: VMApi
{
    #[allow(clippy :: too_many_arguments)] #[allow(clippy :: type_complexity)]
    fn init(& mut self,) -> multiversx_sc :: types :: ContractDeploy < A,
        multiversx_sc :: types :: ManagedBuffer < A > >
    {
        let ___opt_address___ = self.extract_opt_address() ; let mut
    ___contract_deploy___ = multiversx_sc :: types ::
    new_contract_deploy(___opt_address___,) ; ___contract_deploy___
    } #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn sum < Arg0 : multiversx_sc :: codec
:: CodecInto < multiversx_sc :: types :: BaseBigUint < A > >, Arg1 :
multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
BaseBigUint < A > > > (& mut self, first : Arg0, second : Arg1) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, multiversx_sc ::
    types :: BaseBigUint < A > >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "sum",) ; multiversx_sc :: types :: ContractCall
:: proxy_arg(& mut ___contract_call___, & first) ; multiversx_sc ::
types :: ContractCall ::
proxy_arg(& mut ___contract_call___, & second) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn sum_sc_result < Arg0 :
multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
BaseBigUint < A > >, Arg1 : multiversx_sc :: codec :: CodecInto <
    multiversx_sc :: types :: BaseBigUint < A > > >
(& mut self, first : Arg0, second : Arg1) -> multiversx_sc :: types ::
    ContractCallNoPayment < A, multiversx_sc :: types :: BaseBigUint < A > >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "sum_sc_result",) ; multiversx_sc :: types ::
ContractCall :: proxy_arg(& mut ___contract_call___, & first) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & second) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_caller_legacy(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, Address >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "get_caller_legacy",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_egld_balance(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, multiversx_sc ::
    types :: BaseBigUint < A > >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "get_egld_balance",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_esdt_balance < Arg0 :
multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
TokenIdentifier < A > >, Arg1 : multiversx_sc :: codec :: CodecInto < u64
> > (& mut self, token_id : Arg0, nonce : Arg1) -> multiversx_sc :: types
    :: ContractCallNoPayment < A, multiversx_sc :: types :: BaseBigUint < A >
    >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "get_esdt_balance",) ; multiversx_sc :: types ::
ContractCall :: proxy_arg(& mut ___contract_call___, & token_id) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & nonce) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn receive_egld(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, multiversx_sc ::
    types :: BaseBigUint < A > >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "receive_egld",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn recieve_egld_half(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "recieve_egld_half",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn receive_esdt(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A,
        (multiversx_sc :: types :: TokenIdentifier < A >, multiversx_sc :: types
        :: BaseBigUint < A >) >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "receive_esdt",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn reject_payment(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "reject_payment",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn receive_esdt_half(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "receive_esdt_half",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn receive_multi_esdt(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, ManagedVec < A,
        EsdtTokenPayment < A > > >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "receive_multi_esdt",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn send_nft < Arg0 : multiversx_sc ::
codec :: CodecInto < multiversx_sc :: types :: ManagedAddress < A > >,
    Arg1 : multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
    TokenIdentifier < A > >, Arg2 : multiversx_sc :: codec :: CodecInto < u64
    >, Arg3 : multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
    BaseBigUint < A > > >
(& mut self, to : Arg0, token_id : Arg1, nft_nonce : Arg2, amount : Arg3)
 -> multiversx_sc :: types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "send_nft",) ; multiversx_sc :: types ::
ContractCall :: proxy_arg(& mut ___contract_call___, & to) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & token_id) ; multiversx_sc ::
types :: ContractCall ::
proxy_arg(& mut ___contract_call___, & nft_nonce) ; multiversx_sc ::
types :: ContractCall ::
proxy_arg(& mut ___contract_call___, & amount) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn mint_esdt < Arg0 : multiversx_sc ::
codec :: CodecInto < multiversx_sc :: types :: TokenIdentifier < A > >,
    Arg1 : multiversx_sc :: codec :: CodecInto < u64 >, Arg2 : multiversx_sc
    :: codec :: CodecInto < multiversx_sc :: types :: BaseBigUint < A > > >
(& mut self, token_id : Arg0, nonce : Arg1, amount : Arg2) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "mint_esdt",) ; multiversx_sc :: types ::
ContractCall :: proxy_arg(& mut ___contract_call___, & token_id) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & nonce) ; multiversx_sc :: types
:: ContractCall :: proxy_arg(& mut ___contract_call___, & amount) ;
    ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn burn_esdt < Arg0 : multiversx_sc ::
codec :: CodecInto < multiversx_sc :: types :: TokenIdentifier < A > >,
    Arg1 : multiversx_sc :: codec :: CodecInto < u64 >, Arg2 : multiversx_sc
    :: codec :: CodecInto < multiversx_sc :: types :: BaseBigUint < A > > >
(& mut self, token_id : Arg0, nonce : Arg1, amount : Arg2) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "burn_esdt",) ; multiversx_sc :: types ::
ContractCall :: proxy_arg(& mut ___contract_call___, & token_id) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & nonce) ; multiversx_sc :: types
:: ContractCall :: proxy_arg(& mut ___contract_call___, & amount) ;
    ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn create_nft < Arg0 : multiversx_sc
:: codec :: CodecInto < multiversx_sc :: types :: TokenIdentifier < A > >,
    Arg1 : multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
    BaseBigUint < A > >, Arg2 : multiversx_sc :: codec :: CodecInto <
        NftDummyAttributes > >
(& mut self, token_id : Arg0, amount : Arg1, attributes : Arg2) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, u64 >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "create_nft",) ; multiversx_sc :: types ::
ContractCall :: proxy_arg(& mut ___contract_call___, & token_id) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & amount) ; multiversx_sc ::
types :: ContractCall ::
proxy_arg(& mut ___contract_call___, & attributes) ;
    ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_block_epoch(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, u64 >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "get_block_epoch",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_block_nonce(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, u64 >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "get_block_nonce",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_block_timestamp(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, u64 >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "get_block_timestamp",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_random_buffer_once < Arg0 :
multiversx_sc :: codec :: CodecInto < usize > > (& mut self, len : Arg0)
                                                 -> multiversx_sc :: types :: ContractCallNoPayment < A, multiversx_sc ::
                                                 types :: ManagedBuffer < A > >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "get_random_buffer_once",) ; multiversx_sc ::
types :: ContractCall :: proxy_arg(& mut ___contract_call___, & len) ;
    ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_random_buffer_twice < Arg0 :
multiversx_sc :: codec :: CodecInto < usize >, Arg1 : multiversx_sc ::
codec :: CodecInto < usize > > (& mut self, len1 : Arg0, len2 : Arg1) ->
    multiversx_sc :: types :: ContractCallNoPayment < A,
        (multiversx_sc :: types :: ManagedBuffer < A >, multiversx_sc :: types ::
        ManagedBuffer < A >) >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "get_random_buffer_twice",) ; multiversx_sc ::
types :: ContractCall :: proxy_arg(& mut ___contract_call___, & len1)
; multiversx_sc :: types :: ContractCall ::
proxy_arg(& mut ___contract_call___, & len2) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn call_other_contract_execute_on_dest
< Arg0 : multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
ManagedAddress < A > > > (& mut self, other_sc_address : Arg0) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, multiversx_sc ::
    types :: BaseBigUint < A > >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "call_other_contract_execute_on_dest",) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & other_sc_address) ;
    ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn call_other_contract_add_async_call
< Arg0 : multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
ManagedAddress < A > >, Arg1 : multiversx_sc :: codec :: CodecInto <
    multiversx_sc :: types :: BaseBigUint < A > > >
(& mut self, other_sc_address : Arg0, value : Arg1) -> multiversx_sc ::
    types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "call_other_contract_add_async_call",) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & other_sc_address) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & value) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn get_total_value(& mut self,) ->
    multiversx_sc :: types :: ContractCallNoPayment < A, multiversx_sc ::
    types :: BaseBigUint < A > >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "getTotalValue",) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn execute_on_dest_add_value < Arg0 :
multiversx_sc :: codec :: CodecInto < multiversx_sc :: types ::
ManagedAddress < A > >, Arg1 : multiversx_sc :: codec :: CodecInto <
    multiversx_sc :: types :: BaseBigUint < A > > >
(& mut self, other_sc_address : Arg0, value : Arg1) -> multiversx_sc ::
    types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "execute_on_dest_add_value",) ; multiversx_sc ::
types :: ContractCall ::
proxy_arg(& mut ___contract_call___, & other_sc_address) ;
    multiversx_sc :: types :: ContractCall ::
    proxy_arg(& mut ___contract_call___, & value) ; ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn add < Arg0 : multiversx_sc :: codec
:: CodecInto < multiversx_sc :: types :: BaseBigUint < A > > >
(& mut self, value : Arg0) -> multiversx_sc :: types ::
    ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "addValue",) ; multiversx_sc :: types ::
ContractCall :: proxy_arg(& mut ___contract_call___, & value) ;
    ___contract_call___
} #[allow(clippy :: too_many_arguments)]
#[allow(clippy :: type_complexity)] fn panic(& mut self,) -> multiversx_sc
    :: types :: ContractCallNoPayment < A, () >
{
    let ___address___ = self.extract_address() ; let mut
___contract_call___ = multiversx_sc :: types :: ContractCallNoPayment
:: new(___address___, "panic",) ; ___contract_call___
}
} pub struct Proxy < A > where A : multiversx_sc :: api :: VMApi + 'static,
{
    pub address : multiversx_sc :: types :: ManagedOption < A, multiversx_sc
    :: types :: ManagedAddress < A >>,
} impl < A > multiversx_sc :: contract_base :: ProxyObjBase < A > for Proxy <
    A > where A : multiversx_sc :: api :: VMApi + 'static,
{
    fn new_proxy_obj() -> Self
    { Proxy { address : multiversx_sc :: types :: ManagedOption :: none(), } }
    fn
    contract(mut self, address : multiversx_sc :: types :: ManagedAddress < A
    >) -> Self
    {
        self.address = multiversx_sc :: types :: ManagedOption ::
        some(address) ; self
    } fn extract_opt_address(& mut self,) -> multiversx_sc :: types ::
ManagedOption < A, multiversx_sc :: types :: ManagedAddress < A >, >
{
    core :: mem ::
    replace(& mut self.address, multiversx_sc :: types :: ManagedOption ::
    none())
} fn extract_address(& mut self) -> multiversx_sc :: types ::
ManagedAddress < A >
{
    self.extract_opt_address().unwrap_or_sc_panic(multiversx_sc :: err_msg
    :: RECIPIENT_ADDRESS_NOT_SET)
}
} impl < A > dummy_module :: ProxyTrait < A > for Proxy < A > where A :
multiversx_sc :: api :: VMApi {} impl < A > ProxyTrait < A > for Proxy < A >
    where A : multiversx_sc :: api :: VMApi {}

