use super::*;

use alloc::vec::Vec;
use multiversx_chain_core::types::{
    Address, BLSKey, BLSSignature, BoxedBytes, CodeMetadata, DurationMillis, DurationSeconds,
    EsdtLocalRole, EsdtTokenType, H256, TimestampMillis, TimestampSeconds,
};

impl TypeAbiFrom<Self> for H256 {}
impl AbiTypeFrom<Self> for H256 {}

impl AbiType for H256 {
    fn type_name() -> TypeName {
        "H256".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for H256 {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "H256".into()
    }
}

impl TypeAbiFrom<Self> for Address {}

impl TypeAbi for Address {
    type Abi = AddressAbi;

    fn type_name_rust() -> TypeName {
        "Address".into()
    }
}

impl TypeAbiFrom<Self> for BoxedBytes {}
impl AbiTypeFrom<Self> for BoxedBytes {}

impl AbiType for BoxedBytes {
    fn type_name() -> TypeName {
        "bytes".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for BoxedBytes {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "BoxedBytes".into()
    }
}

impl TypeAbiFrom<Self> for CodeMetadata {}
impl AbiTypeFrom<Self> for CodeMetadata {}

impl AbiType for CodeMetadata {
    fn type_name() -> TypeName {
        "CodeMetadata".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for CodeMetadata {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "CodeMetadata".into()
    }
}

impl TypeAbiFrom<Self> for BLSKey {}
impl AbiTypeFrom<Self> for BLSKey {}

impl AbiType for BLSKey {
    fn type_name() -> TypeName {
        <[u8; BLSKey::len()] as AbiType>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for BLSKey {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "BLSKey".into()
    }
}

impl TypeAbiFrom<Self> for BLSSignature {}
impl TypeAbiFrom<[u8; BLSSignature::len()]> for BLSSignature {}
impl TypeAbiFrom<BLSSignature> for [u8; BLSSignature::len()] {}
impl AbiTypeFrom<Self> for BLSSignature {}

impl AbiType for BLSSignature {
    fn type_name() -> TypeName {
        <[u8; BLSSignature::len()] as AbiType>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for BLSSignature {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "BLSSignature".into()
    }
}

impl TypeAbiFrom<Self> for EsdtTokenType {}
impl TypeAbiFrom<&Self> for EsdtTokenType {}
impl AbiTypeFrom<Self> for EsdtTokenType {}

// implementation originally generated via #[type_abi] attribute
impl AbiType for EsdtTokenType {
    fn type_name() -> TypeName {
        "EsdtTokenType".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        let type_names = TypeNames {
            abi: <Self as AbiType>::type_name(),
            rust: "EsdtTokenType".into(),
            specific: None,
        };
        if !accumulator.contains_type(&type_names.abi) {
            accumulator.reserve_type_name(type_names.clone());
            let mut variant_descriptions = Vec::new();
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "Fungible",
                0usize,
                Vec::new(),
            ));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "NonFungible",
                1usize,
                Vec::new(),
            ));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "SemiFungible",
                2usize,
                Vec::new(),
            ));
            variant_descriptions.push(EnumVariantDescription::new(&[], "Meta", 3usize, Vec::new()));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "Invalid",
                4usize,
                Vec::new(),
            ));
            accumulator.insert(
                type_names.clone(),
                TypeDescription::new(
                    &[],
                    type_names,
                    TypeContents::Enum(variant_descriptions),
                    &[
                        "TopDecode",
                        "TopEncode",
                        "NestedDecode",
                        "NestedEncode",
                        "Clone",
                        "PartialEq",
                        "Eq",
                        "Debug",
                        "ManagedVecItem",
                    ],
                ),
            );
        }
    }
}

impl TypeAbi for EsdtTokenType {
    type Abi = Self;
}

impl TypeAbiFrom<Self> for EsdtLocalRole {}
impl TypeAbiFrom<&Self> for EsdtLocalRole {}
impl AbiTypeFrom<Self> for EsdtLocalRole {}

// implementation originally generated via #[type_abi] attribute
impl AbiType for EsdtLocalRole {
    fn type_name() -> TypeName {
        "EsdtLocalRole".into()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        let type_names = TypeNames {
            abi: <Self as AbiType>::type_name(),
            rust: "EsdtLocalRole".into(),
            specific: None,
        };
        if !accumulator.contains_type(&type_names.abi) {
            accumulator.reserve_type_name(type_names.clone());
            let mut variant_descriptions = Vec::new();
            variant_descriptions.push(EnumVariantDescription::new(&[], "None", 0usize, Vec::new()));
            variant_descriptions.push(EnumVariantDescription::new(&[], "Mint", 1usize, Vec::new()));
            variant_descriptions.push(EnumVariantDescription::new(&[], "Burn", 2usize, Vec::new()));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "NftCreate",
                3usize,
                Vec::new(),
            ));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "NftAddQuantity",
                4usize,
                Vec::new(),
            ));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "NftBurn",
                5usize,
                Vec::new(),
            ));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "NftAddUri",
                6usize,
                Vec::new(),
            ));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "NftUpdateAttributes",
                7usize,
                Vec::new(),
            ));
            variant_descriptions.push(EnumVariantDescription::new(
                &[],
                "Transfer",
                8usize,
                Vec::new(),
            ));
            accumulator.insert(
                type_names.clone(),
                TypeDescription::new(
                    &[],
                    type_names,
                    TypeContents::Enum(variant_descriptions),
                    &[
                        "TopDecode",
                        "TopEncode",
                        "NestedDecode",
                        "NestedEncode",
                        "Clone",
                        "PartialEq",
                        "Eq",
                        "Debug",
                        "Copy",
                    ],
                ),
            );
        }
    }
}

impl TypeAbi for EsdtLocalRole {
    type Abi = Self;
}

impl TypeAbiFrom<Self> for DurationMillis {}
impl AbiTypeFrom<Self> for DurationMillis {}

impl AbiType for DurationMillis {
    fn type_name() -> TypeName {
        "u64".into()
    }

    fn type_name_specific() -> Option<TypeName> {
        Some("DurationMillis".into())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for DurationMillis {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "DurationMillis".into()
    }
}

impl TypeAbiFrom<Self> for DurationSeconds {}
impl AbiTypeFrom<Self> for DurationSeconds {}

impl AbiType for DurationSeconds {
    fn type_name() -> TypeName {
        "u64".into()
    }

    fn type_name_specific() -> Option<TypeName> {
        Some("DurationSeconds".into())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for DurationSeconds {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "DurationSeconds".into()
    }
}

impl TypeAbiFrom<Self> for TimestampMillis {}
impl AbiTypeFrom<Self> for TimestampMillis {}

impl AbiType for TimestampMillis {
    fn type_name() -> TypeName {
        "u64".into()
    }

    fn type_name_specific() -> Option<TypeName> {
        Some("TimestampMillis".into())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for TimestampMillis {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "TimestampMillis".into()
    }
}

impl TypeAbiFrom<Self> for TimestampSeconds {}
impl AbiTypeFrom<Self> for TimestampSeconds {}

impl AbiType for TimestampSeconds {
    fn type_name() -> TypeName {
        "u64".into()
    }

    fn type_name_specific() -> Option<TypeName> {
        Some("TimestampSeconds".into())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbi for TimestampSeconds {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        "TimestampSeconds".into()
    }
}
