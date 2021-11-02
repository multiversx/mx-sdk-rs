#[macro_export]
macro_rules! create_endpoint_macro {
    ($macro_name:ident, $ns:ident) => { create_endpoint_macro!{$macro_name, $ns, $} };

    ($macro_name:ident, $ns:ident, $dol:tt) => {
        macro_rules! $macro_name {
            ($name:ident) => {
                #[no_mangle]
                fn $name() {
                    $ns::endpoints::$name(elrond_wasm_node::vm_api());
                }
            };
        }
    };
}
