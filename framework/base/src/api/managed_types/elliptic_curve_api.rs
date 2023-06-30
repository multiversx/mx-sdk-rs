use crate::types::heap::BoxedBytes;

use super::HandleTypeInfo;

/// Wrapper around the EllipticCurve functionality provided by Arwen.
pub trait EllipticCurveApiImpl: HandleTypeInfo {
    fn ec_create_from_name_bytes(&self, name: &[u8]) -> Self::EllipticCurveHandle;

    fn ec_create_from_name_mb(
        &self,
        name_handle: Self::ManagedBufferHandle,
    ) -> Self::EllipticCurveHandle;

    fn ec_get_values(
        &self,
        ec_handle: Self::EllipticCurveHandle,
        field_order_handle: Self::BigIntHandle,
        base_point_order_handle: Self::BigIntHandle,
        eq_constant_handle: Self::BigIntHandle,
        x_base_point_handle: Self::BigIntHandle,
        y_base_point_handle: Self::BigIntHandle,
    );

    fn ec_curve_length(&self, ec_handle: Self::EllipticCurveHandle) -> u32;

    fn ec_private_key_byte_length(&self, ec_handle: Self::EllipticCurveHandle) -> u32;

    #[allow(clippy::too_many_arguments)]
    fn ec_add(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        x_first_point: Self::BigIntHandle,
        y_first_point: Self::BigIntHandle,
        x_second_point: Self::BigIntHandle,
        y_second_point: Self::BigIntHandle,
    );

    fn ec_double(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        x_point_handle: Self::BigIntHandle,
        y_point_handle: Self::BigIntHandle,
    );

    fn ec_is_on_curve(
        &self,
        ec_handle: Self::EllipticCurveHandle,
        x_point_handle: Self::BigIntHandle,
        y_point_handle: Self::BigIntHandle,
    ) -> bool;

    fn ec_scalar_mult_legacy(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        x_point_handle: Self::BigIntHandle,
        y_point_handle: Self::BigIntHandle,
        data: &[u8],
    );

    fn ec_scalar_mult(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        x_point_handle: Self::BigIntHandle,
        y_point_handle: Self::BigIntHandle,
        data_handle: Self::ManagedBufferHandle,
    );

    fn ec_scalar_base_mult_legacy(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        data: &[u8],
    );

    fn ec_scalar_base_mult(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        data_handle: Self::ManagedBufferHandle,
    );

    fn ec_marshal_legacy(
        &self,
        ec_handle: Self::EllipticCurveHandle,
        x_pair_handle: Self::BigIntHandle,
        y_pair_handle: Self::BigIntHandle,
    ) -> BoxedBytes;

    fn ec_marshal(
        &self,
        ec_handle: Self::EllipticCurveHandle,
        x_pair_handle: Self::BigIntHandle,
        y_pair_handle: Self::BigIntHandle,
        result_handle: Self::ManagedBufferHandle,
    );

    fn ec_marshal_compressed_legacy(
        &self,
        ec_handle: Self::EllipticCurveHandle,
        x_pair_handle: Self::BigIntHandle,
        y_pair_handle: Self::BigIntHandle,
    ) -> BoxedBytes;

    fn ec_marshal_compressed(
        &self,
        ec_handle: Self::EllipticCurveHandle,
        x_pair_handle: Self::BigIntHandle,
        y_pair_handle: Self::BigIntHandle,
        result_handle: Self::ManagedBufferHandle,
    );

    fn ec_unmarshal_legacy(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        data: &[u8],
    );

    fn ec_unmarshal(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        data_handle: Self::ManagedBufferHandle,
    );

    fn ec_unmarshal_compressed_legacy(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        data: &[u8],
    );

    fn ec_unmarshal_compressed(
        &self,
        x_result_handle: Self::BigIntHandle,
        y_result_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        data_handle: Self::ManagedBufferHandle,
    );

    fn ec_generate_key_legacy(
        &self,
        x_pub_key_handle: Self::BigIntHandle,
        y_pub_key_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
    ) -> BoxedBytes;

    fn ec_generate_key(
        &self,
        x_pub_key_handle: Self::BigIntHandle,
        y_pub_key_handle: Self::BigIntHandle,
        ec_handle: Self::EllipticCurveHandle,
        result_handle: Self::ManagedBufferHandle,
    );
}
