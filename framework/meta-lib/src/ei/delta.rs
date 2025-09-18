/// Delta between 1.0 and 1.1.
///
/// Names of VM hooks added around November-December 2021.
pub const EI_1_1_ADDED_NAMES: &[&str] = &[
    "mBufferSetByteSlice",
    "managedSha256",
    "managedKeccak256",
    "mBufferStorageLoadFromAddress",
    "validateTokenIdentifier",
    "getESDTLocalRoles",
    "cleanReturnData",
    "deleteFromReturnData",
];

/// Delta between 1.1 and 1.2.
///
/// Names of VM hooks added around May 2022.
pub const EI_1_2_ADDED_NAMES: &[&str] = &[
    // debugging/display utilities
    "bigIntToString",
    "managedBufferToHex",
    // more managed crypto functions
    "managedRipemd160",
    "managedVerifyBLS",
    "managedVerifyEd25519",
    "managedVerifySecp256k1",
    "managedVerifyCustomSecp256k1",
    "managedEncodeSecp256k1DerSignature",
    "managedScalarBaseMultEC",
    "managedScalarMultEC",
    "managedMarshalEC",
    "managedUnmarshalEC",
    "managedMarshalCompressedEC",
    "managedUnmarshalCompressedEC",
    "managedGenerateKeyEC",
    "managedCreateEC",
    // big floats
    "mBufferToBigFloat",
    "mBufferFromBigFloat",
    "bigFloatNewFromParts",
    "bigFloatNewFromFrac",
    "bigFloatNewFromSci",
    "bigFloatAdd",
    "bigFloatSub",
    "bigFloatMul",
    "bigFloatDiv",
    "bigFloatNeg",
    "bigFloatClone",
    "bigFloatCmp",
    "bigFloatAbs",
    "bigFloatSign",
    "bigFloatSqrt",
    "bigFloatPow",
    "bigFloatFloor",
    "bigFloatCeil",
    "bigFloatTruncate",
    "bigFloatSetInt64",
    "bigFloatIsInt",
    "bigFloatSetBigInt",
    "bigFloatGetConstPi",
    "bigFloatGetConstE",
    // more ESDT utilities
    "managedIsESDTFrozen",
    "managedIsESDTPaused",
    "managedIsESDTLimitedTransfer",
];

/// Delta between 1.2 and 1.3.
///
/// Names of VM hooks released with VM 1.5.
pub const EI_1_3_ADDED_NAMES: &[&str] = &[
    "managedCreateAsyncCall",
    "managedGetBackTransfers",
    "managedGetCallbackClosure",
    "managedGetCodeMetadata",
    "managedIsBuiltinFunction",
];

/// Delta between 1.3 and 1.4.
///
/// Names of VM hooks added in the Spica release.
pub const EI_1_4_ADDED_NAMES: &[&str] = &[
    "isReservedFunctionName",
    "managedGetOriginalCallerAddr",
    "managedGetRelayerAddr",
    "managedMultiTransferESDTNFTExecuteByUser",
    "managedVerifySecp256r1",
    "managedVerifyBLSSignatureShare",
    "managedVerifyBLSAggregatedSignature",
];

/// Delta between 1.4 and 1.5.
///
/// Names of VM hooks added in the Barnard release.
pub const EI_1_5_ADDED_NAMES: &[&str] = &[
    "getBlockTimestampMs",
    "getPrevBlockTimestampMs",
    "getBlockRoundTimeMs",
    "epochStartBlockTimestampMs",
    "epochStartBlockNonce",
    "epochStartBlockRound",
    "managedGetAllTransfersCallValue",
    "managedGetESDTTokenType",
    "managedExecuteOnDestContextWithErrorReturn",
    "managedMultiTransferESDTNFTExecuteWithReturn",
    "managedGetCodeHash",
    "mBufferToSmallIntUnsigned",
    "mBufferToSmallIntSigned",
    "mBufferFromSmallIntUnsigned",
    "mBufferFromSmallIntSigned",
];
