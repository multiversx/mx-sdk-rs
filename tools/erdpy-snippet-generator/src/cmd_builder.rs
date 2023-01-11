use multiversx_sc::codec::{top_encode_to_vec_u8_or_panic, TopEncode};

const FLAG_PREFIX: &str = "--";
const HEX_PREFIX: &str = "0x";
const SPACE: &str = " ";
const QUOTE: &str = "\"";

pub struct CmdBuilder {
    cmd: String,
}

impl CmdBuilder {
    pub fn new(program_name: &str) -> Self {
        CmdBuilder {
            cmd: program_name.to_owned(),
        }
    }

    pub fn append_string_no_quotes(&mut self, string: &str) {
        self.add_space();
        self.cmd += string;
    }

    pub fn add_command(&mut self, command: &str) {
        self.add_space();
        self.cmd += command;
    }

    pub fn add_flag(&mut self, flag_name: &str) {
        self.add_space();
        self.cmd += FLAG_PREFIX;
        self.cmd += flag_name;
    }

    pub fn to_hex<T: TopEncode>(arg: &T) -> String {
        hex::encode(top_encode_to_vec_u8_or_panic(arg))
    }

    pub fn add_standalone_argument<T: TopEncode>(&mut self, arg: &T) {
        let arg_as_hex = Self::to_hex(arg);

        self.add_space();
        self.cmd += HEX_PREFIX;
        self.cmd += &arg_as_hex;
    }

    pub fn add_numerical_argument(&mut self, arg_name: &str, arg: &num_bigint::BigUint) {
        self.add_flag(arg_name);
        self.add_space();
        self.cmd += &arg.to_string();
    }

    pub fn add_raw_named_argument(&mut self, arg_name: &str, arg: &str) {
        self.add_flag(arg_name);
        self.add_raw_standalone_argument(arg);
    }

    pub fn add_raw_standalone_argument(&mut self, arg: &str) {
        self.add_space();
        self.cmd += QUOTE;
        self.cmd += arg;
        self.cmd += QUOTE;
    }

    pub fn print(self) {
        println!("{}", self.cmd);
    }

    fn add_space(&mut self) {
        self.cmd += SPACE;
    }
}
