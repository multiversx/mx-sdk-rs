use crate::{LotteryInfo, Status};
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait ViewsModule {
    #[view]
    fn status(&self, lottery_name: &ManagedBuffer) -> Status {
        if self.lottery_info(lottery_name).is_empty() {
            return Status::Inactive;
        }

        let info = self.lottery_info(lottery_name).get();
        let current_time = self.blockchain().get_block_timestamp();
        if current_time > info.deadline || info.tickets_left == 0 {
            return Status::Ended;
        }

        Status::Running
    }

    #[view(getLotteryInfo)]
    #[storage_mapper("lotteryInfo")]
    fn lottery_info(
        &self,
        lottery_name: &ManagedBuffer,
    ) -> SingleValueMapper<LotteryInfo<Self::Api>>;

    #[view(getLotteryWhitelist)]
    #[storage_mapper("lotteryWhitelist")]
    fn lottery_whitelist(&self, lottery_name: &ManagedBuffer) -> UnorderedSetMapper<u64>;
}
