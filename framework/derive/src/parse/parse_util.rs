pub fn validate_attribute_args(args: proc_macro::TokenStream) {
    assert!(
        args.is_empty(),
        "No arguments expected in contract, module or proxy annotation."
    );
}
