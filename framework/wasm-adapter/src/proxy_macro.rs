#[macro_export]
macro_rules! endpoints_proxy {
    ($mod_name:ident ( $(
    ($docs:ident)?
    $endpoint_name:ident => $method_name:ident$(< $generic:ident >)*( $($arg:ident: $arg_type:ident)* )
    )* ) ) => {
        $(
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn $method_name<
                $($generic: multiversx_sc::codec::CodecInto<multiversx_sc::types::$arg_type<Self::Api>>,)*
            >(
                &mut self,
                $(
                    $arg: $generic,
                )*
            ) -> multiversx_sc::types::ContractCallNoPayment<Self::Api, ()> {
                let ___address___ = self.extract_address();
                let mut ___contract_call___ = multiversx_sc::types::ContractCallNoPayment::new(
                    ___address___,
                    stringify!($endpoint_name),
                );

                $(
                    multiversx_sc::types::ContractCall::proxy_arg(&mut ___contract_call___, &$arg);
                )*

                ___contract_call___
            }
        )*

        $(
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn $method_name(
                &mut self
            ) -> multiversx_sc::types::ContractCallNoPayment<
                Self::Api,
                SingleValueMapper<Self::Api, multiversx_sc::types::BigUint<Self::Api>
            > {
                let ___address___ = self.extract_address();
                let mut ___contract_call___ = multiversx_sc::types::ContractCallNoPayment::new(
                    ___address___,
                    stringify!($endpoint_name),
                );
                ___contract_call___
            }
        )*
    };
}