use crate::scenario_model::{NewAddress, SetStateStep};

use super::SetStateBuilderItem;

pub struct NewAddressItem {
    pub(super) new_address: NewAddress,
}

impl SetStateBuilderItem for NewAddressItem {
    fn commit_to_step(&mut self, step: &mut SetStateStep) {
        step.new_addresses
            .push(core::mem::take(&mut self.new_address));
    }
}
