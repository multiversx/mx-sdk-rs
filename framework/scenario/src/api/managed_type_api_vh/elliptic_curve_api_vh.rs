use multiversx_sc::{api::EllipticCurveApiImpl, types::BoxedBytes};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> EllipticCurveApiImpl for VMHooksApi<VHB> {
    fn ec_create_from_name_bytes(&self, _name: &[u8]) -> Self::EllipticCurveHandle {
        panic!("ec_create not implemented")
    }

    fn ec_create_from_name_mb(
        &self,
        _name_handle: Self::ManagedBufferHandle,
    ) -> Self::EllipticCurveHandle {
        panic!("ec_create not implemented")
    }

    fn ec_get_values(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _field_order_handle: Self::BigIntHandle,
        _base_point_order_handle: Self::BigIntHandle,
        _eq_constant_handle: Self::BigIntHandle,
        _x_base_point_handle: Self::BigIntHandle,
        _y_base_point_handle: Self::BigIntHandle,
    ) {
        panic!("ec_get_values not implemented")
    }

    fn ec_curve_length(&self, _ec_handle: Self::EllipticCurveHandle) -> u32 {
        panic!("ec_curve_length not implemented")
    }

    fn ec_private_key_byte_length(&self, _ec_handle: Self::EllipticCurveHandle) -> u32 {
        panic!("ec_private_key_byte_length not implemented")
    }

    fn ec_add(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _x_first_point: Self::BigIntHandle,
        _y_first_point: Self::BigIntHandle,
        _x_second_point: Self::BigIntHandle,
        _y_second_point: Self::BigIntHandle,
    ) {
        panic!("ec_add not implemented")
    }

    fn ec_double(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _x_point_handle: Self::BigIntHandle,
        _y_point_handle: Self::BigIntHandle,
    ) {
        panic!("ec_double not implemented")
    }

    fn ec_is_on_curve(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_point_handle: Self::BigIntHandle,
        _y_point_handle: Self::BigIntHandle,
    ) -> bool {
        panic!("ec_is_on_curve not implemented")
    }

    fn ec_scalar_mult_legacy(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _x_point_handle: Self::BigIntHandle,
        _y_point_handle: Self::BigIntHandle,
        _data: &[u8],
    ) {
        panic!("ec_scalar_mult not implemented")
    }

    fn ec_scalar_mult(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _x_point_handle: Self::BigIntHandle,
        _y_point_handle: Self::BigIntHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ec_scalar_mult not implemented")
    }

    fn ec_scalar_base_mult_legacy(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data: &[u8],
    ) {
        panic!("ec_scalar_base_mult not implemented")
    }

    fn ec_scalar_base_mult(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ec_scalar_base_mult not implemented")
    }

    fn ec_marshal_legacy(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_pair_handle: Self::BigIntHandle,
        _y_pair_handle: Self::BigIntHandle,
    ) -> BoxedBytes {
        panic!("ec_marshal not implemented")
    }

    fn ec_marshal(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_pair_handle: Self::BigIntHandle,
        _y_pair_handle: Self::BigIntHandle,
        _result_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ec_marshal not implemented")
    }

    fn ec_marshal_compressed_legacy(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_pair_handle: Self::BigIntHandle,
        _y_pair_handle: Self::BigIntHandle,
    ) -> BoxedBytes {
        panic!("ec_marshal_compressed not implemented")
    }

    fn ec_marshal_compressed(
        &self,
        _ec_handle: Self::EllipticCurveHandle,
        _x_pair_handle: Self::BigIntHandle,
        _y_pair_handle: Self::BigIntHandle,
        _result_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ec_marshal_compressed not implemented")
    }

    fn ec_unmarshal_legacy(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data: &[u8],
    ) {
        panic!("ec_unmarshal not implemented")
    }

    fn ec_unmarshal(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ec_unmarshal not implemented")
    }

    fn ec_unmarshal_compressed_legacy(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data: &[u8],
    ) {
        panic!("ec_unmarshal_compressed not implemented")
    }

    fn ec_unmarshal_compressed(
        &self,
        _x_result_handle: Self::BigIntHandle,
        _y_result_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ec_unmarshal_compressed not implemented")
    }

    fn ec_generate_key_legacy(
        &self,
        _x_pub_key_handle: Self::BigIntHandle,
        _y_pub_key_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
    ) -> BoxedBytes {
        panic!("ec_generate_key not implemented")
    }

    fn ec_generate_key(
        &self,
        _x_pub_key_handle: Self::BigIntHandle,
        _y_pub_key_handle: Self::BigIntHandle,
        _ec_handle: Self::EllipticCurveHandle,
        _result_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ec_generate_key not implemented")
    }
}
