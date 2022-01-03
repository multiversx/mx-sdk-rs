use alloc::vec::Vec;
use elrond_wasm::types::Address;
use mandos::model::Checkable;

#[derive(Clone, Debug)]
pub struct TxLog {
    pub address: Address,
    pub endpoint: Vec<u8>,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl TxLog {
    pub fn mandos_check(&self, check_log: &mandos::model::CheckLog) -> bool {
        if self.address.to_vec() == check_log.address.value
            && check_log.endpoint.check(self.endpoint.as_slice())
            && check_log.data.check(self.data.as_slice())
        {
            for (topic, other_topic) in self.topics.iter().zip(check_log.topics.iter()) {
                if !other_topic.check(topic) {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }
}
