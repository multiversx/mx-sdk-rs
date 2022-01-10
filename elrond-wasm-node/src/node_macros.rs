#[macro_export]
macro_rules! wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)+ ) ) => {
        pub use elrond_wasm_output;

        #[no_mangle]
        fn init() {
            $mod_name::endpoints::init::<elrond_wasm_node::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<elrond_wasm_node::VmApiImpl>();
            }
        )+
    };
}

#[macro_export]
macro_rules! external_view_wasm_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)+ ) ) => {
        pub use elrond_wasm_output;

        #[no_mangle]
        fn init() {
            elrond_wasm_node::elrond_wasm::external_view_contract::external_view_contract_constructor::<elrond_wasm_node::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<elrond_wasm_node::elrond_wasm::api::ExternalViewApi<elrond_wasm_node::VmApiImpl>>();
            }
        )+
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
