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

        fn __set_panic_hook() {}
    };
}

#[macro_export]
macro_rules! panic_handler_with_message {
    () => {
        #[panic_handler]
        fn panic_fmt(panic_info: &multiversx_sc_wasm_adapter::panic::PanicInfo) -> ! {
            multiversx_sc_wasm_adapter::panic::panic_fmt_with_message(panic_info)
        }

        fn __set_panic_hook() {}
    };
}

#[macro_export]
macro_rules! panic_handler_std {
    () => {
        fn __set_panic_hook() {
            multiversx_sc_wasm_adapter::panic_std::set_panic_hook();
        }
    };
}

#[macro_export]
macro_rules! panic_handler_std_with_message {
    () => {
        fn __set_panic_hook() {
            multiversx_sc_wasm_adapter::panic_std::set_panic_hook_with_message();
        }
    };
}

#[macro_export]
macro_rules! endpoints {
    ($mod_name:ident ( $($endpoint_name:ident => $method_name:ident)* ) ) => {
        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            fn $endpoint_name() {
                __set_panic_hook();
                $mod_name::__wasm__endpoints__::$method_name::<multiversx_sc_wasm_adapter::api::VmApiImpl>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_endpoints {
    ($mod_name:ident ( $($endpoint_name:ident => $method_name:ident)* ) ) => {
        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            fn $endpoint_name() {
                __set_panic_hook();
                $mod_name::__wasm__endpoints__::$method_name::<multiversx_sc_wasm_adapter::multiversx_sc::api::ExternalViewApi<multiversx_sc_wasm_adapter::api::VmApiImpl>>();
            }
        )*
    };
}

#[macro_export]
macro_rules! external_view_init {
    () => {
        #[unsafe(no_mangle)]
        fn init() {
            __set_panic_hook();
            multiversx_sc_wasm_adapter::multiversx_sc::external_view_contract::external_view_contract_constructor::<multiversx_sc_wasm_adapter::api::VmApiImpl>();
        }
    };
}

#[macro_export]
macro_rules! async_callback {
    ($mod_name:ident) => {
        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        fn callBack() {
            __set_panic_hook();
            $mod_name::__wasm__endpoints__::callBack::<multiversx_sc_wasm_adapter::api::VmApiImpl>(
            );
        }
    };
}

#[macro_export]
macro_rules! async_callback_empty {
    () => {
        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        fn callBack() {}
    };
}
