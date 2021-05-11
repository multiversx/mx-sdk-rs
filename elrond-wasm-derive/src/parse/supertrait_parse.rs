use crate::model::Supertrait;

pub fn parse_supertrait(supertrait: &syn::TypeParamBound) -> Supertrait {
	match supertrait {
		syn::TypeParamBound::Trait(t) => {
			assert!(t.path.segments.len() >= 2, "All contract module supertraits must be specfied with some module specifier (e.g. `path::to::module::ContractName`)");
			let mut remaining_segments = t.path.segments.clone();
			let last_segment = remaining_segments.pop().unwrap().into_value();
			assert!(
				last_segment.arguments.is_empty(),
				"No generics allowed when specifying contract supertraits."
			);
			Supertrait {
				full_path: t.path.clone(),
				trait_name: last_segment,
				module_path: remaining_segments,
			}
		},
		_ => panic!("Contract trait can only extend other traits."),
	}
}
