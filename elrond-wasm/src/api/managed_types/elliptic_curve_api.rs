use crate::types::heap::BoxedBytes;

use super::Handle;

/// Wrapper around the EllipticCurve functionality provided by Arwen.
pub trait EllipticCurveApi {
    fn ec_create_from_name_bytes(&self, name: &[u8]) -> Handle;

    fn ec_create_from_name_mb(&self, name_handle: Handle) -> Handle;

    fn ec_get_values(
        &self,
        ec_handle: Handle,
        field_order_handle: Handle,
        base_point_order_handle: Handle,
        eq_constant_handle: Handle,
        x_base_point_handle: Handle,
        y_base_point_handle: Handle,
    );

    fn ec_curve_length(&self, ec_handle: Handle) -> u32;

    fn ec_private_key_byte_length(&self, ec_handle: Handle) -> u32;

    #[allow(clippy::too_many_arguments)]
    fn ec_add(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        x_first_point: Handle,
        y_first_point: Handle,
        x_second_point: Handle,
        y_second_point: Handle,
    );

    fn ec_double(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        x_point_handle: Handle,
        y_point_handle: Handle,
    );

    fn ec_is_on_curve(
        &self,
        ec_handle: Handle,
        x_point_handle: Handle,
        y_point_handle: Handle,
    ) -> bool;

    fn ec_scalar_mult_legacy(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        x_point_handle: Handle,
        y_point_handle: Handle,
        data: &[u8],
    );

    fn ec_scalar_mult(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        x_point_handle: Handle,
        y_point_handle: Handle,
        data_handle: Handle,
    );

    fn ec_scalar_base_mult_legacy(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data: &[u8],
    );

    fn ec_scalar_base_mult(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data_handle: Handle,
    );

    fn ec_marshal_legacy(
        &self,
        ec_handle: Handle,
        x_pair_handle: Handle,
        y_pair_handle: Handle,
    ) -> BoxedBytes;

    fn ec_marshal(
        &self,
        ec_handle: Handle,
        x_pair_handle: Handle,
        y_pair_handle: Handle,
        result_handle: Handle,
    );

    fn ec_marshal_compressed_legacy(
        &self,
        ec_handle: Handle,
        x_pair_handle: Handle,
        y_pair_handle: Handle,
    ) -> BoxedBytes;

    fn ec_marshal_compressed(
        &self,
        ec_handle: Handle,
        x_pair_handle: Handle,
        y_pair_handle: Handle,
        result_handle: Handle,
    );

    fn ec_unmarshal_legacy(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data: &[u8],
    );

    fn ec_unmarshal(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data_handle: Handle,
    );

    fn ec_unmarshal_compressed_legacy(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data: &[u8],
    );

    fn ec_unmarshal_compressed(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data_handle: Handle,
    );

    fn ec_generate_key_legacy(
        &self,
        x_pub_key_handle: Handle,
        y_pub_key_handle: Handle,
        ec_handle: Handle,
    ) -> BoxedBytes;

    fn ec_generate_key(
        &self,
        x_pub_key_handle: Handle,
        y_pub_key_handle: Handle,
        ec_handle: Handle,
        result_handle: Handle,
    );
}
