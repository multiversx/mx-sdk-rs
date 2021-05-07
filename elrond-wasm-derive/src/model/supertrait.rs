use syn::punctuated::Punctuated;
use syn::token::Colon2;

/// Path to a Rust module containing a contract module.
pub type ModulePath = Punctuated<syn::PathSegment, Colon2>;

#[derive(Clone, Debug)]
pub struct Supertrait {
	pub full_path: syn::Path,
	pub trait_name: syn::PathSegment,
	pub module_path: ModulePath,
}
