use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    hex_util::byte_to_hex_digits,
    types::{ManagedBuffer, ManagedBufferCachedBuilder},
};
use elrond_codec::TopEncode;

const OPEN_BRACE: u8 = b'{';
const CLOSED_BRACE: u8 = b'}';
const TWO_DOTS: u8 = b':';
const QUESTION_MARK: u8 = b'?';
const HEX_VALUE_PREFIX: &[u8] = b"0x";

const STATIC_BUFFER_LEN: usize = 10;
const TOO_MANY_ARGUMENTS_ERR_MSG: &[u8] = b"Too many arguments for error message";

enum ArgDisplayType {
    Hex {
        static_part_start: usize,
        static_part_end: usize,
    },
    Ascii {
        static_part_start: usize,
        static_part_end: usize,
    },
}

pub struct FormattedMessageBuilder<M: ManagedTypeApi> {
    buffer: ManagedBufferCachedBuilder<M>,
    format_message: &'static [u8],
    index_in_message: usize,
}

impl<M: ManagedTypeApi> FormattedMessageBuilder<M> {
    pub fn new(format_message: &'static [u8]) -> Self {
        Self {
            buffer: ManagedBufferCachedBuilder::new_from_slice(&[]),
            format_message,
            index_in_message: 0,
        }
    }

    pub fn signal_error(mut self) -> ! {
        self.flush_to_buffer();

        let msg = self.buffer.into_managed_buffer();
        M::error_api_impl().signal_error_from_buffer(msg.handle);
    }

    pub fn add_argument<T: TopEncode>(&mut self, arg: &T) {
        let mut encoded_arg = ManagedBuffer::<M>::new();
        if let Result::Err(err) = arg.top_encode(&mut encoded_arg) {
            M::error_api_impl().signal_error(err.message_bytes())
        }

        let starting_index = self.index_in_message;
        let msg_len = self.format_message.len();

        if starting_index >= msg_len {
            M::error_api_impl().signal_error(TOO_MANY_ARGUMENTS_ERR_MSG);
        }

        match self.advance_after_next_arg_position() {
            ArgDisplayType::Hex {
                static_part_start,
                static_part_end,
            } => {
                self.add_static_part(static_part_start, static_part_end);
                self.add_arg_as_hex(encoded_arg);
            },
            ArgDisplayType::Ascii {
                static_part_start,
                static_part_end,
            } => {
                self.add_static_part(static_part_start, static_part_end);
                self.add_arg_as_ascii(encoded_arg);
            },
        }
    }

    fn advance_after_next_arg_position(&mut self) -> ArgDisplayType {
        let starting_index = self.index_in_message;

        loop {
            while self.get_format_byte_or_panic(self.index_in_message) != OPEN_BRACE {
                self.index_in_message += 1;
            }

            let static_msg_end_index = self.index_in_message;
            self.index_in_message += 1;

            match self.get_format_byte_or_panic(self.index_in_message) {
                CLOSED_BRACE => {
                    self.index_in_message += 1;

                    return ArgDisplayType::Hex {
                        static_part_start: starting_index,
                        static_part_end: static_msg_end_index,
                    };
                },
                TWO_DOTS => {
                    if self.get_format_byte_or_panic(self.index_in_message + 1) == QUESTION_MARK
                        && self.get_format_byte_or_panic(self.index_in_message + 2) == CLOSED_BRACE
                    {
                        self.index_in_message += 3;

                        return ArgDisplayType::Ascii {
                            static_part_start: starting_index,
                            static_part_end: static_msg_end_index,
                        };
                    }
                },
                _ => {}, // continue the loop
            }
        }
    }

    fn add_static_part(&mut self, start_index: usize, end_index: usize) {
        let static_part = &self.format_message[start_index..end_index];
        self.buffer.append_bytes(static_part);
    }

    fn add_arg_as_hex(&mut self, arg: ManagedBuffer<M>) {
        self.buffer.append_bytes(HEX_VALUE_PREFIX);

        let arg_len = arg.len();
        if arg_len == 0 {
            return;
        }

        let mut static_buffer = [0u8; STATIC_BUFFER_LEN];
        let mut hex_bytes_buffer = [0u8; STATIC_BUFFER_LEN * 2];

        let mut current_arg_index = 0;
        while current_arg_index < arg_len {
            let bytes_remaining = arg_len - current_arg_index;
            let bytes_to_load = core::cmp::min(bytes_remaining, STATIC_BUFFER_LEN);

            let slice = &mut static_buffer[0..bytes_to_load];
            let _ = arg.load_slice(current_arg_index, slice);

            for i in 0..bytes_to_load {
                let (hex1, hex2) = byte_to_hex_digits(slice[i]);
                hex_bytes_buffer[i * 2] = hex1;
                hex_bytes_buffer[i * 2 + 1] = hex2;
            }

            let hex_slice = &hex_bytes_buffer[0..(bytes_to_load * 2)];
            self.buffer.append_bytes(hex_slice);

            current_arg_index += STATIC_BUFFER_LEN;
        }
    }

    fn add_arg_as_ascii(&mut self, arg: ManagedBuffer<M>) {
        self.buffer.append_managed_buffer(&arg);
    }

    fn get_format_byte_or_panic(&self, index: usize) -> u8 {
        match self.format_message.get(index) {
            Some(b) => *b,
            None => M::error_api_impl().signal_error(TOO_MANY_ARGUMENTS_ERR_MSG),
        }
    }

    fn flush_to_buffer(&mut self) {
        if self.index_in_message < self.format_message.len() {
            let slice = &self.format_message[self.index_in_message..];
            self.buffer.append_bytes(slice);
        }
    }
}
