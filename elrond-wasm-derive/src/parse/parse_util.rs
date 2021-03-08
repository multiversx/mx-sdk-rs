pub fn extract_struct_name(args: syn::AttributeArgs) -> syn::Path {
	if args.len() != 1 {
		panic!("Exactly one argument expected in contract annotation, specifying the implementation struct name.");
	}

	if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = args.get(0).unwrap() {
		path.clone()
	} else {
		panic!("Malformed contract implementation struct name")
	}
}
