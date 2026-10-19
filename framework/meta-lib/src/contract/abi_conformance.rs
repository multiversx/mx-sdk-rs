use multiversx_sc::abi::{
    ContractAbi, EndpointAbi, InputAbi, OutputAbi, TypeContents, TypeDescription, TypeNames,
};

/// Checks a contract's own ABI against every spec it declared via `#[multiversx_sc::contract(
/// implements_abi = ..., implements_abi_exactly = ...)]` (recorded on `ContractAbi::implements_abi`/
/// `ContractAbi::implements_abi_exactly`).
///
/// `implements_abi` specs must be a subset of `contract_abi`'s exports; `implements_abi_exactly` specs
/// must match `contract_abi`'s exports exactly (each spec independently). A contract that
/// declares neither is unaffected: this is then a no-op.
pub fn validate_abi_conformance(contract_abi: &ContractAbi) -> Result<(), String> {
    for spec in &contract_abi.implements_abi {
        check_contains(contract_abi, spec)?;
    }
    for spec in &contract_abi.implements_abi_exactly {
        check_contains(contract_abi, spec)?;
        check_no_extra_exports(contract_abi, spec)?;
    }
    Ok(())
}

/// Every export in `spec` must be present in `actual`, with matching wire-relevant fields.
/// `actual` may have additional exports beyond what `spec` requires.
fn check_contains(actual: &ContractAbi, spec: &ContractAbi) -> Result<(), String> {
    for spec_endpoint in spec.iter_all_exports() {
        let actual_endpoint = find_export(actual, &spec_endpoint.name).ok_or_else(|| {
            format!(
                "missing endpoint '{}' required by ABI spec '{}'",
                spec_endpoint.name, spec.name
            )
        })?;
        check_endpoint_matches(actual, actual_endpoint, spec, spec_endpoint)?;
    }
    Ok(())
}

/// `actual`'s exports must be exactly `spec`'s exports (as a set of names) — no more, no less.
/// Assumes `check_contains(actual, spec)` already passed (missing exports are not re-checked
/// here, only extras).
fn check_no_extra_exports(actual: &ContractAbi, spec: &ContractAbi) -> Result<(), String> {
    for actual_endpoint in actual.iter_all_exports() {
        if find_export(spec, &actual_endpoint.name).is_none() {
            return Err(format!(
                "endpoint '{}' is not part of the exact ABI spec '{}'",
                actual_endpoint.name, spec.name
            ));
        }
    }
    Ok(())
}

fn find_export<'a>(contract_abi: &'a ContractAbi, name: &str) -> Option<&'a EndpointAbi> {
    contract_abi.iter_all_exports().find(|e| e.name == name)
}

/// Compares wire-relevant fields only: `endpoint_type`, `mutability`, `payable_in_tokens` (as a
/// set), and inputs/outputs (count, and per-position ABI type name + multi-value flag, diving
/// into `type_descriptions` for custom types). Docs, title, labels, `only_owner`/`only_admin`,
/// `allow_multiple_var_args` and Rust type names are intentionally ignored.
fn check_endpoint_matches(
    actual_contract: &ContractAbi,
    actual: &EndpointAbi,
    spec_contract: &ContractAbi,
    spec: &EndpointAbi,
) -> Result<(), String> {
    let ctx = format!("endpoint '{}' (spec '{}')", spec.name, spec_contract.name);

    if actual.endpoint_type != spec.endpoint_type {
        return Err(format!(
            "{ctx}: endpoint type mismatch ({:?} vs spec's {:?})",
            actual.endpoint_type, spec.endpoint_type
        ));
    }
    if actual.mutability != spec.mutability {
        return Err(format!(
            "{ctx}: mutability mismatch ({:?} vs spec's {:?})",
            actual.mutability, spec.mutability
        ));
    }
    if !same_token_set(&actual.payable_in_tokens, &spec.payable_in_tokens) {
        return Err(format!(
            "{ctx}: payable tokens mismatch ({:?} vs spec's {:?})",
            actual.payable_in_tokens, spec.payable_in_tokens
        ));
    }

    if actual.inputs.len() != spec.inputs.len() {
        return Err(format!(
            "{ctx}: input count mismatch ({} vs spec's {})",
            actual.inputs.len(),
            spec.inputs.len()
        ));
    }
    for (actual_input, spec_input) in actual.inputs.iter().zip(spec.inputs.iter()) {
        check_input_matches(
            actual_contract,
            actual_input,
            spec_contract,
            spec_input,
            &ctx,
        )?;
    }

    if actual.outputs.len() != spec.outputs.len() {
        return Err(format!(
            "{ctx}: output count mismatch ({} vs spec's {})",
            actual.outputs.len(),
            spec.outputs.len()
        ));
    }
    for (actual_output, spec_output) in actual.outputs.iter().zip(spec.outputs.iter()) {
        check_output_matches(
            actual_contract,
            actual_output,
            spec_contract,
            spec_output,
            &ctx,
        )?;
    }

    Ok(())
}

