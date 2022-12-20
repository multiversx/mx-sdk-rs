#[macro_export]
macro_rules! wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        #[no_mangle]
        fn init() {
            $mod_name::endpoints::init::<mx_sc_wasm_adapter::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<mx_sc_wasm_adapter::VmApiImpl>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        #[no_mangle]
        fn init() {
            mx_sc_wasm_adapter::mx_sc::external_view_contract::external_view_contract_constructor::<mx_sc_wasm_adapter::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<mx_sc_wasm_adapter::mx_sc::api::ExternalViewApi<mx_sc_wasm_adapter::VmApiImpl>>();
            }
        )*
    };
}

#[macro_export]
macro_rules! wasm_empty_callback {
    () => {
        #[allow(non_snake_case)]
        #[no_mangle]
        fn callBack() {}
    };
}
