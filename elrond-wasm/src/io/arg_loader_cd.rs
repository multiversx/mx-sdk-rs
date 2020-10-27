
use crate::*;
use crate::call_data::*;
use elrond_codec::*;

pub struct CallDataArgLoader<'a>{
    deser: CallDataDeserializer<'a>,
}

impl<'a> CallDataArgLoader<'a> {
    pub fn new(deser: CallDataDeserializer<'a>) -> Self {
        CallDataArgLoader {
            deser,
        }
    }
}

impl<'a, T> DynArgLoader<T> for CallDataArgLoader<'a>
where
    T: TopDecode,
{
    #[inline]
    fn has_next(&self) -> bool {
        self.deser.has_next()
    }

    fn next_arg(&mut self, arg_id: ArgId) -> Result<Option<T>, SCError> {
        match self.deser.next_argument() {
            Ok(Some(arg_bytes)) => {
                T::top_decode(arg_bytes.as_slice(), |res| match res {
                    Ok(v) => Ok(Some(v)),
                    Err(de_err) => {
                        let mut decode_err_message: Vec<u8> = Vec::new();
                        decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_1);
                        decode_err_message.extend_from_slice(arg_id);
                        decode_err_message.extend_from_slice(err_msg::ARG_DECODE_ERROR_2);
                        decode_err_message.extend_from_slice(de_err.message_bytes());
                        Err(SCError::from(decode_err_message))
                    }
                })
            },
            Ok(None) => Ok(None),
            Err(sc_err) => Err(sc_err)
        }
    }
}

