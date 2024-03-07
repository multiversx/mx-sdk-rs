multiversx_sc::imports!();

pub struct TxProxy;

impl<Env> TxProxyTrait<Env> for TxProxy
where
    Env: TxEnv,
{
    type TxProxyMethods = TxProxyMethods<Env>;

    fn env(self, env: Env) -> Self::TxProxyMethods {
        TxProxyMethods { env }
    }
}

impl<Env: TxEnv + multiversx_sc::api::CallTypeApi> TxProxyMethods<Env> {
	pub fn init<
		Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>,
	>(
		&mut self,
		initial_value: Arg0,
	) -> multiversx_sc::types::Tx<Env,
        (),
        (),
        (),
        (),
        FunctionCall<<Env as multiversx_sc::types::TxEnv>::Api>,
        (),
    > {
		Tx::new_with_env(self.env.clone())
            .raw_call()
            .function_name("init")
			.argument(&initial_value)
	}

	pub fn sum(
		&mut self,
	) -> multiversx_sc::types::Tx<Env,
        (),
        (),
        (),
        (),
        FunctionCall<<Env as multiversx_sc::types::TxEnv>::Api>,
        (),
    > {
		Tx::new_with_env(self.env.clone())
            .raw_call()
            .function_name("getSum")
	}

	pub fn upgrade<
		Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>,
	>(
		&mut self,
		initial_value: Arg0,
	) -> multiversx_sc::types::Tx<Env,
        (),
        (),
        (),
        (),
        FunctionCall<<Env as multiversx_sc::types::TxEnv>::Api>,
        (),
    > {
		Tx::new_with_env(self.env.clone())
            .raw_call()
            .function_name("upgrade")
			.argument(&initial_value)
	}

	//Add desired amount to the storage variable. 
	pub fn add<
		Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>,
	>(
		&mut self,
		value: Arg0,
	) -> multiversx_sc::types::Tx<Env,
        (),
        (),
        (),
        (),
        FunctionCall<<Env as multiversx_sc::types::TxEnv>::Api>,
        (),
    > {
		Tx::new_with_env(self.env.clone())
            .raw_call()
            .function_name("add")
			.argument(&value)
	}

}
