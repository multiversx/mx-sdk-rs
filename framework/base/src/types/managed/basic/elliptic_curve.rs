use crate::{
    abi::{TypeAbi, TypeName},
    api::{use_raw_handle, BigIntApiImpl, EllipticCurveApiImpl, ManagedTypeApi},
    types::{BigUint, ManagedType},
};

use crate::{api::StaticVarApiImpl, types::ManagedBuffer};

use crate::codec::*;

pub const ELLIPTIC_CURVE_P224_INT: u32 = 224;
pub const ELLIPTIC_CURVE_P224_NAME: &str = "p224";
pub const ELLIPTIC_CURVE_P256_INT: u32 = 256;
pub const ELLIPTIC_CURVE_P256_NAME: &str = "p256";
pub const ELLIPTIC_CURVE_P384_INT: u32 = 384;
pub const ELLIPTIC_CURVE_P384_NAME: &str = "p384";
pub const ELLIPTIC_CURVE_P521_INT: u32 = 521;
pub const ELLIPTIC_CURVE_P521_NAME: &str = "p521";

pub type EllipticCurveComponents<M> = (
    BigUint<M>,
    BigUint<M>,
    BigUint<M>,
    BigUint<M>,
    BigUint<M>,
    u32,
);

#[repr(transparent)]
#[derive(Debug)]
pub struct EllipticCurve<M: ManagedTypeApi> {
    pub(super) handle: M::EllipticCurveHandle,
}

impl<M: ManagedTypeApi> ManagedType<M> for EllipticCurve<M> {
    type OwnHandle = M::EllipticCurveHandle;

    fn from_handle(handle: M::EllipticCurveHandle) -> Self {
        EllipticCurve { handle }
    }

    fn get_handle(&self) -> M::EllipticCurveHandle {
        self.handle.clone()
    }

    fn transmute_from_handle_ref(handle_ref: &M::EllipticCurveHandle) -> &Self {
        unsafe { core::mem::transmute(handle_ref) }
    }
}

impl<M: ManagedTypeApi> EllipticCurve<M> {
    pub fn from_name(name: &ManagedBuffer<M>) -> Self {
        let handle = M::managed_type_impl().ec_create_from_name_mb(name.get_handle());
        EllipticCurve::from_handle(handle)
    }

    pub fn from_name_str(name: &str) -> Self {
        let handle = M::managed_type_impl().ec_create_from_name_bytes(name.as_bytes());
        EllipticCurve::from_handle(handle)
    }

    pub fn from_bitsize(bitsize: u32) -> Option<Self> {
        match bitsize {
            ELLIPTIC_CURVE_P224_INT => Some(Self::from_name_str(ELLIPTIC_CURVE_P224_NAME)),
            ELLIPTIC_CURVE_P256_INT => Some(Self::from_name_str(ELLIPTIC_CURVE_P256_NAME)),
            ELLIPTIC_CURVE_P384_INT => Some(Self::from_name_str(ELLIPTIC_CURVE_P384_NAME)),
            ELLIPTIC_CURVE_P521_INT => Some(Self::from_name_str(ELLIPTIC_CURVE_P521_NAME)),
            _ => None,
        }
    }

    pub fn get_values(&self) -> EllipticCurveComponents<M> {
        let api = M::managed_type_impl();
        let field_order_handle = api.bi_new_zero();
        let base_point_order_handle = api.bi_new_zero();
        let eq_constant_handle = api.bi_new_zero();
        let x_base_point_handle = api.bi_new_zero();
        let y_base_point_handle = api.bi_new_zero();
        api.ec_get_values(
            self.handle.clone(),
            field_order_handle.clone(),
            base_point_order_handle.clone(),
            eq_constant_handle.clone(),
            x_base_point_handle.clone(),
            y_base_point_handle.clone(),
        );
        (
            BigUint::from_handle(field_order_handle),
            BigUint::from_handle(base_point_order_handle),
            BigUint::from_handle(eq_constant_handle),
            BigUint::from_handle(x_base_point_handle),
            BigUint::from_handle(y_base_point_handle),
            api.ec_curve_length(self.handle.clone()),
        )
    }

    pub fn get_curve_length(&self) -> u32 {
        let api = M::managed_type_impl();
        api.ec_curve_length(self.handle.clone())
    }

    pub fn get_priv_key_byte_length(&self) -> u32 {
        let api = M::managed_type_impl();
        api.ec_private_key_byte_length(self.handle.clone())
    }

