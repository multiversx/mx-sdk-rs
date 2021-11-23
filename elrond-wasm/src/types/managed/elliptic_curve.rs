use core::marker::PhantomData;

use alloc::string::String;

use crate::{
    abi::TypeAbi,
    api::{Handle, ManagedTypeApi},
};
use elrond_codec::*;

use crate::types::BoxedBytes;

use super::{BigUint, ManagedType};

pub type EllipticCurveComponents<M> = (
    BigUint<M>,
    BigUint<M>,
    BigUint<M>,
    BigUint<M>,
    BigUint<M>,
    u32,
);

#[derive(Debug)]
pub struct EllipticCurve<M: ManagedTypeApi> {
    pub(super) handle: Handle,
    _phantom: PhantomData<M>,
}

impl<M: ManagedTypeApi> ManagedType<M> for EllipticCurve<M> {
    #[doc(hidden)]
    fn from_raw_handle(handle: Handle) -> Self {
        EllipticCurve {
            handle,
            _phantom: PhantomData,
        }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.handle
    }
}

impl<M: ManagedTypeApi> EllipticCurve<M> {
    pub fn from_name(name: &str) -> Self {
        let handle = M::instance().ec_create(name.as_bytes());
        EllipticCurve::from_raw_handle(handle)
    }

    pub fn from_bitsize(bitsize: u32) -> Option<Self> {
        match bitsize {
            224 => Some(Self::from_name("p224")),
            256 => Some(Self::from_name("p256")),
            384 => Some(Self::from_name("p384")),
            521 => Some(Self::from_name("p521")),
            _ => None,
        }
    }

    pub fn get_values(&self) -> EllipticCurveComponents<M> {
        let api = M::instance();
        let field_order_handle = api.bi_new_zero();
        let base_point_order_handle = api.bi_new_zero();
        let eq_constant_handle = api.bi_new_zero();
        let x_base_point_handle = api.bi_new_zero();
        let y_base_point_handle = api.bi_new_zero();
        api.ec_get_values(
            self.handle,
            field_order_handle,
            base_point_order_handle,
            eq_constant_handle,
            x_base_point_handle,
            y_base_point_handle,
        );
        (
            BigUint::from_raw_handle(field_order_handle),
            BigUint::from_raw_handle(base_point_order_handle),
            BigUint::from_raw_handle(eq_constant_handle),
            BigUint::from_raw_handle(x_base_point_handle),
            BigUint::from_raw_handle(y_base_point_handle),
            api.ec_curve_length(self.handle),
        )
    }

    pub fn get_curve_length(&self) -> u32 {
        let api = M::instance();
        api.ec_curve_length(self.handle)
    }

    pub fn get_priv_key_byte_length(&self) -> u32 {
        let api = M::instance();
        api.ec_private_key_byte_length(self.handle)
    }

    pub fn add(
        &self,
        x_first_point: BigUint<M>,
        y_first_point: BigUint<M>,
        x_second_point: BigUint<M>,
        y_second_point: BigUint<M>,
    ) -> (BigUint<M>, BigUint<M>) {
        let api = M::instance();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_add(
            x_result_handle,
            y_result_handle,
            self.handle,
            x_first_point.handle,
            y_first_point.handle,
            x_second_point.handle,
            y_second_point.handle,
        );
        (
            BigUint::from_raw_handle(x_result_handle),
            BigUint::from_raw_handle(y_result_handle),
        )
    }

    pub fn double(&self, x_point: BigUint<M>, y_point: BigUint<M>) -> (BigUint<M>, BigUint<M>) {
        let api = M::instance();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_double(
            x_result_handle,
            y_result_handle,
            self.handle,
            x_point.handle,
            y_point.handle,
        );
        (
            BigUint::from_raw_handle(x_result_handle),
            BigUint::from_raw_handle(y_result_handle),
        )
    }

    pub fn is_on_curve(&self, x_point: BigUint<M>, y_point: BigUint<M>) -> bool {
        let api = M::instance();
        api.ec_is_on_curve(self.handle, x_point.handle, y_point.handle)
    }

