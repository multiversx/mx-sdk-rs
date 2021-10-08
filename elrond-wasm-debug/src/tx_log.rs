use alloc::vec::Vec;
use elrond_wasm::types::Address;

#[derive(Clone, Debug)]
pub struct TxLog {
    pub address: Address,
    pub endpoint: Vec<u8>,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl TxLog {
    pub fn equals(&self, check_log: &mandos::model::CheckLog) -> bool {
        if self.address.to_vec() == check_log.address.value
            && self.endpoint == check_log.endpoint.value
            && self.data == check_log.data.value
        {
            for (topic, other_topic) in self.topics.iter().zip(check_log.topics.iter()) {
                if topic != &other_topic.value {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }
}
