#[macro_export]
macro_rules! allocator {
    () => {
        #[global_allocator]
        static ALLOC: multiversx_sc_wasm_adapter::wasm_deps::WeeAlloc =
            multiversx_sc_wasm_adapter::wasm_deps::WeeAlloc::INIT;
    };
}

#[macro_export]
macro_rules! panic_handler {
    () => {
        #[alloc_error_handler]
        fn alloc_error_handler(layout: multiversx_sc_wasm_adapter::wasm_deps::Layout) -> ! {
            multiversx_sc_wasm_adapter::wasm_deps::alloc_error_handler(layout)
        }

        #[panic_handler]
        fn panic_fmt(panic_info: &multiversx_sc_wasm_adapter::wasm_deps::PanicInfo) -> ! {
            multiversx_sc_wasm_adapter::wasm_deps::panic_fmt(panic_info)
        }

        #[lang = "eh_personality"]
        fn eh_personality() {}
    };
}

#[macro_export]
macro_rules! panic_handler_with_message {
    () => {
        #[alloc_error_handler]
        fn alloc_error_handler(layout: multiversx_sc_wasm_adapter::wasm_deps::Layout) -> ! {
            multiversx_sc_wasm_adapter::wasm_deps::alloc_error_handler(layout)
        }

        #[panic_handler]
        fn panic_fmt(panic_info: &multiversx_sc_wasm_adapter::wasm_deps::PanicInfo) -> ! {
            multiversx_sc_wasm_adapter::wasm_deps::panic_fmt_with_message(panic_info)
        }

        #[lang = "eh_personality"]
        fn eh_personality() {}
    };
}

#[macro_export]
macro_rules! endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        #[no_mangle]
        fn init() {
            $mod_name::endpoints::init::<multiversx_sc_wasm_adapter::api::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<multiversx_sc_wasm_adapter::api::VmApiImpl>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        #[no_mangle]
        fn init() {
            multiversx_sc_wasm_adapter::multiversx_sc::external_view_contract::external_view_contract_constructor::<multiversx_sc_wasm_adapter::api::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<multiversx_sc_wasm_adapter::multiversx_sc::api::ExternalViewApi<multiversx_sc_wasm_adapter::api::VmApiImpl>>();
            }
        )*
    };
}

#[macro_export]
macro_rules! empty_callback {
    () => {
        #[allow(non_snake_case)]
        #[no_mangle]
        fn callBack() {}
    };
}
