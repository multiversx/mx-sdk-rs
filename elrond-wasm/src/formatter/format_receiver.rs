use elrond_codec::TopEncode;

pub trait FormatReceiver {
    fn push_static_ascii(&mut self, arg: &'static [u8]);

    fn push_top_encode_bytes<T>(&mut self, item: &T)
    where
        T: TopEncode;

    fn push_top_encode_hex<T>(&mut self, item: &T)
    where
        T: TopEncode;
}
