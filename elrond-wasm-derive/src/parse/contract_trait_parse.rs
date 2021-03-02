pub fn parse_contract_trait(
	args: syn::AttributeArgs,
	contract_trait: &syn::ItemTrait,
) -> ContractTrait {
	let contract_impl_name = extract_struct_name(args);

	let docs = extract_doc(contract_trait.attrs.as_slice());

	let supertrait_paths: Vec<syn::Path> = contract_trait
		.supertraits
		.iter()
		.map(|supertrait| match supertrait {
			syn::TypeParamBound::Trait(t) => t.path.clone(),
			_ => panic!("Contract trait can only extend other traits."),
		})
		.collect();

	let methods: Vec<Method> = contract_trait
		.items
		.iter()
		.map(|itm| match itm {
			syn::TraitItem::Method(m) => Method::parse(m),
			_ => panic!("Only methods allowed in contract traits"),
		})
		.collect();

	Contract {
		docs,
		trait_name: contract_trait.ident.clone(),
		contract_impl_name,
		supertrait_paths,
		methods,
	}
}

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

pub fn extract_methods(contract_trait: &syn::ItemTrait) -> Vec<syn::TraitItemMethod> {
	contract_trait
		.items
		.iter()
		.filter_map(|itm| match itm {
			syn::TraitItem::Method(m) => {
				let msig = &m.sig;
				let bad_self_ref = format!(
					"ABI function `{}` must have `&self` as its first argument.",
					msig.ident.to_string()
				);
				match msig.inputs[0] {
					syn::FnArg::Receiver(ref selfref) => {
						if selfref.mutability.is_some() {
							panic!("{}", bad_self_ref)
						}
					},
					_ => panic!("{}", bad_self_ref),
				}

				Some(m.clone())
			},
			_ => None,
		})
		.collect()
}

