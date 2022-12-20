#[macro_export]
macro_rules! allocator_declaration {
    () => {
        #[global_allocator]
        static ALLOC: mx_sc_wasm_adapter::wasm_deps::WeeAlloc =
            mx_sc_wasm_adapter::wasm_deps::WeeAlloc::INIT;
    };
}

#[macro_export]
macro_rules! panic_handler_declaration {
    () => {
        #[alloc_error_handler]
        fn alloc_error_handler(layout: mx_sc_wasm_adapter::wasm_deps::Layout) -> ! {
            mx_sc_wasm_adapter::wasm_deps::alloc_error_handler(layout)
        }

        #[panic_handler]
        fn panic_fmt(panic_info: &mx_sc_wasm_adapter::wasm_deps::PanicInfo) -> ! {
            mx_sc_wasm_adapter::wasm_deps::panic_fmt(panic_info)
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
            $mod_name::endpoints::init::<mx_sc_wasm_adapter::api::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<mx_sc_wasm_adapter::api::VmApiImpl>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident)* ) ) => {
        #[no_mangle]
        fn init() {
            mx_sc_wasm_adapter::mx_sc::external_view_contract::external_view_contract_constructor::<mx_sc_wasm_adapter::api::VmApiImpl>();
        }

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$endpoint_name::<mx_sc_wasm_adapter::mx_sc::api::ExternalViewApi<mx_sc_wasm_adapter::api::VmApiImpl>>();
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
