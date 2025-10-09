use super::*;

use alloc::vec::Vec;
use multiversx_chain_core::types::{
    Address, BLSKey, BLSSignature, BoxedBytes, CodeMetadata, EsdtLocalRole, EsdtTokenType, H256,
};

impl TypeAbiFrom<Self> for H256 {}

impl TypeAbi for H256 {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "H256".into()
    }

    fn type_name_rust() -> TypeName {
        "H256".into()
    }
}

impl TypeAbiFrom<Self> for Address {}

impl TypeAbi for Address {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "Address".into()
    }

    fn type_name_rust() -> TypeName {
        "Address".into()
    }
}

impl TypeAbiFrom<Self> for BoxedBytes {}

impl TypeAbi for BoxedBytes {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "bytes".into()
    }

    fn type_name_rust() -> TypeName {
        "BoxedBytes".into()
    }
}

impl TypeAbiFrom<Self> for CodeMetadata {}
impl TypeAbi for CodeMetadata {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "CodeMetadata".into()
    }

    fn type_name_rust() -> TypeName {
        "CodeMetadata".into()
    }
}

impl TypeAbiFrom<Self> for BLSKey {}
impl TypeAbi for BLSKey {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        <[u8; BLSKey::len()]>::type_name()
    }

    fn type_name_rust() -> TypeName {
        "BLSKey".into()
    }
}

impl TypeAbiFrom<Self> for BLSSignature {}
impl TypeAbiFrom<[u8; BLSSignature::len()]> for BLSSignature {}
impl TypeAbiFrom<BLSSignature> for [u8; BLSSignature::len()] {}
impl TypeAbi for BLSSignature {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        <[u8; BLSSignature::len()]>::type_name()
    }

    fn type_name_rust() -> TypeName {
        "BLSSignature".into()
    }
}

impl TypeAbiFrom<Self> for EsdtTokenType {}
impl TypeAbiFrom<&Self> for EsdtTokenType {}

// implementation originally generated via #[type_abi] attribute
impl TypeAbi for EsdtTokenType {
    type Unmanaged = Self;
    fn type_name() -> TypeName {
        "EsdtTokenType".into()
    }

    #[allow(clippy::vec_init_then_push)]
    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        let type_names = Self::type_names();
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

impl TypeAbiFrom<Self> for EsdtLocalRole {}
impl TypeAbiFrom<&Self> for EsdtLocalRole {}

// implementation originally generated via #[type_abi] attribute
impl TypeAbi for EsdtLocalRole {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "EsdtLocalRole".into()
    }

    #[allow(clippy::vec_init_then_push)]
    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        let type_names = Self::type_names();
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
