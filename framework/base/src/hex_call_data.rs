mod cd_de;
mod cd_ser;

pub use cd_de::HexCallDataDeserializer;
pub use cd_ser::HexCallDataSerializer;

const SEPARATOR: u8 = b'@';
