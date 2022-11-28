pub trait FormatReceiver {
    fn push_static_ascii(&mut self, arg: &'static [u8]);

    fn push_bytes(&mut self, item: &mut ManagedFormatter<M>);
    fn push_lower_hex(&mut self, item: &mut ManagedFormatter<M>);
}

struct FormatterIgnorer;

impl FormatReceiver for FormatterIgnorer {
    fn push_static_ascii(&mut self, _arg: &'static [u8]) {}

    fn push_bytes(&mut self, item: &mut ManagedFormatter<M>) {}

    fn push_lower_hex(&mut self, item: &mut ManagedFormatter<M>) {}
}

pub trait DisplayFormatter {}

pub trait LowerHexFormatter {}
