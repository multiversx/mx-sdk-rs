
use crate::*;
use crate::call_data::*;
use elrond_codec::*;

pub struct CallDataArgLoader<'a>{
    deser: CallDataDeserializer<'a>,
}

impl<'a> CallDataArgLoader<'a> {
    pub fn new(deser: CallDataDeserializer<'a>) -> Self {
        CallDataArgLoader {
            deser: deser,
        }
    }
}

impl<'a, T> DynArgLoader<T> for CallDataArgLoader<'a>
where
    T: Decode,
{
    #[inline]
    fn has_next(&self) -> bool {
        self.deser.has_next()
    }

    fn next_arg(&mut self, arg_id: ArgId) -> Result<Option<T>, SCError> {
        match self.deser.next_argument() {
            Ok(Some(arg_bytes)) => {
                match elrond_codec::decode_from_byte_slice(arg_bytes.as_slice()) {
                    Ok(v) => Ok(Some(v)),
                    Err(de_err) => {
                        let mut decode_err_message: Vec<u8> = Vec::new();
                        decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_1);
                        decode_err_message.extend_from_slice(arg_id);
                        decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_2);
                        decode_err_message.extend_from_slice(de_err.message_bytes());
                        Err(SCError::Dynamic(decode_err_message))
                    }
                }
            },
            Ok(None) => Ok(None),
            Err(sc_err) => Err(sc_err)
        }
    }
}