    pub fn add(
        &self,
        x_first_point: BigUint<M>,
        y_first_point: BigUint<M>,
        x_second_point: BigUint<M>,
        y_second_point: BigUint<M>,
    ) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_add(
            x_result_handle.clone(),
            y_result_handle.clone(),
            self.handle.clone(),
            x_first_point.handle,
            y_first_point.handle,
            x_second_point.handle,
            y_second_point.handle,
        );
        (
            BigUint::from_handle(x_result_handle),
            BigUint::from_handle(y_result_handle),
        )
    }

    pub fn double(&self, x_point: BigUint<M>, y_point: BigUint<M>) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_double(
            x_result_handle.clone(),
            y_result_handle.clone(),
            self.handle.clone(),
            x_point.handle,
            y_point.handle,
        );
        (
            BigUint::from_handle(x_result_handle),
            BigUint::from_handle(y_result_handle),
        )
    }

    pub fn is_on_curve(&self, x_point: BigUint<M>, y_point: BigUint<M>) -> bool {
        let api = M::managed_type_impl();
        api.ec_is_on_curve(self.handle.clone(), x_point.handle, y_point.handle)
    }

    #[deprecated(since = "0.41.0", note = "Please use method `scalar_mult` instead.")]
    pub fn scalar_mult_legacy(
        &self,
        x_point: BigUint<M>,
        y_point: BigUint<M>,
        data: &[u8],
    ) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_scalar_mult_legacy(
            x_result_handle.clone(),
            y_result_handle.clone(),
            self.handle.clone(),
            x_point.handle,
            y_point.handle,
            data,
        );
        (
            BigUint::from_handle(x_result_handle),
            BigUint::from_handle(y_result_handle),
        )
    }

    pub fn scalar_mult(
        &self,
        x_point: BigUint<M>,
        y_point: BigUint<M>,
        data: &ManagedBuffer<M>,
    ) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_scalar_mult(
            x_result_handle.clone(),
            y_result_handle.clone(),
            self.handle.clone(),
            x_point.handle,
            y_point.handle,
            data.get_handle(),
        );
        (
            BigUint::from_handle(x_result_handle),
            BigUint::from_handle(y_result_handle),
        )
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `scalar_base_mult` instead."
    )]
    pub fn scalar_base_mult_legacy(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_scalar_base_mult_legacy(
            x_result_handle.clone(),
            y_result_handle.clone(),
            self.handle.clone(),
            data,
        );
        (
            BigUint::from_handle(x_result_handle),
            BigUint::from_handle(y_result_handle),
        )
    }

    pub fn scalar_base_mult(&self, data: &ManagedBuffer<M>) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_scalar_base_mult(
            x_result_handle.clone(),
            y_result_handle.clone(),
            self.handle.clone(),
            data.get_handle(),
        );
        (
            BigUint::from_handle(x_result_handle),
            BigUint::from_handle(y_result_handle),
        )
    }

    #[deprecated(since = "0.41.0", note = "Please use method `marshal` instead.")]
    #[cfg(feature = "alloc")]
    pub fn marshal_legacy(
        &self,
        x_pair: BigUint<M>,
        y_pair: BigUint<M>,
    ) -> crate::types::heap::BoxedBytes {
        let api = M::managed_type_impl();
        api.ec_marshal_legacy(self.handle.clone(), x_pair.handle, y_pair.handle)
    }

    pub fn marshal(&self, x_pair: BigUint<M>, y_pair: BigUint<M>) -> ManagedBuffer<M> {
        let result_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().ec_marshal(
            self.handle.clone(),
            x_pair.handle,
            y_pair.handle,
            result_handle.clone(),
        );
        ManagedBuffer::from_handle(result_handle)
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `marshal_compressed` instead."
    )]
    #[cfg(feature = "alloc")]
    pub fn marshal_compressed_legacy(
        &self,
        x_pair: BigUint<M>,
        y_pair: BigUint<M>,
    ) -> crate::types::heap::BoxedBytes {
        let api = M::managed_type_impl();
        api.ec_marshal_compressed_legacy(self.handle.clone(), x_pair.handle, y_pair.handle)
    }

    pub fn marshal_compressed(&self, x_pair: BigUint<M>, y_pair: BigUint<M>) -> ManagedBuffer<M> {
        let result_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().ec_marshal_compressed(
            self.handle.clone(),
            x_pair.handle,
            y_pair.handle,
            result_handle.clone(),
        );
        ManagedBuffer::from_handle(result_handle)
    }

    #[deprecated(since = "0.41.0", note = "Please use method `unmarshal` instead.")]
    pub fn unmarshal_legacy(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_pair_handle = api.bi_new_zero();
        let y_pair_handle = api.bi_new_zero();
        api.ec_unmarshal_legacy(
            x_pair_handle.clone(),
            y_pair_handle.clone(),
            self.handle.clone(),
            data,
        );
        (
            BigUint::from_handle(x_pair_handle),
            BigUint::from_handle(y_pair_handle),
        )
    }

    pub fn unmarshal(&self, data: &ManagedBuffer<M>) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_pair_handle = api.bi_new_zero();
        let y_pair_handle = api.bi_new_zero();
        api.ec_unmarshal(
            x_pair_handle.clone(),
            y_pair_handle.clone(),
            self.handle.clone(),
            data.get_handle(),
        );
        (
            BigUint::from_handle(x_pair_handle),
            BigUint::from_handle(y_pair_handle),
        )
    }

    #[deprecated(
        since = "0.41.0",
        note = "Please use method `unmarshal_compressed` instead."
    )]
    pub fn unmarshal_compressed_legacy(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_pair_handle = api.bi_new_zero();
        let y_pair_handle = api.bi_new_zero();
        api.ec_unmarshal_compressed_legacy(
            x_pair_handle.clone(),
            y_pair_handle.clone(),
            self.handle.clone(),
            data,
        );
        (
            BigUint::from_handle(x_pair_handle),
            BigUint::from_handle(y_pair_handle),
        )
    }

    pub fn unmarshal_compressed(&self, data: &ManagedBuffer<M>) -> (BigUint<M>, BigUint<M>) {
        let api = M::managed_type_impl();
        let x_pair_handle = api.bi_new_zero();
        let y_pair_handle = api.bi_new_zero();
        api.ec_unmarshal_compressed(
            x_pair_handle.clone(),
            y_pair_handle.clone(),
            self.handle.clone(),
            data.get_handle(),
        );
        (
            BigUint::from_handle(x_pair_handle),
            BigUint::from_handle(y_pair_handle),
        )
    }

    #[deprecated(since = "0.41.0", note = "Please use method `generate_key` instead.")]
    #[cfg(feature = "alloc")]
    pub fn generate_key_legacy(&self) -> (BigUint<M>, BigUint<M>, crate::types::heap::BoxedBytes) {
        let api = M::managed_type_impl();
        let x_pub_key_handle = api.bi_new_zero();
        let y_pub_key_handle = api.bi_new_zero();
        let private_key = api.ec_generate_key_legacy(
            x_pub_key_handle.clone(),
            y_pub_key_handle.clone(),
            self.handle.clone(),
        );
        (
            BigUint::from_handle(x_pub_key_handle),
            BigUint::from_handle(y_pub_key_handle),
            private_key,
        )
    }

    pub fn generate_key(&self) -> (BigUint<M>, BigUint<M>, ManagedBuffer<M>) {
        let api = M::managed_type_impl();
        let x_pub_key_handle = api.bi_new_zero();
        let y_pub_key_handle = api.bi_new_zero();
        let private_key_handle: M::ManagedBufferHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        api.ec_generate_key(
            x_pub_key_handle.clone(),
            y_pub_key_handle.clone(),
            self.handle.clone(),
            private_key_handle.clone(),
        );
        (
            BigUint::from_handle(x_pub_key_handle),
            BigUint::from_handle(y_pub_key_handle),
            ManagedBuffer::from_handle(private_key_handle),
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for EllipticCurve<M> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        let (field_order, base_point_order, eq_constant, x_base_point, y_base_point, size_of_field) =
            self.get_values();
        NestedEncode::dep_encode_or_handle_err(&field_order, dest, h)?;
        NestedEncode::dep_encode_or_handle_err(&base_point_order, dest, h)?;
        NestedEncode::dep_encode_or_handle_err(&eq_constant, dest, h)?;
        NestedEncode::dep_encode_or_handle_err(&x_base_point, dest, h)?;
        NestedEncode::dep_encode_or_handle_err(&y_base_point, dest, h)?;
        NestedEncode::dep_encode_or_handle_err(&size_of_field, dest, h)?;
        Ok(())
    }
}

impl<M: ManagedTypeApi> TopEncode for EllipticCurve<M> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        top_encode_from_nested(self, output, h)
    }
}

impl<M: ManagedTypeApi> TypeAbi for EllipticCurve<M> {
    fn type_name() -> TypeName {
        TypeName::from("EllipticCurve")
    }
}
