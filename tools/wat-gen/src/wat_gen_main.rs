mod wat_gen_single_import;

fn main() {
    for &hook_name in EI_1_5_ADDED_NAMES {
        wat_gen_single_import::write_sc_files(hook_name);
    }
}

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
