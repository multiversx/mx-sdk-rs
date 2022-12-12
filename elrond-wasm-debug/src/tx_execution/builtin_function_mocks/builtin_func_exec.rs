use std::rc::Rc;

use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    builtin_func_map::BuiltinFunctionMap,
    builtin_func_role_check_wrapper::BuiltinFunctionRoleCheckWrapper,
    builtin_func_trait::BuiltinFunction,
    esdt_nft::{
        ESDTLocalBurn, ESDTLocalMint, ESDTNftAddQuantity, ESDTNftAddUri, ESDTNftBurn,
        ESDTNftCreate, ESDTNftUpdateAttributes,
    },
    general::{ChangeOwner, ClaimDeveloperRewards, SetUsername, UpgradeContract},
    transfer::{ESDTMultiTransfer, ESDTNftTransfer, ESDTTransfer},
};

const ESDT_ROLE_LOCAL_MINT: &str = "ESDTRoleLocalMint";
const ESDT_ROLE_LOCAL_BURN: &str = "ESDTRoleLocalBurn";
const ESDT_ROLE_NFT_CREATE: &str = "ESDTRoleNFTCreate";
const ESDT_ROLE_NFT_ADD_QUANTITY: &str = "ESDTRoleNFTAddQuantity";
const ESDT_ROLE_NFT_BURN: &str = "ESDTRoleNFTBurn";
const ESDT_ROLE_NFT_ADD_URI: &str = "ESDTRoleNFTAddURI";
const ESDT_ROLE_NFT_UPDATE_ATTRIBUTES: &str = "ESDTRoleNFTUpdateAttributes";

fn builtin_function_impls() -> Vec<Box<dyn BuiltinFunction>> {
    vec![
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            ESDT_ROLE_LOCAL_MINT,
            Box::new(ESDTLocalMint),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            ESDT_ROLE_LOCAL_BURN,
            Box::new(ESDTLocalBurn),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            ESDT_ROLE_NFT_CREATE,
            Box::new(ESDTNftCreate),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            ESDT_ROLE_NFT_ADD_QUANTITY,
            Box::new(ESDTNftAddQuantity),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            ESDT_ROLE_NFT_BURN,
            Box::new(ESDTNftBurn),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            ESDT_ROLE_NFT_ADD_URI,
            Box::new(ESDTNftAddUri),
        )),
        Box::new(BuiltinFunctionRoleCheckWrapper::new(
            ESDT_ROLE_NFT_UPDATE_ATTRIBUTES,
            Box::new(ESDTNftUpdateAttributes),
        )),
        Box::new(ESDTMultiTransfer),
        Box::new(ESDTNftTransfer),
        Box::new(ESDTTransfer),
        Box::new(ChangeOwner),
        Box::new(ClaimDeveloperRewards),
        Box::new(SetUsername),
        Box::new(UpgradeContract),
    ]
}

pub fn init_builtin_functions() -> BuiltinFunctionMap {
    BuiltinFunctionMap::init(builtin_function_impls())
}

pub fn execute_builtin_function_or_default(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    let builtin_functions = Rc::clone(&tx_cache.blockchain_ref().builtin_functions);
    if let Some(builtin_func) = builtin_functions.get(&tx_input.func_name) {
        builtin_func.execute(tx_input, tx_cache)
    } else {
        default_execution(tx_input, tx_cache)
    }
}
