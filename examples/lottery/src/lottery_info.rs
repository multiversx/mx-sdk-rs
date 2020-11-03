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
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.ticket_price.dep_encode(dest)?;
        self.tickets_left.dep_encode(dest)?;
        self.deadline.dep_encode(dest)?;
        self.max_entries_per_user.dep_encode(dest)?;
        self.prize_distribution.dep_encode(dest)?;
        self.whitelist.dep_encode(dest)?;
        self.current_ticket_number.dep_encode(dest)?;
        self.prize_pool.dep_encode(dest)?;

        Ok(())
    }
}

impl<BigUint:BigUintApi> TopEncode for LotteryInfo<BigUint> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        top_encode_from_nested(self, output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
        top_encode_from_nested_or_exit(self, output, c, exit);
    }
}


impl<BigUint:BigUintApi> NestedDecode for LotteryInfo<BigUint> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(LotteryInfo {
            ticket_price: BigUint::dep_decode(input)?,
            tickets_left: u32::dep_decode(input)?,
            deadline: u64::dep_decode(input)?,
            max_entries_per_user: u32::dep_decode(input)?,
            prize_distribution: Vec::<u8>::dep_decode(input)?,
            whitelist: Vec::<Address>::dep_decode(input)?,
            current_ticket_number: u32::dep_decode(input)?,
            prize_pool: BigUint::dep_decode(input)?,
        })
    }
}

impl<BigUint:BigUintApi> TopDecode for LotteryInfo<BigUint> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        top_decode_from_nested(input)
    }
}
