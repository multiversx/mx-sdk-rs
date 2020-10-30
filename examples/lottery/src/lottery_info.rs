use elrond_wasm::elrond_codec::*;
use elrond_wasm::{Vec, BigUintApi, Address};

pub struct LotteryInfo<BigUint:BigUintApi> {
    pub ticket_price: BigUint, 
    pub tickets_left: u32, 
    pub deadline: u64,
    pub max_entries_per_user: u32,
    pub prize_distribution: Vec<u8>,
    pub whitelist: Vec<Address>, 
    pub current_ticket_number: u32,
    pub prize_pool: BigUint
}

impl<BigUint:BigUintApi> NestedEncode for LotteryInfo<BigUint> {
    fn dep_encode_to<O: OutputBuffer>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.ticket_price.dep_encode_to(dest)?;
        self.tickets_left.dep_encode_to(dest)?;
        self.deadline.dep_encode_to(dest)?;
        self.max_entries_per_user.dep_encode_to(dest)?;
        self.prize_distribution.dep_encode_to(dest)?;
        self.whitelist.dep_encode_to(dest)?;
        self.current_ticket_number.dep_encode_to(dest)?;
        self.prize_pool.dep_encode_to(dest)?;

        Ok(())
    }
}

impl<BigUint:BigUintApi> TopEncode for LotteryInfo<BigUint> {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_slice_u8(dep_encode_to_vec(self)?.as_slice());
        Ok(())
    }
}


impl<BigUint:BigUintApi> NestedDecode for LotteryInfo<BigUint> {
    fn dep_decode_to<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(LotteryInfo {
            ticket_price: BigUint::dep_decode_to(input)?,
            tickets_left: u32::dep_decode_to(input)?,
            deadline: u64::dep_decode_to(input)?,
            max_entries_per_user: u32::dep_decode_to(input)?,
            prize_distribution: Vec::<u8>::dep_decode_to(input)?,
            whitelist: Vec::<Address>::dep_decode_to(input)?,
            current_ticket_number: u32::dep_decode_to(input)?,
            prize_pool: BigUint::dep_decode_to(input)?,
        })
    }
}

impl<BigUint:BigUintApi> TopDecode for LotteryInfo<BigUint> {
    fn top_decode<I: TopDecodeInput, R, F: FnOnce(Result<Self, DecodeError>) -> R>(input: I, f: F) -> R {
        top_decode_from_nested(input, f)
    }
}
