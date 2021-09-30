pub fn validate_attribute_args(args: syn::AttributeArgs) {
    assert!(
        args.is_empty(),
        "No arguments expected in contract, module or proxy annotation."
    );
}
