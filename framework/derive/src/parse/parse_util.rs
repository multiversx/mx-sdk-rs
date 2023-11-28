pub fn validate_attribute_args(args: syn::MetaList) {
    assert!(
        args.tokens.is_empty(),
        "No arguments expected in contract, module or proxy annotation."
    );
}


