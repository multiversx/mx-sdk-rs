//// TYPE ABI ////
impl multiversx_sc::abi::TypeAbi for EsdtLocalRole {
    fn type_name() -> multiversx_sc::abi::TypeName {
        "EsdtLocalRole".into()
    }
    fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(
        accumulator: &mut TDC,
    ) {
        let type_name = Self::type_name();
        if !accumulator.contains_type(&type_name) {
            accumulator.reserve_type_name(type_name.clone());
            let mut variant_descriptions = multiversx_sc::types::heap::Vec::new();
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "None",
                0usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "Mint",
                1usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "Burn",
                2usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "NftCreate",
                3usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "NftAddQuantity",
                4usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "NftBurn",
                5usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "NftAddUri",
                6usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "NftUpdateAttributes",
                7usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "Transfer",
                8usize,
                field_descriptions,
            ));
            accumulator.insert(
                type_name.clone(),
                multiversx_sc::abi::TypeDescription::new(
                    &[],
                    type_name,
                    multiversx_sc::abi::TypeContents::Enum(variant_descriptions),
                ),
            );
        }
    }
}
//// TYPE ABI ////
impl multiversx_sc::abi::TypeAbi for EsdtTokenType {
    fn type_name() -> multiversx_sc::abi::TypeName {
        "EsdtTokenType".into()
    }
    fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(
        accumulator: &mut TDC,
    ) {
        let type_name = Self::type_name();
        if !accumulator.contains_type(&type_name) {
            accumulator.reserve_type_name(type_name.clone());
            let mut variant_descriptions = multiversx_sc::types::heap::Vec::new();
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "Fungible",
                0usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "NonFungible",
                1usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "SemiFungible",
                2usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "Meta",
                3usize,
                field_descriptions,
            ));
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            variant_descriptions.push(multiversx_sc::abi::EnumVariantDescription::new(
                &[],
                "Invalid",
                4usize,
                field_descriptions,
            ));
            accumulator.insert(
                type_name.clone(),
                multiversx_sc::abi::TypeDescription::new(
                    &[],
                    type_name,
                    multiversx_sc::abi::TypeContents::Enum(variant_descriptions),
                ),
            );
        }
    }
}
//// MANAGED VEC ITEM ////
impl multiversx_sc::types::ManagedVecItem for EsdtTokenType {
    const PAYLOAD_SIZE: usize = 1;
    const SKIPS_RESERIALIZATION: bool = true;
    type Ref<'a> = Self;
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        let mut arr: [u8; 1] = [0u8; 1];
        reader(&mut arr[..]);
        match arr[0] {
            0u8 => EsdtTokenType::Fungible,
            1u8 => EsdtTokenType::NonFungible,
            2u8 => EsdtTokenType::SemiFungible,
            3u8 => EsdtTokenType::Meta,
            4u8 => EsdtTokenType::Invalid,
            _ => EsdtTokenType::Fungible,
        }
    }
    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }
    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        let mut arr: [u8; 1] = [0u8; 1];
        arr[0] = match self {
            EsdtTokenType::Fungible => 0u8,
            EsdtTokenType::NonFungible => 1u8,
            EsdtTokenType::SemiFungible => 2u8,
            EsdtTokenType::Meta => 3u8,
            EsdtTokenType::Invalid => 4u8,
        };
        writer(&arr[..])
    }
}
//// MANAGED VEC ITEM ////
impl<M: ManagedTypeApi> multiversx_sc::types::ManagedVecItem for EgldOrEsdtTokenIdentifier<M> {
    const PAYLOAD_SIZE : usize = < ManagedOption < M, TokenIdentifier < M > >
    as multiversx_sc :: types :: ManagedVecItem > :: PAYLOAD_SIZE ;
    const
    SKIPS_RESERIALIZATION : bool = < ManagedOption < M, TokenIdentifier < M >
    > as multiversx_sc :: types :: ManagedVecItem > :: SKIPS_RESERIALIZATION ;
    type Ref<'a> = Self;
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        const SELF_PAYLOAD_SIZE: usize = <EgldOrEsdtTokenIdentifier<
            multiversx_sc::api::uncallable::UncallableApi,
        > as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
        let mut arr: [u8; SELF_PAYLOAD_SIZE] = [0u8; SELF_PAYLOAD_SIZE];
        reader(&mut arr[..]);
        let mut index = 0;
        EgldOrEsdtTokenIdentifier {
            data: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index = index + < ManagedOption < M, TokenIdentifier
                < M > > as multiversx_sc :: types :: ManagedVecItem > ::
                PAYLOAD_SIZE ;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
        }
    }
    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }
    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        const SELF_PAYLOAD_SIZE: usize = <EgldOrEsdtTokenIdentifier<
            multiversx_sc::api::uncallable::UncallableApi,
        > as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
        let mut arr: [u8; SELF_PAYLOAD_SIZE] = [0u8; SELF_PAYLOAD_SIZE];
        let mut index = 0;
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.data, |bytes| {
            let next_index = index + < ManagedOption < M, TokenIdentifier < M
            > > as multiversx_sc :: types :: ManagedVecItem > :: PAYLOAD_SIZE
            ;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        writer(&arr[..])
    }
}
//// TYPE ABI ////
impl<M: ManagedTypeApi> multiversx_sc::abi::TypeAbi for EgldOrEsdtTokenPayment<M> {
    fn type_name() -> multiversx_sc::abi::TypeName {
        "EgldOrEsdtTokenPayment".into()
    }
    fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(
        accumulator: &mut TDC,
    ) {
        let type_name = Self::type_name();
        if !accumulator.contains_type(&type_name) {
            accumulator.reserve_type_name(type_name.clone());
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "token_identifier",
                <EgldOrEsdtTokenIdentifier<M>>::type_name(),
            ));
            <EgldOrEsdtTokenIdentifier<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "token_nonce",
                <u64>::type_name(),
            ));
            <u64>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "amount",
                <BigUint<M>>::type_name(),
            ));
            <BigUint<M>>::provide_type_descriptions(accumulator);
            accumulator.insert(
                type_name.clone(),
                multiversx_sc::abi::TypeDescription::new(
                    &[],
                    type_name,
                    multiversx_sc::abi::TypeContents::Struct(field_descriptions),
                ),
            );
        }
    }
}