fn same_token_set(a: &[String], b: &[String]) -> bool {
    a.len() == b.len() && a.iter().all(|token| b.contains(token))
}

fn check_input_matches(
    actual_contract: &ContractAbi,
    actual: &InputAbi,
    spec_contract: &ContractAbi,
    spec: &InputAbi,
    ctx: &str,
) -> Result<(), String> {
    if actual.multi_arg != spec.multi_arg {
        return Err(format!(
            "{ctx}: input '{}' multi-arg mismatch",
            spec.arg_name
        ));
    }
    let ctx = format!("{ctx}: input '{}'", spec.arg_name);
    check_type_matches(
        actual_contract,
        &actual.type_names,
        spec_contract,
        &spec.type_names,
        &ctx,
    )
}

fn check_output_matches(
    actual_contract: &ContractAbi,
    actual: &OutputAbi,
    spec_contract: &ContractAbi,
    spec: &OutputAbi,
    ctx: &str,
) -> Result<(), String> {
    if actual.multi_result != spec.multi_result {
        return Err(format!("{ctx}: output multi-result mismatch"));
    }
    let ctx = format!("{ctx}: output");
    check_type_matches(
        actual_contract,
        &actual.type_names,
        spec_contract,
        &spec.type_names,
        &ctx,
    )
}

/// Type identity is by ABI name only (`TypeNames.abi`); Rust type names never matter. When the
/// spec's type has a real (custom) description, the same-named type on the `actual` side must
/// also have a description, and the two must be structurally identical — a name collision with
/// mismatched structure is always a hard error, never silently accepted.
fn check_type_matches(
    actual_contract: &ContractAbi,
    actual: &TypeNames,
    spec_contract: &ContractAbi,
    spec: &TypeNames,
    ctx: &str,
) -> Result<(), String> {
    if actual.abi != spec.abi {
        return Err(format!(
            "{ctx}: type mismatch ('{}' vs spec's '{}')",
            actual.abi, spec.abi
        ));
    }

    let Some(spec_description) = spec_contract.type_descriptions.find(&spec.abi) else {
        // The spec has no structural description for this type (e.g. it's a primitive) —
        // nothing further to compare.
        return Ok(());
    };
    if !spec_description.contents.is_specified() {
        return Ok(());
    }

    let actual_description = actual_contract
        .type_descriptions
        .find(&actual.abi)
        .ok_or_else(|| {
            format!(
                "{ctx}: type '{}' has no description in the contract's ABI, but spec '{}' describes it",
                actual.abi, spec_contract.name
            )
        })?;

    check_type_description_matches(
        actual_contract,
        actual_description,
        spec_contract,
        spec_description,
        ctx,
    )
}

