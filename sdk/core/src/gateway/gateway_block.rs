use crate::data::hyperblock::{HyperBlock, HyperBlockResponse};
use anyhow::anyhow;

use super::{
    GatewayRequest, GatewayRequestType, GET_HYPER_BLOCK_BY_HASH_ENDPOINT,
    GET_HYPER_BLOCK_BY_NONCE_ENDPOINT,
};

/// Retrieves the data of a hyper block.
pub struct GetHyperBlockRequest {
    pub query: String,
}

impl GetHyperBlockRequest {
    pub fn by_nonce(nonce: u64) -> Self {
        Self {
            query: format!("{GET_HYPER_BLOCK_BY_NONCE_ENDPOINT}/{nonce}"),
        }
    }

    pub fn by_hash(hash: &str) -> Self {
        Self {
            query: format!("{GET_HYPER_BLOCK_BY_HASH_ENDPOINT}/{hash}"),
        }
    }
}

impl GatewayRequest for GetHyperBlockRequest {
    type Payload = ();
    type DecodedJson = HyperBlockResponse;
    type Result = HyperBlock;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        self.query.clone()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.hyperblock),
        }
    }
}
