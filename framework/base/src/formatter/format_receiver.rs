pub trait FormatReceiver {
    fn push_static_ascii(&mut self, arg: &'static [u8]);

    fn push_bytes(&mut self, item: &mut ManagedFormatter<'a, M>);
    fn push_lower_hex(&mut self, item: &mut ManagedFormatter<'a, M>);
}

struct FormatterIgnorer;

impl FormatReceiver for FormatterIgnorer {
    fn push_static_ascii(&mut self, _arg: &'static [u8]) {}

    fn push_bytes(&mut self, item: &mut ManagedFormatter<'a, M>) {}

    fn push_lower_hex(&mut self, item: &mut ManagedFormatter<'a, M>) {}
}

pub trait DisplayFormatter {}

pub trait LowerHexFormatter {}
