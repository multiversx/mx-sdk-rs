#![no_std]

use basics::{constants, storage, utils, views};
use multiversx_sc::imports::*;
use specific::{award, awarding_status, buy, claim, lottery_info, setup, status};

mod basics;
mod specific;

use awarding_status::AwardingStatus;
use lottery_info::LotteryInfo;
use status::Status;

#[multiversx_sc::contract]
pub trait Lottery:
    award::AwardingModule
    + views::ViewsModule
    + storage::StorageModule
    + utils::UtilsModule
    + claim::ClaimModule
    + buy::BuyTicketModule
    + setup::SetupModule
{
    #[init]
    fn init(&self) {}
}
