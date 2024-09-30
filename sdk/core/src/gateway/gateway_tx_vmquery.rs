use crate::data::vm::{ResponseVmValue, VMQueryInput, VmValuesResponseData};
use anyhow::anyhow;

use super::{GatewayRequest, GatewayRequestType, VM_VALUES_ENDPOINT};

/// Executes a VM query.
pub struct VMQueryRequest<'a>(pub &'a VMQueryInput);

impl<'a> GatewayRequest for VMQueryRequest<'a> {
    type Payload = VMQueryInput;
    type DecodedJson = ResponseVmValue;
    type Result = VmValuesResponseData;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Post
    }

    fn get_payload(&self) -> Option<&Self::Payload> {
        Some(self.0)
    }

    fn get_endpoint(&self) -> String {
        VM_VALUES_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b),
        }
    }
}
