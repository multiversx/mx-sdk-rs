use multiversx_chain_core::types::{EsdtLocalRole, EsdtLocalRoleFlags};

/// Every non-None role paired with its expected numeric ID, canonical name,
/// and expected flag bit. This is the single source of truth for all
/// round-trip checks below.
#[rustfmt::skip]
const ROLE_TABLE: &[(EsdtLocalRole, u16, &str, EsdtLocalRoleFlags)] = &[
    (EsdtLocalRole::Mint,                1,  "ESDTRoleLocalMint",          EsdtLocalRoleFlags::MINT),
    (EsdtLocalRole::Burn,                2,  "ESDTRoleLocalBurn",          EsdtLocalRoleFlags::BURN),
    (EsdtLocalRole::NftCreate,           3,  "ESDTRoleNFTCreate",          EsdtLocalRoleFlags::NFT_CREATE),
    (EsdtLocalRole::NftAddQuantity,      4,  "ESDTRoleNFTAddQuantity",     EsdtLocalRoleFlags::NFT_ADD_QUANTITY),
    (EsdtLocalRole::NftBurn,             5,  "ESDTRoleNFTBurn",            EsdtLocalRoleFlags::NFT_BURN),
    (EsdtLocalRole::NftUpdateAttributes, 6,  "ESDTRoleNFTUpdateAttributes",EsdtLocalRoleFlags::NFT_UPDATE_ATTRIBUTES),
    (EsdtLocalRole::NftAddUri,           7,  "ESDTRoleNFTAddURI",          EsdtLocalRoleFlags::NFT_ADD_URI),
    (EsdtLocalRole::NftRecreate,         8,  "ESDTRoleNFTRecreate",        EsdtLocalRoleFlags::NFT_RECREATE),
    (EsdtLocalRole::ModifyCreator,       9,  "ESDTRoleModifyCreator",      EsdtLocalRoleFlags::MODIFY_CREATOR),
    (EsdtLocalRole::ModifyRoyalties,     10, "ESDTRoleModifyRoyalties",    EsdtLocalRoleFlags::MODIFY_ROYALTIES),
    (EsdtLocalRole::SetNewUri,           11, "ESDTRoleSetNewURI",          EsdtLocalRoleFlags::SET_NEW_URI),
    (EsdtLocalRole::Transfer,            12, "ESDTTransferRole",           EsdtLocalRoleFlags::TRANSFER),
];

#[test]
fn test_none_role() {
    assert_eq!(EsdtLocalRole::None.as_u16(), 0);
    assert_eq!(EsdtLocalRole::None.name(), "");
    assert_eq!(EsdtLocalRole::None.as_role_name(), b"");
    assert_eq!(EsdtLocalRole::None.to_flag(), EsdtLocalRoleFlags::NONE);
}

#[test]
fn test_as_u16() {
    for (role, id, _, _) in ROLE_TABLE {
        assert_eq!(role.as_u16(), *id, "{role:?}.as_u16()");
    }
}

#[test]
fn test_from_u16_round_trip() {
    for (role, id, _, _) in ROLE_TABLE {
        assert_eq!(EsdtLocalRole::from(*id), *role, "from({})", id);
    }
}

#[test]
fn test_from_u16_unknown_returns_none() {
    assert_eq!(EsdtLocalRole::from(0u16), EsdtLocalRole::None);
    assert_eq!(EsdtLocalRole::from(13u16), EsdtLocalRole::None);
    assert_eq!(EsdtLocalRole::from(u16::MAX), EsdtLocalRole::None);
}

/// `as_u16` and `From<u16>` must be exact inverses for every defined role.
#[test]
fn test_u16_round_trip_both_directions() {
    for (role, _, _, _) in ROLE_TABLE {
        let id = role.as_u16();
        assert_eq!(&EsdtLocalRole::from(id), role, "round-trip for {role:?}");
    }
}

#[test]
fn test_name_and_as_role_name() {
    for (role, _, name, _) in ROLE_TABLE {
        assert_eq!(role.name(), *name, "{role:?}.name()");
        assert_eq!(
            role.as_role_name(),
            name.as_bytes(),
            "{role:?}.as_role_name()"
        );
    }
}

#[test]
fn test_from_byte_slice_round_trip() {
    for (role, _, name, _) in ROLE_TABLE {
        let decoded = EsdtLocalRole::from(name.as_bytes());
        assert_eq!(decoded, *role, "from(b\"{name}\")");
    }
}

#[test]
fn test_from_byte_slice_unknown_returns_none() {
    assert_eq!(EsdtLocalRole::from(b"".as_ref()), EsdtLocalRole::None);
    assert_eq!(
        EsdtLocalRole::from(b"unknown".as_ref()),
        EsdtLocalRole::None
    );
    // Ensure a near-match (extra char) does not decode as a valid role.
    assert_eq!(
        EsdtLocalRole::from(b"ESDTRoleLocalMintX".as_ref()),
        EsdtLocalRole::None,
    );
}

/// `name()` and `From<&[u8]>` must be exact inverses for every defined role.
#[test]
fn test_byte_slice_round_trip_both_directions() {
    for (role, _, _, _) in ROLE_TABLE {
        let name = role.name();
        assert_eq!(
            &EsdtLocalRole::from(name.as_bytes()),
            role,
            "round-trip for {role:?}",
        );
    }
}

#[test]
fn test_to_flag() {
    for (role, _, _, flag) in ROLE_TABLE {
        assert_eq!(role.to_flag(), *flag, "{role:?}.to_flag()");
    }
    assert_eq!(EsdtLocalRole::None.to_flag(), EsdtLocalRoleFlags::NONE);
}

/// `ModifyCreator` (9) and `ModifyRoyalties` (10) are the historically
/// swapped pair — verify both directions explicitly.
#[test]
fn test_modify_creator_and_modify_royalties_not_swapped() {
    assert_eq!(EsdtLocalRole::ModifyCreator.as_u16(), 9);
    assert_eq!(EsdtLocalRole::ModifyRoyalties.as_u16(), 10);
    assert_eq!(EsdtLocalRole::from(9u16), EsdtLocalRole::ModifyCreator);
    assert_eq!(EsdtLocalRole::from(10u16), EsdtLocalRole::ModifyRoyalties);
    assert_eq!(
        EsdtLocalRole::from(b"ESDTRoleModifyCreator".as_ref()),
        EsdtLocalRole::ModifyCreator,
    );
    assert_eq!(
        EsdtLocalRole::from(b"ESDTRoleModifyRoyalties".as_ref()),
        EsdtLocalRole::ModifyRoyalties,
    );
    assert_eq!(
        EsdtLocalRole::ModifyCreator.to_flag(),
        EsdtLocalRoleFlags::MODIFY_CREATOR,
    );
    assert_eq!(
        EsdtLocalRole::ModifyRoyalties.to_flag(),
        EsdtLocalRoleFlags::MODIFY_ROYALTIES,
    );
}

#[test]
fn test_iter_all_order_and_completeness() {
    let roles: Vec<EsdtLocalRole> = EsdtLocalRole::iter_all().copied().collect();
    assert_eq!(roles.len(), ROLE_TABLE.len());
    for (idx, (expected_role, _, _, _)) in ROLE_TABLE.iter().enumerate() {
        assert_eq!(roles[idx], *expected_role, "iter_all position {idx}");
    }
}
