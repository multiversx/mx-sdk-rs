use super::substitution_map::SubstitutionsMap;

pub fn substitutions() -> SubstitutionsMap {
    let mut substitutions = SubstitutionsMap::new();

    add_managed_types(&mut substitutions);
    add_special_methods(&mut substitutions);
    add_storage_mappers(&mut substitutions);

    substitutions
}

fn add_managed_type(substitutions: &mut SubstitutionsMap, type_name: &proc_macro2::TokenStream) {
    substitutions.add_substitution(
        quote!(#type_name::),
        quote!(elrond_wasm::types::#type_name::<Self::Api>::),
    );
    substitutions.add_substitution(
        quote!(#type_name),
        quote!(elrond_wasm::types::#type_name<Self::Api>),
    );
}

fn add_managed_type_with_generics(
    substitutions: &mut SubstitutionsMap,
    type_name: &proc_macro2::TokenStream,
) {
    substitutions.add_substitution(
        quote!(#type_name<Self::Api, ),
        quote!(#type_name<Self::Api, ),
    );
    substitutions.add_substitution(quote!(#type_name<), quote!(#type_name<Self::Api, ));
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

    add_managed_type_with_generics(substitutions, &quote!(ManagedVec));
    add_managed_type_with_generics(substitutions, &quote!(ManagedVarArgs));
    add_managed_type_with_generics(substitutions, &quote!(ManagedMultiResultVec));
    add_managed_type_with_generics(substitutions, &quote!(ManagedAsyncCallResult));
    add_managed_type_with_generics(substitutions, &quote!(ManagedCountedVarArgs));
    add_managed_type_with_generics(substitutions, &quote!(ManagedCountedMultiResultVec));
}

fn add_special_methods(substitutions: &mut SubstitutionsMap) {
    substitutions.add_substitution(
        quote!(.managed_into()),
        quote!(.managed_into(self.type_manager())),
    );

    substitutions.add_substitution(
        quote!(BigUint::zero()),
        quote!(self.types().big_uint_zero()),
    );
    substitutions.add_substitution(quote!(BigUint::from), quote!(self.types().big_uint_from));
    substitutions.add_substitution(quote!(BigInt::zero()), quote!(self.types().big_int_zero()));
    substitutions.add_substitution(quote!(BigInt::from), quote!(self.types().big_int_from));
    substitutions.add_substitution(
        quote!(ManagedBuffer::new()),
        quote!(self.types().managed_buffer_new()),
    );
    substitutions.add_substitution(
        quote!(ManagedBuffer::from),
        quote!(self.types().managed_buffer_from),
    );
    substitutions.add_substitution(
        quote!(ManagedVec::new()),
        quote!(self.types().managed_vec_new()),
    );
    substitutions.add_substitution(
        quote!(ManagedVec::from_single_item),
        quote!(self.types().managed_vec_from_single_item),
    );
    substitutions.add_substitution(
        quote!(ManagedVec::from),
        quote!(self.types().managed_vec_from),
    );
    substitutions.add_substitution(
        quote!(TokenIdentifier::egld()),
        quote!(self.types().token_identifier_egld()),
    );
    substitutions.add_substitution(
        quote!(TokenIdentifier::from),
        quote!(self.types().token_identifier_from),
    );
    substitutions.add_substitution(
        quote!(ManagedAddress::zero()),
        quote!(self.types().managed_address_zero()),
    );
    substitutions.add_substitution(
        quote!(ManagedAddress::from),
        quote!(self.types().managed_address_from),
    );
    substitutions.add_substitution(
        quote!(.unwrap_or_signal_error()),
        quote!(.unwrap_or_signal_error(self.raw_vm_api())),
    );
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
    add_managed_type_with_generics(substitutions, mapper_name);
}

fn add_storage_mappers(substitutions: &mut SubstitutionsMap) {
    add_storage_mapper_single_generic_arg(substitutions, &quote!(TokenAttributesMapper));
    add_storage_mapper_single_generic_arg(substitutions, &quote!(UserMapper));

    add_storage_mapper(substitutions, &quote!(LinkedListMapper));
    add_storage_mapper(substitutions, &quote!(MapMapper));
    add_storage_mapper(substitutions, &quote!(MapStorageMapper));
    add_storage_mapper(substitutions, &quote!(SetMapper));
    add_storage_mapper(substitutions, &quote!(UnorderedSetMapper));
    add_storage_mapper(substitutions, &quote!(SingleValueMapper));
    add_storage_mapper(substitutions, &quote!(VecMapper));
    add_storage_mapper(substitutions, &quote!(QueueMapper));
}
