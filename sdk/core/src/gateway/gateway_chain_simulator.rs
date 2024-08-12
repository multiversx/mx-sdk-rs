use std::collections::HashMap;

use super::GatewayProxy;

const SEND_USER_FUNDS_ENDPOINT: &str = "transaction/send-user-funds";
const GENERATE_BLOCKS_FUNDS_ENDPOINT: &str = "simulator/generate-blocks";
const ACCOUNT_DATA: &str = "address/";

impl GatewayProxy {
    pub async fn send_user_funds(&self, receiver: &String) {
        let mut r = HashMap::new();
        r.insert("receiver", receiver);
        let endpoint_funds = self.get_endpoint(SEND_USER_FUNDS_ENDPOINT);
        let _ = self.client.post(endpoint_funds).json(&r).send().await;
    }

    pub async fn generate_blocks(&self, number_blocks: u64) {
        let url_gen_blocks: String =
            format!("{}/{}", GENERATE_BLOCKS_FUNDS_ENDPOINT, number_blocks);
        let endpoint_blocks = self.get_endpoint(&url_gen_blocks);
        let _ = self.client.post(endpoint_blocks).send().await;
    }

    pub async fn set_state_chain_sim(&self, address: String) {
        let endpoint_funds = self.get_endpoint(&format!("{}{}", ACCOUNT_DATA, address));
        let data = self.client.get(endpoint_funds).send().await;
        match data {
            Ok(d) => println!("{:?}", d.text().await),
            Err(_) => todo!(),
        }
    }
}