    pub fn scalar_mult(
        &self,
        x_point: BigUint<M>,
        y_point: BigUint<M>,
        data: &[u8],
    ) -> (BigUint<M>, BigUint<M>) {
        let api = M::instance();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_scalar_mult(
            x_result_handle,
            y_result_handle,
            self.handle,
            x_point.handle,
            y_point.handle,
            data,
        );
        (
            BigUint::from_raw_handle(x_result_handle),
            BigUint::from_raw_handle(y_result_handle),
        )
    }

    pub fn scalar_base_mult(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        let api = M::instance();
        let x_result_handle = api.bi_new_zero();
        let y_result_handle = api.bi_new_zero();
        api.ec_scalar_base_mult(x_result_handle, y_result_handle, self.handle, data);
        (
            BigUint::from_raw_handle(x_result_handle),
            BigUint::from_raw_handle(y_result_handle),
        )
    }

    pub fn marshal(&self, x_pair: BigUint<M>, y_pair: BigUint<M>) -> BoxedBytes {
        let api = M::instance();
        api.ec_marshal(self.handle, x_pair.handle, y_pair.handle)
    }

    pub fn marshal_compressed(&self, x_pair: BigUint<M>, y_pair: BigUint<M>) -> BoxedBytes {
        let api = M::instance();
        api.ec_marshal_compressed(self.handle, x_pair.handle, y_pair.handle)
    }

    pub fn unmarshal(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        let api = M::instance();
        let x_pair_handle = api.bi_new_zero();
        let y_pair_handle = api.bi_new_zero();
        api.ec_unmarshal(x_pair_handle, y_pair_handle, self.handle, data);
        (
            BigUint::from_raw_handle(x_pair_handle),
            BigUint::from_raw_handle(y_pair_handle),
        )
    }

    pub fn unmarshal_compressed(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        let api = M::instance();
        let x_pair_handle = api.bi_new_zero();
        let y_pair_handle = api.bi_new_zero();
        api.ec_unmarshal_compressed(x_pair_handle, y_pair_handle, self.handle, data);
        (
            BigUint::from_raw_handle(x_pair_handle),
            BigUint::from_raw_handle(y_pair_handle),
        )
    }

    pub fn generate_key(&self) -> (BigUint<M>, BigUint<M>, BoxedBytes) {
        let api = M::instance();
        let x_pub_key_handle = api.bi_new_zero();
        let y_pub_key_handle = api.bi_new_zero();
        let private_key = api.ec_generate_key(x_pub_key_handle, y_pub_key_handle, self.handle);
        (
            BigUint::from_raw_handle(x_pub_key_handle),
            BigUint::from_raw_handle(y_pub_key_handle),
            private_key,
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for EllipticCurve<M> {
    fn dep_encode<O: NestedEncodeOutput>(
        &self,
        dest: &mut O,
    ) -> core::result::Result<(), EncodeError> {
        let (field_order, base_point_order, eq_constant, x_base_point, y_base_point, size_of_field) =
            self.get_values();
        NestedEncode::dep_encode(&field_order, dest)?;
        NestedEncode::dep_encode(&base_point_order, dest)?;
        NestedEncode::dep_encode(&eq_constant, dest)?;
        NestedEncode::dep_encode(&x_base_point, dest)?;
        NestedEncode::dep_encode(&y_base_point, dest)?;
        NestedEncode::dep_encode(&size_of_field, dest)?;
        Ok(())
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        let (field_order, base_point_order, eq_constant, x_base_point, y_base_point, size_of_field) =
            self.get_values();
        NestedEncode::dep_encode_or_exit(&field_order, dest, c.clone(), exit);
        NestedEncode::dep_encode_or_exit(&base_point_order, dest, c.clone(), exit);
        NestedEncode::dep_encode_or_exit(&eq_constant, dest, c.clone(), exit);
        NestedEncode::dep_encode_or_exit(&x_base_point, dest, c.clone(), exit);
        NestedEncode::dep_encode_or_exit(&y_base_point, dest, c.clone(), exit);
        NestedEncode::dep_encode_or_exit(&size_of_field, dest, c, exit);
    }
}

impl<M: ManagedTypeApi> TopEncode for EllipticCurve<M> {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        top_encode_from_nested(self, output)
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        top_encode_from_nested_or_exit(self, output, c, exit);
    }
}

impl<M: ManagedTypeApi> TypeAbi for EllipticCurve<M> {
    fn type_name() -> String {
        String::from("EllipticCurve")
    }
}
