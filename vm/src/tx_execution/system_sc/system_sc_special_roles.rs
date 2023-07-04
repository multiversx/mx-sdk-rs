use crate::{tx_mock::{TxContext, TxResult}, types::VMAddress};

pub fn set_special_role(tx_context: TxContext) -> (TxContext, TxResult) {
    let tx_input = tx_context.input_ref();
    let tx_cache = tx_context.blockchain_cache();
    let mut tx_result = TxResult::empty();

    if tx_input.args.len() < 3 {
        tx_result = TxResult::from_vm_error("setSpecialRole too few arguments");
        return (tx_context, tx_result);
    }
    let token_identifier = tx_input.args[0].clone();
    let address = VMAddress::from_slice(tx_input.args[1].as_slice());
    let role = tx_input.args[2].clone();

    tx_cache.with_account_mut(&address, |account| {
        account.esdt.set_special_role(&token_identifier, &role);
    });

    (tx_context, tx_result)
}
