use super::substitution_map::SubstitutionsMap;

pub fn substitutions() -> SubstitutionsMap {
    let mut substitutions = SubstitutionsMap::new();

    substitutions.add_substitution(
        quote!(.managed_into()),
        quote!(.managed_into(self.type_manager())),
    );

    add_managed_types(&mut substitutions);
    add_storage_mappers(&mut substitutions);

    substitutions
}

fn add_managed_type(substitutions: &mut SubstitutionsMap, type_name: &proc_macro2::TokenStream) {
    substitutions.add_substitution(
        type_name.clone(),
        quote!(elrond_wasm::types::#type_name<Self::Api>),
    );
}

fn add_managed_type_with_generics(
    substitutions: &mut SubstitutionsMap,
    alias: &proc_macro2::TokenStream,
    type_name: &proc_macro2::TokenStream,
) {
    substitutions.add_substitution(quote!(#alias<Self::Api, ), quote!(#type_name<Self::Api, ));
    substitutions.add_substitution(quote!(#alias<), quote!(#type_name<Self::Api, ));
}

fn add_managed_types(substitutions: &mut SubstitutionsMap) {
    add_managed_type(substitutions, &quote!(BigInt));
    add_managed_type(substitutions, &quote!(BigUint));
    add_managed_type(substitutions, &quote!(ManagedBuffer));
    add_managed_type(substitutions, &quote!(EllipticCurve));
    add_managed_type(substitutions, &quote!(ManagedAddress));
    add_managed_type(substitutions, &quote!(TokenIdentifier));
    add_managed_type(substitutions, &quote!(ManagedSCError));
    add_managed_type(substitutions, &quote!(AsyncCall));
    add_managed_type(substitutions, &quote!(ManagedAsyncCallError));

    add_managed_type_with_generics(substitutions, &quote!(ManagedVec), &quote!(ManagedVec));
    add_managed_type_with_generics(
        substitutions,
        &quote!(ManagedVarArgs),
        &quote!(ManagedMultiResultVec),
    );
    add_managed_type_with_generics(
        substitutions,
        &quote!(ManagedMultiResultVec),
        &quote!(ManagedMultiResultVec),
    );
    add_managed_type_with_generics(
        substitutions,
        &quote!(ManagedAsyncCallResult),
        &quote!(ManagedAsyncCallResult),
    );

    substitutions.add_substitution(quote!(BigUint::from), quote!(self.types().big_uint_from));
}

fn add_storage_mapper_single_generic_arg(
    substitutions: &mut SubstitutionsMap,
    mapper_name: &proc_macro2::TokenStream,
) {
    substitutions.add_substitution(
        quote!(#mapper_name<Self::Api>),
        quote!(#mapper_name<Self::Api>),
    );
    substitutions.add_substitution(quote!(#mapper_name), quote!(#mapper_name<Self::Api>));
}

fn add_storage_mapper(
    substitutions: &mut SubstitutionsMap,
    mapper_name: &proc_macro2::TokenStream,
) {
    add_managed_type_with_generics(substitutions, mapper_name, mapper_name);
}

fn add_storage_mappers(substitutions: &mut SubstitutionsMap) {
    add_storage_mapper_single_generic_arg(substitutions, &quote!(TokenAttributesMapper));
    add_storage_mapper_single_generic_arg(substitutions, &quote!(UserMapper));

    add_storage_mapper(substitutions, &quote!(LinkedListMapper));
    add_storage_mapper(substitutions, &quote!(MapMapper));
    add_storage_mapper(substitutions, &quote!(MapStorageMapper));
    add_storage_mapper(substitutions, &quote!(SetMapper));
    add_storage_mapper(substitutions, &quote!(SingleValueMapper));
    add_storage_mapper(substitutions, &quote!(VecMapper));
    add_storage_mapper(substitutions, &quote!(QueueMapper));
}
