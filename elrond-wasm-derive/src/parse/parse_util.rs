pub fn validate_attribute_args(args: syn::AttributeArgs) {
	if !args.is_empty() {
		panic!("No arguments expected in contract, module or proxy annotation.");
	}
}