fn check_type_description_matches(
    actual_contract: &ContractAbi,
    actual: &TypeDescription,
    spec_contract: &ContractAbi,
    spec: &TypeDescription,
    ctx: &str,
) -> Result<(), String> {
    match (&actual.contents, &spec.contents) {
        (TypeContents::Struct(actual_fields), TypeContents::Struct(spec_fields)) => {
            if actual_fields.len() != spec_fields.len() {
                return Err(format!(
                    "{ctx}: type '{}' field count mismatch ({} vs spec's {})",
                    spec.names.abi,
                    actual_fields.len(),
                    spec_fields.len()
                ));
            }
            // Struct fields are encoded positionally (no field names on the wire), so field
            // *order* is part of the ABI contract, not just the set of names — matched by
            // position (`zip`), not by name lookup.
            for (position, (actual_field, spec_field)) in
                actual_fields.iter().zip(spec_fields.iter()).enumerate()
            {
                if actual_field.name != spec_field.name {
                    return Err(format!(
                        "{ctx}: type '{}' field {position} name mismatch ('{}' vs spec's '{}') — field order matters",
                        spec.names.abi, actual_field.name, spec_field.name
                    ));
                }
                let field_ctx = format!(
                    "{ctx}: type '{}', field '{}'",
                    spec.names.abi, spec_field.name
                );
                check_type_matches(
                    actual_contract,
                    &actual_field.field_type,
                    spec_contract,
                    &spec_field.field_type,
                    &field_ctx,
                )?;
            }
            Ok(())
        }
        (TypeContents::Enum(actual_variants), TypeContents::Enum(spec_variants)) => {
            if actual_variants.len() != spec_variants.len() {
                return Err(format!(
                    "{ctx}: enum '{}' variant count mismatch ({} vs spec's {})",
                    spec.names.abi,
                    actual_variants.len(),
                    spec_variants.len()
                ));
            }
            for spec_variant in spec_variants {
                let actual_variant = actual_variants
                    .iter()
                    .find(|v| v.name == spec_variant.name)
                    .ok_or_else(|| {
                        format!(
                            "{ctx}: enum '{}' is missing variant '{}'",
                            spec.names.abi, spec_variant.name
                        )
                    })?;
                if actual_variant.discriminant != spec_variant.discriminant {
                    return Err(format!(
                        "{ctx}: enum '{}', variant '{}' discriminant mismatch ({} vs spec's {})",
                        spec.names.abi,
                        spec_variant.name,
                        actual_variant.discriminant,
                        spec_variant.discriminant
                    ));
                }
                if actual_variant.fields.len() != spec_variant.fields.len() {
                    return Err(format!(
                        "{ctx}: enum '{}', variant '{}' field count mismatch",
                        spec.names.abi, spec_variant.name
                    ));
                }
                // Same as struct fields: a variant's payload is encoded positionally, so field
                // order matters here too.
                for (position, (actual_field, spec_field)) in actual_variant
                    .fields
                    .iter()
                    .zip(spec_variant.fields.iter())
                    .enumerate()
                {
                    if actual_field.name != spec_field.name {
                        return Err(format!(
                            "{ctx}: enum '{}', variant '{}' field {position} name mismatch ('{}' vs spec's '{}') — field order matters",
                            spec.names.abi, spec_variant.name, actual_field.name, spec_field.name
                        ));
                    }
                    let field_ctx = format!(
                        "{ctx}: enum '{}', variant '{}', field '{}'",
                        spec.names.abi, spec_variant.name, spec_field.name
                    );
                    check_type_matches(
                        actual_contract,
                        &actual_field.field_type,
                        spec_contract,
                        &spec_field.field_type,
                        &field_ctx,
                    )?;
                }
            }
            Ok(())
        }
        (
            TypeContents::ExplicitEnum(actual_variants),
            TypeContents::ExplicitEnum(spec_variants),
        ) => {
            if actual_variants.len() != spec_variants.len()
                || spec_variants
                    .iter()
                    .any(|sv| !actual_variants.iter().any(|av| av.name == sv.name))
            {
                return Err(format!(
                    "{ctx}: explicit enum '{}' variant set mismatch",
                    spec.names.abi
                ));
            }
            Ok(())
        }
        _ => Err(format!(
            "{ctx}: type '{}' has a different shape (struct/enum/explicit-enum) than spec '{}'",
            spec.names.abi, spec_contract.name
        )),
    }
}

#[cfg(test)]
mod tests {
    use multiversx_sc::abi::{
        BuildInfoAbi, EndpointMutabilityAbi, EndpointTypeAbi, EnumVariantDescription,
        StructFieldDescription, TypeDescription, TypeDescriptionContainer,
        TypeDescriptionContainerImpl,
    };

    use super::*;

    fn empty_contract(name: &str) -> ContractAbi {
        ContractAbi::new(BuildInfoAbi::default(), &[], name, false)
    }

    fn endpoint(name: &str, mutability: EndpointMutabilityAbi) -> EndpointAbi {
        EndpointAbi::new(name, name, mutability, EndpointTypeAbi::Endpoint)
    }

