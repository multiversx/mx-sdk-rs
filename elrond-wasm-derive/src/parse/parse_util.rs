pub fn validate_attribute_args(args: syn::AttributeArgs) {
	if args.len() > 0 {
		panic!("No arguments expected in contract, module or proxy annotation.");
	}
}
