#[macro_export]
macro_rules! allocator {
    () => {
        #[global_allocator]
        static ALLOC: multiversx_sc_wasm_adapter::wasm_alloc::FailAllocator =
            multiversx_sc_wasm_adapter::wasm_alloc::FailAllocator;
    };
    (leaking) => {
        #[global_allocator]
        static ALLOC: multiversx_sc_wasm_adapter::wasm_alloc::LeakingAllocator =
            multiversx_sc_wasm_adapter::wasm_alloc::LeakingAllocator::new();
    };
    (static64k) => {
        #[global_allocator]
        static ALLOC: multiversx_sc_wasm_adapter::wasm_alloc::StaticAllocator64K =
            multiversx_sc_wasm_adapter::wasm_alloc::StaticAllocator64K::new();
    };
    (wee_alloc) => {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    };
}

#[macro_export]
macro_rules! panic_handler {
    () => {
        #[panic_handler]
        fn panic_fmt(panic_info: &multiversx_sc_wasm_adapter::panic::PanicInfo) -> ! {
            multiversx_sc_wasm_adapter::panic::panic_fmt(panic_info)
        }

        #[lang = "eh_personality"]
        fn eh_personality() {}
    };
}

#[macro_export]
macro_rules! panic_handler_with_message {
    () => {
        #[panic_handler]
        fn panic_fmt(panic_info: &multiversx_sc_wasm_adapter::panic::PanicInfo) -> ! {
            multiversx_sc_wasm_adapter::panic::panic_fmt_with_message(panic_info)
        }

        #[lang = "eh_personality"]
        fn eh_personality() {}
    };
}

#[macro_export]
macro_rules! endpoints_old {
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
macro_rules! endpoints {
    ($mod_name:ident ( $($endpoint_name:ident => $method_name:ident)* ) ) => {
        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$method_name::<multiversx_sc_wasm_adapter::api::VmApiImpl>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident => $method_name:ident)* ) ) => {
        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            fn $endpoint_name() {
                $mod_name::endpoints::$method_name::<multiversx_sc_wasm_adapter::multiversx_sc::api::ExternalViewApi<multiversx_sc_wasm_adapter::api::VmApiImpl>>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_endpoints_old {
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
macro_rules! external_view_init {
    () => {
        #[no_mangle]
        fn init() {
            multiversx_sc_wasm_adapter::multiversx_sc::external_view_contract::external_view_contract_constructor::<multiversx_sc_wasm_adapter::api::VmApiImpl>();
        }
    };
}

#[macro_export]
macro_rules! async_callback {
    ($mod_name:ident) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        fn callBack() {
            $mod_name::endpoints::callBack::<multiversx_sc_wasm_adapter::api::VmApiImpl>();
        }
    };
}

#[macro_export]
macro_rules! async_callback_empty {
    () => {
        #[allow(non_snake_case)]
        #[no_mangle]
        fn callBack() {}
    };
}
