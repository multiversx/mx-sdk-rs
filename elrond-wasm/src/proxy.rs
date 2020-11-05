use super::*;

pub struct OtherContractHandle<T, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: AddAssign<&'b BigUint>,
	for<'b> BigUint: SubAssign<&'b BigUint>,
	for<'b> BigUint: MulAssign<&'b BigUint>,
	for<'b> BigUint: DivAssign<&'b BigUint>,
	for<'b> BigUint: RemAssign<&'b BigUint>,
	for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: BitAndAssign<&'b BigUint>,
	for<'b> BigUint: BitOrAssign<&'b BigUint>,
	for<'b> BigUint: BitXorAssign<&'b BigUint>,
	for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
	for<'a> &'a BigUint: Shl<usize, Output = BigUint>,

	BigInt: BigIntApi<BigUint> + 'static,
	for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
	for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
	for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
	for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
	for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
	for<'b> BigInt: AddAssign<&'b BigInt>,
	for<'b> BigInt: SubAssign<&'b BigInt>,
	for<'b> BigInt: MulAssign<&'b BigInt>,
	for<'b> BigInt: DivAssign<&'b BigInt>,
	for<'b> BigInt: RemAssign<&'b BigInt>,

	T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + Clone + 'static,
{
	pub api: T,
	pub address: Address,
	_phantom1: core::marker::PhantomData<BigInt>,
	_phantom2: core::marker::PhantomData<BigUint>,
}

impl<T, BigInt, BigUint> OtherContractHandle<T, BigInt, BigUint>
where
	BigUint: BigUintApi + 'static,
	for<'a, 'b> &'a BigUint: Add<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: Sub<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: Mul<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: Div<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: Rem<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: AddAssign<&'b BigUint>,
	for<'b> BigUint: SubAssign<&'b BigUint>,
	for<'b> BigUint: MulAssign<&'b BigUint>,
	for<'b> BigUint: DivAssign<&'b BigUint>,
	for<'b> BigUint: RemAssign<&'b BigUint>,
	for<'a, 'b> &'a BigUint: BitAnd<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: BitOr<&'b BigUint, Output = BigUint>,
	for<'a, 'b> &'a BigUint: BitXor<&'b BigUint, Output = BigUint>,
	for<'b> BigUint: BitAndAssign<&'b BigUint>,
	for<'b> BigUint: BitOrAssign<&'b BigUint>,
	for<'b> BigUint: BitXorAssign<&'b BigUint>,
	for<'a> &'a BigUint: Shr<usize, Output = BigUint>,
	for<'a> &'a BigUint: Shl<usize, Output = BigUint>,

	BigInt: BigIntApi<BigUint> + 'static,
	for<'a, 'b> &'a BigInt: Add<&'b BigInt, Output = BigInt>,
	for<'a, 'b> &'a BigInt: Sub<&'b BigInt, Output = BigInt>,
	for<'a, 'b> &'a BigInt: Mul<&'b BigInt, Output = BigInt>,
	for<'a, 'b> &'a BigInt: Div<&'b BigInt, Output = BigInt>,
	for<'a, 'b> &'a BigInt: Rem<&'b BigInt, Output = BigInt>,
	for<'b> BigInt: AddAssign<&'b BigInt>,
	for<'b> BigInt: SubAssign<&'b BigInt>,
	for<'b> BigInt: MulAssign<&'b BigInt>,
	for<'b> BigInt: DivAssign<&'b BigInt>,
	for<'b> BigInt: RemAssign<&'b BigInt>,

	T: ContractHookApi<BigInt, BigUint> + ContractIOApi<BigInt, BigUint> + Clone + 'static,
{
	pub fn new(api: T, address: &Address) -> Self {
		OtherContractHandle {
			api,
			address: address.clone(),
			_phantom1: core::marker::PhantomData,
			_phantom2: core::marker::PhantomData,
		}
	}
}