//// TYPE ABI ////
impl<M: ManagedTypeApi> multiversx_sc::abi::TypeAbi for EsdtTokenData<M> {
    fn type_name() -> multiversx_sc::abi::TypeName {
        "EsdtTokenData".into()
    }
    fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(
        accumulator: &mut TDC,
    ) {
        let type_name = Self::type_name();
        if !accumulator.contains_type(&type_name) {
            accumulator.reserve_type_name(type_name.clone());
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "token_type",
                <EsdtTokenType>::type_name(),
            ));
            <EsdtTokenType>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "amount",
                <BigUint<M>>::type_name(),
            ));
            <BigUint<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "frozen",
                <bool>::type_name(),
            ));
            <bool>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "hash",
                <ManagedBuffer<M>>::type_name(),
            ));
            <ManagedBuffer<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "name",
                <ManagedBuffer<M>>::type_name(),
            ));
            <ManagedBuffer<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "attributes",
                <ManagedBuffer<M>>::type_name(),
            ));
            <ManagedBuffer<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "creator",
                <ManagedAddress<M>>::type_name(),
            ));
            <ManagedAddress<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "royalties",
                <BigUint<M>>::type_name(),
            ));
            <BigUint<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "uris",
                <ManagedVec<M, ManagedBuffer<M>>>::type_name(),
            ));
            <ManagedVec<M, ManagedBuffer<M>>>::provide_type_descriptions(accumulator);
            accumulator.insert(
                type_name.clone(),
                multiversx_sc::abi::TypeDescription::new(
                    &[],
                    type_name,
                    multiversx_sc::abi::TypeContents::Struct(field_descriptions),
                ),
            );
        }
    }
}
//// MANAGED VEC ITEM ////
impl<M: ManagedTypeApi> multiversx_sc::types::ManagedVecItem for EsdtTokenData<M> {
    const PAYLOAD_SIZE : usize = < EsdtTokenType as multiversx_sc :: types ::
    ManagedVecItem > :: PAYLOAD_SIZE + < BigUint < M > as multiversx_sc ::
    types :: ManagedVecItem > :: PAYLOAD_SIZE + < bool as multiversx_sc ::
    types :: ManagedVecItem > :: PAYLOAD_SIZE + < ManagedBuffer < M > as
    multiversx_sc :: types :: ManagedVecItem > :: PAYLOAD_SIZE + <
    ManagedBuffer < M > as multiversx_sc :: types :: ManagedVecItem > ::
    PAYLOAD_SIZE + < ManagedBuffer < M > as multiversx_sc :: types ::
    ManagedVecItem > :: PAYLOAD_SIZE + < ManagedAddress < M > as multiversx_sc
    :: types :: ManagedVecItem > :: PAYLOAD_SIZE + < BigUint < M > as
    multiversx_sc :: types :: ManagedVecItem > :: PAYLOAD_SIZE + < ManagedVec
    < M, ManagedBuffer < M > > as multiversx_sc :: types :: ManagedVecItem >
    :: PAYLOAD_SIZE ;
    const SKIPS_RESERIALIZATION : bool = < EsdtTokenType as
    multiversx_sc :: types :: ManagedVecItem > :: SKIPS_RESERIALIZATION && <
    BigUint < M > as multiversx_sc :: types :: ManagedVecItem > ::
    SKIPS_RESERIALIZATION && < bool as multiversx_sc :: types ::
    ManagedVecItem > :: SKIPS_RESERIALIZATION && < ManagedBuffer < M > as
    multiversx_sc :: types :: ManagedVecItem > :: SKIPS_RESERIALIZATION && <
    ManagedBuffer < M > as multiversx_sc :: types :: ManagedVecItem > ::
    SKIPS_RESERIALIZATION && < ManagedBuffer < M > as multiversx_sc :: types
    :: ManagedVecItem > :: SKIPS_RESERIALIZATION && < ManagedAddress < M > as
    multiversx_sc :: types :: ManagedVecItem > :: SKIPS_RESERIALIZATION && <
    BigUint < M > as multiversx_sc :: types :: ManagedVecItem > ::
    SKIPS_RESERIALIZATION && < ManagedVec < M, ManagedBuffer < M > > as
    multiversx_sc :: types :: ManagedVecItem > :: SKIPS_RESERIALIZATION ;
    type Ref<'a> = Self;
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(mut reader: Reader) -> Self {
        const SELF_PAYLOAD_SIZE: usize = <EsdtTokenData<
            multiversx_sc::api::uncallable::UncallableApi,
        > as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
        let mut arr: [u8; SELF_PAYLOAD_SIZE] = [0u8; SELF_PAYLOAD_SIZE];
        reader(&mut arr[..]);
        let mut index = 0;
        EsdtTokenData {
            token_type: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index =
                    index + <EsdtTokenType as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
            amount: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index =
                    index + <BigUint<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
            frozen: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index =
                    index + <bool as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
            hash: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index = index
                    + <ManagedBuffer<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
            name: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index = index
                    + <ManagedBuffer<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
            attributes: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index = index
                    + <ManagedBuffer<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
            creator: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index = index
                    + <ManagedAddress<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
            royalties: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index =
                    index + <BigUint<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
            uris: multiversx_sc::types::ManagedVecItem::from_byte_reader(|bytes| {
                let next_index = index + < ManagedVec < M, ManagedBuffer < M >
                > as multiversx_sc :: types :: ManagedVecItem > ::
                PAYLOAD_SIZE ;
                bytes.copy_from_slice(&arr[index..next_index]);
                index = next_index;
            }),
        }
    }
    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }
    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, mut writer: Writer) -> R {
        const SELF_PAYLOAD_SIZE: usize = <EsdtTokenData<
            multiversx_sc::api::uncallable::UncallableApi,
        > as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
        let mut arr: [u8; SELF_PAYLOAD_SIZE] = [0u8; SELF_PAYLOAD_SIZE];
        let mut index = 0;
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.token_type, |bytes| {
            let next_index =
                index + <EsdtTokenType as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.amount, |bytes| {
            let next_index =
                index + <BigUint<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.frozen, |bytes| {
            let next_index = index + <bool as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.hash, |bytes| {
            let next_index =
                index + <ManagedBuffer<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.name, |bytes| {
            let next_index =
                index + <ManagedBuffer<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.attributes, |bytes| {
            let next_index =
                index + <ManagedBuffer<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.creator, |bytes| {
            let next_index =
                index + <ManagedAddress<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.royalties, |bytes| {
            let next_index =
                index + <BigUint<M> as multiversx_sc::types::ManagedVecItem>::PAYLOAD_SIZE;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        multiversx_sc::types::ManagedVecItem::to_byte_writer(&self.uris, |bytes| {
            let next_index = index + < ManagedVec < M, ManagedBuffer < M > >
            as multiversx_sc :: types :: ManagedVecItem > :: PAYLOAD_SIZE ;
            arr[index..next_index].copy_from_slice(bytes);
            index = next_index;
        });
        writer(&arr[..])
    }
}
//// TYPE ABI ////
impl<M: ManagedTypeApi> multiversx_sc::abi::TypeAbi for EsdtTokenPayment<M> {
    fn type_name() -> multiversx_sc::abi::TypeName {
        "EsdtTokenPayment".into()
    }
    fn provide_type_descriptions<TDC: multiversx_sc::abi::TypeDescriptionContainer>(
        accumulator: &mut TDC,
    ) {
        let type_name = Self::type_name();
        if !accumulator.contains_type(&type_name) {
            accumulator.reserve_type_name(type_name.clone());
            let mut field_descriptions = multiversx_sc::types::heap::Vec::new();
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "token_identifier",
                <TokenIdentifier<M>>::type_name(),
            ));
            <TokenIdentifier<M>>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "token_nonce",
                <u64>::type_name(),
            ));
            <u64>::provide_type_descriptions(accumulator);
            field_descriptions.push(multiversx_sc::abi::StructFieldDescription::new(
                &[],
                "amount",
                <BigUint<M>>::type_name(),
            ));
            <BigUint<M>>::provide_type_descriptions(accumulator);
            accumulator.insert(
                type_name.clone(),
                multiversx_sc::abi::TypeDescription::new(
                    &[],
                    type_name,
                    multiversx_sc::abi::TypeContents::Struct(field_descriptions),
                ),
            );
        }
    }
}
