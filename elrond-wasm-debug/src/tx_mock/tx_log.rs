use alloc::vec::Vec;
use elrond_wasm::types::Address;
use mandos::model::Checkable;

use crate::verbose_hex;

#[derive(Clone, Debug)]
pub struct TxLog {
    pub address: Address,
    pub endpoint: Vec<u8>,
    pub topics: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl TxLog {
    pub fn mandos_check(&self, check_log: &mandos::model::CheckLog) -> bool {
        self.address.to_vec() == check_log.address.value
            && check_log.endpoint.check(self.endpoint.as_slice())
            && check_log.topics.check(self.topics.as_slice())
            && check_log.data.check(self.data.as_slice())
    }

    pub fn topics_pretty(&self) -> String {
        let mut s = String::new();
        s.push('[');
        for (i, topic) in self.topics.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(verbose_hex(topic).as_str());
        }
        s.push(']');
        s
    }
}
