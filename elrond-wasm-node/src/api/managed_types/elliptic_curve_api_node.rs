use elrond_wasm::{
    api::{EllipticCurveApi, Handle},
    types::BoxedBytes,
};

extern "C" {
    fn createEC(dataOffset: i32, dataLength: i32) -> i32;

    fn ellipticCurveGetValues(
        ecHandle: i32,
        fieldOrderHandle: i32,
        basePointOrderHandle: i32,
        eqConstantHandle: i32,
        xBasePointHandle: i32,
        yBasePointHandle: i32,
    ) -> i32;

    fn getCurveLengthEC(ecHandle: i32) -> i32;

    fn getPrivKeyByteLengthEC(ecHandle: i32) -> i32;

    fn addEC(
        xResultHandle: i32,
        yResultHandle: i32,
        ecHandle: i32,
        fstPointXHandle: i32,
        fstPointYHandle: i32,
        sndPointXHandle: i32,
        sndPointYHandle: i32,
    );

    fn doubleEC(
        xResultHandle: i32,
        yResultHandle: i32,
        ecHandle: i32,
        pointXHandle: i32,
        pointYHandle: i32,
    );

    fn isOnCurveEC(ecHandle: i32, pointXHandle: i32, pointYHandle: i32) -> i32;

    fn scalarMultEC(
        xResultHandle: i32,
        yResultHandle: i32,
        ecHandle: i32,
        pointXHandle: i32,
        pointYHandle: i32,
        dataOffset: *const u8,
        length: i32,
    ) -> i32;

    fn scalarBaseMultEC(
        xResultHandle: i32,
        yResultHandle: i32,
        ecHandle: i32,
        dataOffset: *const u8,
        length: i32,
    ) -> i32;

    fn marshalEC(xPairHandle: i32, yPairHandle: i32, ecHandle: i32, resultOffset: *mut u8) -> i32;

    fn marshalCompressedEC(
        xPairHandle: i32,
        yPairHandle: i32,
        ecHandle: i32,
        resultOffset: *mut u8,
    ) -> i32;

    fn unmarshalEC(
        xResultHandle: i32,
        yResultHandle: i32,
        ecHandle: i32,
        dataOffset: *const u8,
        length: i32,
    ) -> i32;

    fn unmarshalCompressedEC(
        xResultHandle: i32,
        yResultHandle: i32,
        ecHandle: i32,
        dataOffset: *const u8,
        length: i32,
    ) -> i32;

    fn generateKeyEC(
        xPubKeyHandle: i32,
        yPubKeyHandle: i32,
        ecHandle: i32,
        resultOffset: *mut u8,
    ) -> i32;

}

impl EllipticCurveApi for crate::VmApiImpl {
    fn ec_create(&self, name: &[u8]) -> Handle {
        unsafe { createEC(name.as_ptr() as i32, name.len() as i32) }
    }

    fn ec_get_values(
        &self,
        ec_handle: Handle,
        field_order_handle: Handle,
        base_point_order_handle: Handle,
        eq_constant_handle: Handle,
        x_base_point_handle: Handle,
        y_base_point_handle: Handle,
    ) {
        unsafe {
            let _ = ellipticCurveGetValues(
                ec_handle,
                field_order_handle,
                base_point_order_handle,
                eq_constant_handle,
                x_base_point_handle,
                y_base_point_handle,
            );
        }
    }

    fn ec_curve_length(&self, ec_handle: Handle) -> u32 {
        unsafe { getCurveLengthEC(ec_handle) as u32 }
    }

    fn ec_private_key_byte_length(&self, ec_handle: Handle) -> u32 {
        unsafe { getPrivKeyByteLengthEC(ec_handle) as u32 }
    }

    fn ec_add(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        x_first_point: Handle,
        y_first_point: Handle,
        x_second_point: Handle,
        y_second_point: Handle,
    ) {
        unsafe {
            addEC(
                x_result_handle,
                y_result_handle,
                ec_handle,
                x_first_point,
                y_first_point,
                x_second_point,
                y_second_point,
            );
        }
    }

    fn ec_double(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        x_point_handle: Handle,
        y_point_handle: Handle,
    ) {
        unsafe {
            doubleEC(
                x_result_handle,
                y_result_handle,
                ec_handle,
                x_point_handle,
                y_point_handle,
            );
        }
    }

    fn ec_is_on_curve(
        &self,
        ec_handle: Handle,
        x_point_handle: Handle,
        y_point_handle: Handle,
    ) -> bool {
        unsafe { isOnCurveEC(ec_handle, x_point_handle, y_point_handle) > 0 }
    }

    fn ec_scalar_mult(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        x_point_handle: Handle,
        y_point_handle: Handle,
        data: &[u8],
    ) {
        unsafe {
            scalarMultEC(
                x_result_handle,
                y_result_handle,
                ec_handle,
                x_point_handle,
                y_point_handle,
                data.as_ptr(),
                data.len() as i32,
            );
        }
    }

    fn ec_scalar_base_mult(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data: &[u8],
    ) {
        unsafe {
            scalarBaseMultEC(
                x_result_handle,
                y_result_handle,
                ec_handle,
                data.as_ptr(),
                data.len() as i32,
            );
        }
    }

    fn ec_marshal(
        &self,
        ec_handle: Handle,
        x_pair_handle: Handle,
        y_pair_handle: Handle,
    ) -> BoxedBytes {
        unsafe {
            let byte_length = (getCurveLengthEC(ec_handle) + 7) / 8;
            let mut result = BoxedBytes::allocate(1 + 2 * byte_length as usize);
            marshalEC(x_pair_handle, y_pair_handle, ec_handle, result.as_mut_ptr());
            result
        }
    }

    fn ec_marshal_compressed(
        &self,
        ec_handle: Handle,
        x_pair_handle: Handle,
        y_pair_handle: Handle,
    ) -> BoxedBytes {
        unsafe {
            let byte_length = (getCurveLengthEC(ec_handle) + 7) / 8;
            let mut result = BoxedBytes::allocate(1 + byte_length as usize);
            marshalCompressedEC(x_pair_handle, y_pair_handle, ec_handle, result.as_mut_ptr());
            result
        }
    }

    fn ec_unmarshal(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data: &[u8],
    ) {
        unsafe {
            unmarshalEC(
                x_result_handle,
                y_result_handle,
                ec_handle,
                data.as_ptr(),
                data.len() as i32,
            );
        }
    }

    fn ec_unmarshal_compressed(
        &self,
        x_result_handle: Handle,
        y_result_handle: Handle,
        ec_handle: Handle,
        data: &[u8],
    ) {
        unsafe {
            unmarshalCompressedEC(
                x_result_handle,
                y_result_handle,
                ec_handle,
                data.as_ptr(),
                data.len() as i32,
            );
        }
    }

    fn ec_generate_key(
        &self,
        x_pub_key_handle: Handle,
        y_pub_key_handle: Handle,
        ec_handle: Handle,
    ) -> BoxedBytes {
        unsafe {
            let priv_key_length = getPrivKeyByteLengthEC(ec_handle);
            let mut private_key = BoxedBytes::allocate(priv_key_length as usize);
            generateKeyEC(
                x_pub_key_handle,
                y_pub_key_handle,
                ec_handle,
                private_key.as_mut_ptr(),
            );
            private_key
        }
    }
}
