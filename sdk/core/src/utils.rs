use base64::{engine::general_purpose, Engine as _};

pub fn base64_decode<T>(to_decode: T) -> Vec<u8>
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.decode(to_decode).unwrap()
}

pub fn base64_encode<T>(to_encode: T) -> String
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(to_encode)
}
