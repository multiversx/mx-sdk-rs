use core::num::NonZero;

use super::TestTokenIdentifier;
use crate::{
    api::ManagedTypeApi,
    contract_base::TransferExecuteFailed,
    types::{
        BigUint, FullPaymentData, FunctionCall, ManagedAddress, Payment, PaymentVec, TxEnv, TxFrom,
        TxPayment, TxPaymentCompose, TxToSpecified,
    },
};

/// A lightweight payment structure designed specifically for use in tests.
///
/// `TestPayment` provides a simple way to create and manipulate payment objects
/// in test scenarios without the overhead of managed types. It automatically
/// ensures that payment amounts are non-zero, preventing common test setup errors.
///
/// # Key Features
///
/// * **Compile-time guarantees**: Uses `NonZero<u128>` to prevent zero-amount payments
/// * **Lightweight**: Uses string slices and primitive types for efficiency in tests
/// * **Composable**: Can be combined with other payments using the `TxPaymentCompose` trait
/// * **Convertible**: Easy conversion to runtime `Payment` objects when needed
///
/// # Usage in Tests
///
/// Use `TestPayment` when writing unit tests or integration tests that need to simulate
/// token transfers. It's particularly useful for:
///
/// * Testing contract endpoints that accept payments
/// * Verifying payment composition and multi-transfer scenarios  
/// * Setting up test scenarios with known payment amounts
/// * Testing payment validation logic
///
/// # Examples
///
/// ## Basic Usage in Blackbox Tests
/// ```ignore
/// use multiversx_sc_scenario::imports::*;
///
/// const TOKEN_1: TestTokenIdentifier = TestTokenIdentifier::new("TOK-000001");
/// const TOKEN_2: TestTokenIdentifier = TestTokenIdentifier::new("TOK-000002");
/// const SFT: TestTokenIdentifier = TestTokenIdentifier::new("SFT-123");
///
/// // Create ESDT payments for testing
/// let token_payment = TestPayment::new(TOKEN_1, 0, 100);
/// let sft_payment = TestPayment::new(SFT, 5, 1); // nonce 5, amount 1
/// ```
///
/// ## Single Payment Contract Calls
/// ```ignore
/// // Test endpoint with single ESDT payment
/// let result = world
///     .tx()
///     .from(USER)
///     .to(CONTRACT_ADDRESS)
///     .typed(contract_proxy::ContractProxy)
///     .payable_endpoint()
///     .payment(TestPayment::new(TOKEN_1, 0, 100))
///     .returns(ReturnsResultUnmanaged)
///     .run();
/// ```
///
/// ## Multiple Payment Contract Calls
/// ```ignore
/// // Test endpoint with multiple payments
/// let result = world
///     .tx()
///     .from(USER)
///     .to(CONTRACT_ADDRESS)
///     .typed(contract_proxy::ContractProxy)
///     .payable_all()
///     .payment(TestPayment::new(TOKEN_1, 0, 100))
///     .payment(TestPayment::new(TOKEN_2, 0, 400))
///     .returns(ReturnsResultUnmanaged)
///     .run();
/// ```
///
/// ## Converting for Assertions
/// ```ignore
/// // Convert TestPayment to managed types for result comparison
/// assert_eq!(
///     result,
///     vec![
///         TestPayment::new(TOKEN_1, 0, 100).to_payment(),
///         TestPayment::new(TOKEN_2, 0, 400).to_payment(),
///     ]
/// );
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TestPayment<'a> {
    token_id: TestTokenIdentifier<'a>,
    nonce: u64,
    amount: NonZero<u128>, // can be changed to NonZeroBigUint if any tests require it
}

impl<'a> TestPayment<'a> {
    pub const fn new(token_id: TestTokenIdentifier<'a>, nonce: u64, amount: u128) -> Self {
        TestPayment {
            token_id,
            nonce,
            amount: NonZero::new(amount).expect("amount must be non-zero"),
        }
    }

    pub fn to_payment<M: ManagedTypeApi>(&self) -> Payment<M> {
        Payment::new(self.token_id.to_token_id(), self.nonce, self.amount.into())
    }
}

impl<'a, Env> TxPayment<Env> for TestPayment<'a>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        // amount is non-zero
        false
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.to_payment()
            .perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        self.to_payment()
            .perform_transfer_execute_legacy(env, to, gas_limit, fc)
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        self.to_payment().with_normalized(env, from, to, fc, f)
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        self.to_payment().into_full_payment_data(env)
    }
}

impl<'a, 'b, Env> TxPaymentCompose<Env, TestPayment<'b>> for TestPayment<'a>
where
    Env: TxEnv,
{
    type Output = PaymentVec<Env::Api>;

    fn compose(self, rhs: TestPayment<'b>) -> Self::Output {
        let mut payments = PaymentVec::new();
        payments.push(self.to_payment());
        payments.push(rhs.to_payment());
        payments
    }
}

impl<'a, Env> TxPaymentCompose<Env, TestPayment<'a>> for PaymentVec<Env::Api>
where
    Env: TxEnv,
{
    type Output = PaymentVec<Env::Api>;

    fn compose(mut self, rhs: TestPayment<'a>) -> Self::Output {
        self.push(rhs.to_payment());
        self
    }
}
