use crate::{model::Supertrait, parse::split_path_last};

pub fn parse_supertrait(supertrait: &syn::TypeParamBound) -> Supertrait {
    match supertrait {
        syn::TypeParamBound::Trait(t) => {
            if let Some((leading_segments, last_segment)) = split_path_last(&t.path) {
                assert!(
                    last_segment.arguments.is_empty(),
                    "No generics allowed when specifying contract supertraits."
                );
                Supertrait {
                    full_path: t.path.clone(),
                    trait_name: last_segment,
                    module_path: leading_segments,
                }
            } else {
                panic!("All contract module supertraits must be specfied with some module specifier (e.g. `path::to::module::ContractName`)");
            }
        },
        _ => panic!("Contract trait can only extend other traits."),
    }
}
