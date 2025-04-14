pub struct RuntimeInstanceCallArg<'a> {
    pub instance: &'a dyn Instance,
    pub func_name: &'a str,
    pub tx_context_ref: &'a TxContextRef,
}