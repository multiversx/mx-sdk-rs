use crate::{rust_biguint, tx_mock::TxInputESDT};
use elrond_wasm::{elrond_codec::TopEncode, types::Address};

pub struct ScCallMandos {
    pub(crate) from: Address,
    pub(crate) to: Address,
    pub(crate) egld_value: num_bigint::BigUint,
    pub(crate) esdt: Vec<TxInputESDT>,
    pub(crate) function: String,
    pub(crate) arguments: Vec<Vec<u8>>,
    pub(crate) gas_limit: u64,
    pub(crate) gas_price: u64,
}

impl ScCallMandos {
    pub fn new(from: &Address, to: &Address, function: &str) -> Self {
        ScCallMandos {
            from: from.clone(),
            to: to.clone(),
            egld_value: rust_biguint!(0),
            esdt: Vec::new(),
            function: function.to_owned(),
            arguments: Vec::new(),
            gas_limit: u64::MAX,
            gas_price: 0,
        }
    }

    pub fn add_egld_value(&mut self, egld_value: &num_bigint::BigUint) {
        self.egld_value = egld_value.clone();
    }

    pub fn add_esdt_transfer(
        &mut self,
        token_id: &[u8],
        nonce: u64,
        esdt_value: &num_bigint::BigUint,
    ) {
        self.esdt.push(TxInputESDT {
            token_identifier: token_id.to_vec(),
            nonce,
            value: esdt_value.clone(),
        });
    }

    pub fn add_argument<T: TopEncode>(&mut self, arg: &T) {
        let mut arg_raw = Vec::new();
        let _ = arg.top_encode(&mut arg_raw);

        self.arguments.push(arg_raw);
    }

    pub fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas_limit = gas_limit;
    }

    pub fn set_gas_price(&mut self, gas_price: u64) {
        self.gas_price = gas_price;
    }
}

pub struct ScQueryMandos {
    pub(crate) to: Address,
    pub(crate) function: String,
    pub(crate) arguments: Vec<Vec<u8>>,
}

impl ScQueryMandos {
    pub fn new(to: &Address, function: &str) -> Self {
        ScQueryMandos {
            to: to.clone(),
            function: function.to_owned(),
            arguments: Vec::new(),
        }
    }

    pub fn add_argument<T: TopEncode>(&mut self, arg: &T) {
        let mut arg_raw = Vec::new();
        let _ = arg.top_encode(&mut arg_raw);

        self.arguments.push(arg_raw);
    }
}

pub struct TxExpectMandos {
    pub(crate) out: Vec<Vec<u8>>,
    pub(crate) status: u64,
    pub(crate) message: String,
    // TODO: Add logs?
}

impl TxExpectMandos {
    pub fn new(status: u64) -> Self {
        TxExpectMandos {
            out: Vec::new(),
            status,
            message: String::new(),
        }
    }

    pub fn add_out_value<T: TopEncode>(&mut self, out_val: &T) {
        let mut out_raw = Vec::new();
        let _ = out_val.top_encode(&mut out_raw);

        self.out.push(out_raw);
    }

    pub fn set_message(&mut self, msg: &str) {
        self.message = msg.to_owned();
    }
}
