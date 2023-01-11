multiversx_sc::imports!();
use crate::types::CodecErrorTestType;

mod encode_err_proxy {
    multiversx_sc::imports!();
    use crate::types::CodecErrorTestType;

    #[multiversx_sc::proxy]
    pub trait EncodeErrorProxy {
        #[init]
        fn init(&self, error_arg: CodecErrorTestType);

        #[endpoint]
        fn encode_error_method(&self, error_arg: CodecErrorTestType);
    }
}

/// Test various serialization errors.
#[multiversx_sc::module]
pub trait CodecErrorTest {
    #[endpoint]
    fn codec_err_finish(&self) -> CodecErrorTestType {
        CodecErrorTestType
    }

    #[storage_get("load_with_key_err")]
    fn load_with_key_err(&self, key: CodecErrorTestType) -> i32;

    #[view]
    fn codec_err_storage_key(&self) -> i32 {
        self.load_with_key_err(CodecErrorTestType)
    }

    #[storage_get("load_with_value_err")]
    fn load_with_value_err(&self) -> CodecErrorTestType;

    #[view]
    fn codec_err_storage_get(&self) -> CodecErrorTestType {
        self.load_with_value_err()
    }

    #[storage_set("store_with_value_err")]
    fn store_with_value_err(&self, value: CodecErrorTestType);

    #[endpoint]
    fn codec_err_storage_set(&self) {
        self.store_with_value_err(CodecErrorTestType);
    }

    #[event("event_err_topic")]
    fn event_err_topic(&self, #[indexed] err_topic: CodecErrorTestType);

    #[endpoint]
    fn codec_err_event_topic(&self) {
        self.event_err_topic(CodecErrorTestType);
    }

    #[event("event_err_data")]
    fn event_err_data(&self, data: CodecErrorTestType);

    #[endpoint]
    fn codec_err_event_data(&self) {
        self.event_err_data(CodecErrorTestType);
    }

    #[proxy]
    fn encode_err_proxy(&self) -> encode_err_proxy::Proxy<Self::Api>;

    /// Never actually calls any deploy/upgrade, so it is appropriate in this contract.
    /// It just covers contract init serialization errors.
    #[endpoint]
    fn codec_err_contract_init(&self) {
        let _ = self.encode_err_proxy().init(CodecErrorTestType);
    }

    /// Never actually calls any async/sync call, so it is appropriate in this contract.
    /// It just covers contract call serialization errors.
    #[endpoint]
    fn codec_err_contract_call(&self) {
        let _ = self
            .encode_err_proxy()
            .contract(ManagedAddress::zero())
            .encode_error_method(CodecErrorTestType);
    }
}