    fn names(abi_name: &str) -> TypeNames {
        TypeNames::from_abi(abi_name.to_string())
    }

    #[test]
    fn contains_passes_when_endpoints_present() {
        let mut actual = empty_contract("Actual");
        actual
            .endpoints
            .push(endpoint("foo", EndpointMutabilityAbi::Mutable));
        actual
            .endpoints
            .push(endpoint("bar", EndpointMutabilityAbi::Readonly));

        let mut spec = empty_contract("Spec");
        spec.endpoints
            .push(endpoint("foo", EndpointMutabilityAbi::Mutable));

        actual.implements_abi.push(spec);

        assert!(validate_abi_conformance(&actual).is_ok());
    }

    #[test]
    fn contains_fails_when_endpoint_missing() {
        let actual = empty_contract("Actual");

        let mut spec = empty_contract("Spec");
        spec.endpoints
            .push(endpoint("foo", EndpointMutabilityAbi::Mutable));

        let mut actual = actual;
        actual.implements_abi.push(spec);

        let err = validate_abi_conformance(&actual).unwrap_err();
        assert!(err.contains("missing endpoint 'foo'"));
    }

    #[test]
    fn contains_fails_on_input_type_mismatch() {
        let mut actual_endpoint = endpoint("foo", EndpointMutabilityAbi::Mutable);
        actual_endpoint.inputs.push(InputAbi {
            arg_name: "x".to_string(),
            type_names: names("u32"),
            multi_arg: false,
        });
        let mut actual = empty_contract("Actual");
        actual.endpoints.push(actual_endpoint);

        let mut spec_endpoint = endpoint("foo", EndpointMutabilityAbi::Mutable);
        spec_endpoint.inputs.push(InputAbi {
            arg_name: "x".to_string(),
            type_names: names("BigUint"),
            multi_arg: false,
        });
        let mut spec = empty_contract("Spec");
        spec.endpoints.push(spec_endpoint);

        actual.implements_abi.push(spec);

        let err = validate_abi_conformance(&actual).unwrap_err();
        assert!(err.contains("type mismatch"));
    }

    #[test]
    fn exact_passes_when_export_sets_match() {
        let mut actual = empty_contract("Actual");
        actual
            .endpoints
            .push(endpoint("foo", EndpointMutabilityAbi::Mutable));

        let mut spec = empty_contract("Spec");
        spec.endpoints
            .push(endpoint("foo", EndpointMutabilityAbi::Mutable));

        actual.implements_abi_exactly.push(spec);

        assert!(validate_abi_conformance(&actual).is_ok());
    }

    #[test]
    fn exact_fails_on_extra_endpoint() {
        let mut actual = empty_contract("Actual");
        actual
            .endpoints
            .push(endpoint("foo", EndpointMutabilityAbi::Mutable));
        actual
            .endpoints
            .push(endpoint("extra", EndpointMutabilityAbi::Mutable));

        let mut spec = empty_contract("Spec");
        spec.endpoints
            .push(endpoint("foo", EndpointMutabilityAbi::Mutable));

        actual.implements_abi_exactly.push(spec);

        let err = validate_abi_conformance(&actual).unwrap_err();
        assert!(err.contains("endpoint 'extra' is not part of the exact ABI spec"));
    }

    #[test]
    fn same_abi_name_different_structure_fails() {
        let mut actual_endpoint = endpoint("foo", EndpointMutabilityAbi::Mutable);
        actual_endpoint.outputs.push(OutputAbi {
            output_name: String::new(),
            type_names: names("MyStruct"),
            multi_result: false,
        });
        let mut actual = empty_contract("Actual");
        let mut actual_types = TypeDescriptionContainerImpl::new();
        actual_types.insert(
            names("MyStruct"),
            TypeDescription::new(
                &[],
                names("MyStruct"),
                TypeContents::Struct(vec![StructFieldDescription::new(&[], "a", names("u32"))]),
                &[],
            ),
        );
        actual.type_descriptions = actual_types;
        actual.endpoints.push(actual_endpoint);

        let mut spec_endpoint = endpoint("foo", EndpointMutabilityAbi::Mutable);
        spec_endpoint.outputs.push(OutputAbi {
            output_name: String::new(),
            type_names: names("MyStruct"),
            multi_result: false,
        });
        let mut spec = empty_contract("Spec");
        let mut spec_types = TypeDescriptionContainerImpl::new();
        spec_types.insert(
            names("MyStruct"),
            TypeDescription::new(
                &[],
                names("MyStruct"),
                TypeContents::Struct(vec![StructFieldDescription::new(&[], "b", names("u32"))]),
                &[],
            ),
        );
        spec.type_descriptions = spec_types;
        spec.endpoints.push(spec_endpoint);

        actual.implements_abi.push(spec);

        let err = validate_abi_conformance(&actual).unwrap_err();
        assert!(err.contains("field 0 name mismatch"));
    }

