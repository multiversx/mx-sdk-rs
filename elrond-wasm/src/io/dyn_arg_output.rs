use elrond_codec::TopEncode;

pub trait DynArgOutput {
    fn push_single_arg<T: TopEncode>(&mut self, arg: T);
}
