use crate::{
    api::{EllipticCurveApi, Handle},
    types::BoxedBytes,
};

impl EllipticCurveApi for super::UncallableApi {
    fn ec_create(&self, _name: &[u8]) -> Handle {
        unreachable!()
    }

    fn ec_get_values(
        &self,
        _ec_handle: Handle,
        _field_order_handle: Handle,
        _base_point_order_handle: Handle,
        _eq_constant_handle: Handle,
        _x_base_point_handle: Handle,
        _y_base_point_handle: Handle,
    ) {
        unreachable!()
    }

    fn ec_curve_length(&self, _ec_handle: Handle) -> u32 {
        unreachable!()
    }

    fn ec_private_key_byte_length(&self, _ec_handle: Handle) -> u32 {
        unreachable!()
    }

    fn ec_add(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _x_first_point: Handle,
        _y_first_point: Handle,
        _x_second_point: Handle,
        _y_second_point: Handle,
    ) {
        unreachable!()
    }

    fn ec_double(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _x_point_handle: Handle,
        _y_point_handle: Handle,
    ) {
        unreachable!()
    }

    fn ec_is_on_curve(
        &self,
        _ec_handle: Handle,
        _x_point_handle: Handle,
        _y_point_handle: Handle,
    ) -> bool {
        unreachable!()
    }

    fn ec_scalar_mult(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _x_point_handle: Handle,
        _y_point_handle: Handle,
        _data: &[u8],
    ) {
        unreachable!()
    }

    fn ec_scalar_base_mult(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _data: &[u8],
    ) {
        unreachable!()
    }

    fn ec_marshal(
        &self,
        _ec_handle: Handle,
        _x_pair_handle: Handle,
        _y_pair_handle: Handle,
    ) -> BoxedBytes {
        unreachable!()
    }

    fn ec_marshal_compressed(
        &self,
        _ec_handle: Handle,
        _x_pair_handle: Handle,
        _y_pair_handle: Handle,
    ) -> BoxedBytes {
        unreachable!()
    }

    fn ec_unmarshal(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _data: &[u8],
    ) {
        unreachable!()
    }

    fn ec_unmarshal_compressed(
        &self,
        _x_result_handle: Handle,
        _y_result_handle: Handle,
        _ec_handle: Handle,
        _data: &[u8],
    ) {
        unreachable!()
    }

    fn ec_generate_key(
        &self,
        _x_pub_key_handle: Handle,
        _y_pub_key_handle: Handle,
        _ec_handle: Handle,
    ) -> BoxedBytes {
        unreachable!()
    }
}
