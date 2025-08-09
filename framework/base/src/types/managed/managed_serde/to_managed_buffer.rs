use core::fmt;
use serde::Serialize;

use crate::{api::ManagedTypeApi, types::ManagedBuffer};

#[derive(Debug)]
pub enum ToManagedBufferError {
    Serde(serde_json::Error),
}

impl fmt::Display for ToManagedBufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToManagedBufferError::Serde(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl From<serde_json::Error> for ToManagedBufferError {
    fn from(e: serde_json::Error) -> Self {
        ToManagedBufferError::Serde(e)
    }
}

struct ManagedBufferWriter<'a, M: ManagedTypeApi> {
    buffer: &'a mut ManagedBuffer<M>,
}

impl<'a, M: ManagedTypeApi> std::io::Write for ManagedBufferWriter<'a, M> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.append_bytes(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn to_managed_buffer<M, T>(value: &T) -> Result<ManagedBuffer<M>, ToManagedBufferError>
where
    M: ManagedTypeApi,
    T: ?Sized + Serialize,
{
    let mut buffer = ManagedBuffer::<M>::new();
    {
        let mut writer = ManagedBufferWriter {
            buffer: &mut buffer,
        };
        let mut serializer = serde_json::Serializer::new(&mut writer);
        value.serialize(&mut serializer)?;
    }
    Ok(buffer)
}
