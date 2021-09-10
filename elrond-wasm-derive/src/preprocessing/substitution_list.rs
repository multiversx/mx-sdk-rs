use super::substitution_map::SubstitutionsMap;

pub fn substitutions() -> SubstitutionsMap {
    let mut substitutions = SubstitutionsMap::new();
    substitutions.add_substitution(
        quote!(BigInt),
        quote!(elrond_wasm::types::BigInt<Self::Api>),
    );
    substitutions.add_substitution(
        quote!(BigUint),
        quote!(elrond_wasm::types::BigUint<Self::Api>),
    );
    substitutions.add_substitution(quote!(BigUint::from), quote!(self.types().big_uint_from));
    substitutions.add_substitution(
        quote!(.managed_into()),
        quote!(.managed_into(self.type_manager())),
    );
    substitutions.add_substitution(
        quote!(ManagedBuffer),
        quote!(elrond_wasm::types::ManagedBuffer<Self::Api>),
    );
    substitutions.add_substitution(
        quote!(EllipticCurve),
        quote!(elrond_wasm::types::EllipticCurve<Self::Api>),
    );
    substitutions.add_substitution(
        quote!(ManagedAddress),
        quote!(elrond_wasm::types::ManagedAddress<Self::Api>),
    );
    substitutions.add_substitution(
        quote!(TokenIdentifier),
        quote!(elrond_wasm::types::TokenIdentifier<Self::Api>),
    );
    substitutions.add_substitution(
        quote!(ManagedSCError),
        quote!(elrond_wasm::types::ManagedSCError<Self::Api>),
    );
    substitutions.add_substitution(
        quote!(AsyncCall),
        quote!(elrond_wasm::types::AsyncCall<Self::Api>),
    );

    substitutions.add_substitution(quote!(SendApi), quote!(Api));
    substitutions.add_substitution(quote!(TypeManager), quote!(Api));
    substitutions.add_substitution(quote!(Storage), quote!(Api));
    substitutions
}