    #[test]
    fn struct_with_same_fields_in_different_order_fails() {
        let struct_with_fields = |field_names: &[&str]| {
            TypeContents::Struct(
                field_names
                    .iter()
                    .map(|name| StructFieldDescription::new(&[], name, names("u32")))
                    .collect(),
            )
        };

        let mut actual_endpoint = endpoint("foo", EndpointMutabilityAbi::Mutable);
        actual_endpoint.outputs.push(OutputAbi {
            output_name: String::new(),
            type_names: names("MyStruct"),
            multi_result: false,
        });
        let mut actual = empty_contract("Actual");
        let mut actual_types = TypeDescriptionContainerImpl::new();
        actual_types.insert(
            names("MyStruct"),
            TypeDescription::new(&[], names("MyStruct"), struct_with_fields(&["a", "b"]), &[]),
        );
        actual.type_descriptions = actual_types;
        actual.endpoints.push(actual_endpoint);

        let mut spec_endpoint = endpoint("foo", EndpointMutabilityAbi::Mutable);
        spec_endpoint.outputs.push(OutputAbi {
            output_name: String::new(),
            type_names: names("MyStruct"),
            multi_result: false,
        });
        let mut spec = empty_contract("Spec");
        let mut spec_types = TypeDescriptionContainerImpl::new();
        // same field names, reversed order: encodes differently on the wire, must not pass.
        spec_types.insert(
            names("MyStruct"),
            TypeDescription::new(&[], names("MyStruct"), struct_with_fields(&["b", "a"]), &[]),
        );
        spec.type_descriptions = spec_types;
        spec.endpoints.push(spec_endpoint);

        actual.implements_abi.push(spec);

        let err = validate_abi_conformance(&actual).unwrap_err();
        assert!(err.contains("field order matters"));
    }

    #[test]
    fn enum_variant_mismatch_fails() {
        let variant =
            |name: &str, disc: usize| EnumVariantDescription::new(&[], name, disc, vec![]);

        let mut actual_endpoint = endpoint("foo", EndpointMutabilityAbi::Mutable);
        actual_endpoint.outputs.push(OutputAbi {
            output_name: String::new(),
            type_names: names("MyEnum"),
            multi_result: false,
        });
        let mut actual = empty_contract("Actual");
        let mut actual_types = TypeDescriptionContainerImpl::new();
        actual_types.insert(
            names("MyEnum"),
            TypeDescription::new(
                &[],
                names("MyEnum"),
                TypeContents::Enum(vec![variant("A", 0), variant("B", 1)]),
                &[],
            ),
        );
        actual.type_descriptions = actual_types;
        actual.endpoints.push(actual_endpoint);

        let mut spec_endpoint = endpoint("foo", EndpointMutabilityAbi::Mutable);
        spec_endpoint.outputs.push(OutputAbi {
            output_name: String::new(),
            type_names: names("MyEnum"),
            multi_result: false,
        });
        let mut spec = empty_contract("Spec");
        let mut spec_types = TypeDescriptionContainerImpl::new();
        spec_types.insert(
            names("MyEnum"),
            TypeDescription::new(
                &[],
                names("MyEnum"),
                TypeContents::Enum(vec![variant("A", 0), variant("B", 2)]),
                &[],
            ),
        );
        spec.type_descriptions = spec_types;
        spec.endpoints.push(spec_endpoint);

        actual.implements_abi_exactly.push(spec);

        let err = validate_abi_conformance(&actual).unwrap_err();
        assert!(err.contains("discriminant mismatch"));
    }
}
