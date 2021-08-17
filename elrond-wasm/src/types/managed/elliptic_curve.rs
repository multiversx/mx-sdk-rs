use alloc::string::String;

use crate::api::{Handle, ManagedTypeApi};

use super::ManagedBuffer;
use crate::types::BoxedBytes;

pub type EllipticCurveComponents<M: ManagedTypeApi> = (
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
    pub(super) api: M,
}

impl<M: ManagedTypeApi> EllipticCurveApi for EllipticCurve<M> {
    fn create_ec(name: &str, api: M) -> Self {
        unsafe {
            let curve_as_slice = curve.as_bytes();
            let data: BoxedBytes = BoxedBytes::from(curve_as_slice);
            let handle = api.create_ec(name.as_bytes());
            EllipticCurve { handle, api }
        }
    }

    fn get_values(&self) -> EllipticCurveComponents<BigUint<M>> {
        unsafe {
            let field_order_handle = self.api.bi_new_zero();
            let base_point_order_handle = self.api.bi_new_zero();
            let eq_constant_handle = self.api.bi_new_zero();
            let x_base_point_handle = self.api.bi_new_zero();
            let y_base_point_handle = self.api.bi_new_zero();
            self.api.ec_get_values(
                self.handle,
                field_order_handle,
                base_point_order_handle,
                eq_constant_handle,
                x_base_point_handle,
                y_base_point_handle,
            );
            (
                BigUint {
                    handle: field_order_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: base_point_order_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: eq_constant_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: x_base_point_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: y_base_point_handle,
                    api: self.api.clone(),
                },
                self.api.get_ec_length(),
            )
        }
    }

    fn get_ec_length(&self) -> u32 {
        self.api.get_ec_length(self.handle)
    }

    fn get_priv_key_byte_length(&self) -> u32 {
        self.api.ec_private_key_byte_length(self.handle)
    }

    fn add_ec(
        &self,
        x_first_point: BigUint<M>,
        y_first_point: BigUint<M>,
        x_second_point: BigUint<M>,
        y_second_point: BigUint<M>,
    ) -> (BigUint<M>, BigUint<M>) {
        unsafe {
            let x_result_handle = self.api.bi_new_zero();
            let y_result_handle = self.api.bi_new_zero();
            self.api.add_ec(
                x_result_handle,
                y_result_handle,
                self.handle,
                x_first_point.handle,
                y_first_point.handle,
                x_second_point.handle,
                y_second_point.handle,
            );
            (
                BigUint {
                    handle: x_result_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: y_result_handle,
                    api: self.api.clone(),
                },
            )
        }
    }

    fn double_ec(&self, x_point: BigUint<M>, y_point: BigUint<M>) -> (BigUint<M>, BigUint<M>) {
        unsafe {
            let x_result_handle = self.api.bi_new_zero();
            let y_result_handle = self.api.bi_new_zero();
            self.api.ec_double(
                x_result_handle,
                y_result_handle,
                self.handle,
                x_point.handle,
                y_point.handle,
            );
            (
                BigUint {
                    handle: x_result_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: y_result_handle,
                    api: self.api.clone(),
                },
            )
        }
    }

    fn is_on_curve_ec(&self, x_point: BigUint<M>, y_point: BigUint<M>) -> bool {
        self.api.ec_is_on_curve(self.handle, x_point.handle, y_point.handle)
    }

    fn scalar_mult(
        &self,
        x_point: BigUint<M>,
        y_point: BigUint<M>,
        data: &[u8],
    ) -> (BigUint<M>, BigUint<M>) {
        unsafe {
            let x_result_handle = self.api.bi_new_zero();
            let y_result_handle = self.api.bi_new_zero();
            self.api.ec_scalar_mult(
                x_result_handle,
                y_result_handle,
                self.handle,
                x_point.handle,
                y_point.handle,
                data.as_ptr(),
                data.len() as i32,
            );
            (
                BigUint {
                    handle: x_result_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: y_result_handle,
                    api: self.api.clone(),
                },
            )
        }
    }

    fn scalar_base_mult(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        unsafe {
            let x_result_handle = self.api.bi_new_zero();
            let y_result_handle = self.api.bi_new_zero();
            self.api.ec_scalar_base_mult(
                x_result_handle,
                y_result_handle,
                self.handle,
                data,
            );
            (
                BigUint {
                    handle: x_result_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: y_result_handle,
                    api: self.api.clone(),
                },
            )
        }
    }

    fn marshal_ec(&self, x_pair: BigUint<M>, y_pair: BigUint<M>) -> BoxedBytes {
        self.api.ec_marshal(self.handle, x_pair.handle, y_pair.handle)
    }

    fn marshal_compressed_ec(&self, x_pair: BigUint<M>, y_pair: BigUint<M>) -> BoxedBytes {
        self.api.ec_marshal_compressed(self.handle, x_pair.handle, y_pair.handle)
    }

    fn unmarshal_ec(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        unsafe {
            let x_pair_handle = self.api.bi_new_zero();
            let y_pair_handle = self.api.bi_new_zero();
            self.api.ec_unmarshal(
                x_pair_handle,
                y_pair_handle,
                self.handle,
                data,
            );
            (
                BigUint {
                    handle: x_pair_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: y_pair_handle,
                    api: self.api.clone(),
                },
            )
        }
    }

    fn unmarshal_compressed_ec(&self, data: &[u8]) -> (BigUint<M>, BigUint<M>) {
        unsafe {
            let x_pair_handle = self.api.bi_new_zero();
            let y_pair_handle = self.api.bi_new_zero();
            self.api.ec_unmarshal_compressed(
                x_pair_handle,
                y_pair_handle,
                self.handle,
                data.as_ptr(),
                data.len() as i32,
            );
            (
                BigUint {
                    handle: x_pair_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: y_pair_handle,
                    api: self.api.clone(),
                },
            )
        }
    }

    fn generate_key_ec(&self) -> (BigUint<M>, BigUint<M>, BoxedBytes) {
        unsafe {
            let x_pub_key_handle = self.api.bi_new_zero();
            let y_pub_key_handle = self.api.bi_new_zero();
            let private_key = self.api.generate_key(
                x_pub_key_handle,
                y_pub_key_handle,
                self.handle,
                private_key.as_mut_ptr(),
            );
            (
                BigUint {
                    handle: x_pub_key_handle,
                    api: self.api.clone(),
                },
                BigUint {
                    handle: y_pub_key_handle,
                    api: self.api.clone(),
                },
                private_key,
            )
        }
    }

    fn from_bitsize_ec(bitsize: u32) -> Option<Self> {
        match bitsize {
            224 => Some(Self::create_ec("p224")),
            256 => Some(Self::create_ec("p256")),
            384 => Some(Self::create_ec("p384")),
            521 => Some(Self::create_ec("p521")),
            _ => None,
        }
    }
}

use elrond_codec::*;

impl NestedEncode for EllipticCurve<M> {
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

impl TopEncode for EllipticCurve<M> {
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

impl<M: ManagedTypeApi> elrond_wasm::abi::TypeAbi for EllipticCurve<M> {
    fn type_name() -> String {
        String::from("EllipticCurve")
    }
}
