multiversx_sc::derive_imports!();

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpInConstructor {
    pub something: (),
}

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested01;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested02 {
    pub something: [u8; 0],
}

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested03();

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested04;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested05;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested06;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested07;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested08;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNested09;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInSingleValueMapper;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInVec;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInArrayVec;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInArray;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInBox;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInBoxedSlice;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInRef;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInSlice;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct OnlyShowsUpAsNestedInOption;
