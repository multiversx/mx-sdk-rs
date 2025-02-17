use multiversx_sc::imports::*;

use crate::basics::{storage, views};

use super::status::Status;

#[multiversx_sc::module]
pub trait BuyTicketModule: storage::StorageModule + views::ViewsModule {
    #[endpoint]
    #[payable]
    fn buy_ticket(&self, lottery_name: ManagedBuffer) {
        let (token_identifier, payment) = self.call_value().egld_or_single_fungible_esdt();

        match self.status(&lottery_name) {
            Status::Inactive => sc_panic!("Lottery is currently inactive."),
            Status::Running => {
                self.update_after_buy_ticket(&lottery_name, &token_identifier, &payment)
            },
            Status::Ended => {
                sc_panic!("Lottery entry period has ended! Awaiting winner announcement.")
            },
        };
    }

    fn update_after_buy_ticket(
        &self,
        lottery_name: &ManagedBuffer,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        payment: &BigUint,
    ) {
        let info_mapper = self.lottery_info(lottery_name);
        let mut info = info_mapper.get();
        let caller = self.blockchain().get_caller();
        let caller_id = self.addres_to_id_mapper().get_id_or_insert(&caller);
        let whitelist = self.lottery_whitelist(lottery_name);

        require!(
            whitelist.is_empty() || whitelist.contains(&caller_id),
            "You are not allowed to participate in this lottery!"
        );
        require!(
            token_identifier == &info.token_identifier && payment == &info.ticket_price,
            "Wrong ticket fee!"
        );

        let entries_mapper = self.number_of_entries_for_user(lottery_name, &caller_id);
        let mut entries = entries_mapper.get();
        require!(
            entries < info.max_entries_per_user,
            "Ticket limit exceeded for this lottery!"
        );

        self.ticket_holders(lottery_name).push(&caller_id);

        entries += 1;
        info.tickets_left -= 1;
        info.prize_pool += &info.ticket_price;
        info.unawarded_amount += &info.ticket_price;

        entries_mapper.set(entries);
        info_mapper.set(&info);
    }
}
