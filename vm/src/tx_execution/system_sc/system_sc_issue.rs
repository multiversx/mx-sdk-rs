use crate::{
    num_bigint::BigUint,
    tx_mock::{TxContext, TxResult},
};
use multiversx_sc::codec::TopDecode;

pub fn issue(tx_context: TxContext) -> (TxContext, TxResult) {
    let tx_input = tx_context.input_ref();
    let tx_cache = tx_context.blockchain_cache();
    let tx_result: TxResult;

    if tx_input.args.len() < 4 {
        tx_result = TxResult::from_vm_error("not enough arguments");
        return (tx_context, tx_result);
    }
    let _name = tx_input.args[0].clone();
    let _ticker = tx_input.args[1].clone();
    let _total_supply = BigUint::from_bytes_be(tx_input.args[2].clone().as_ref());
    let _decimals = u32::top_decode(tx_input.args[3].clone().as_ref()).unwrap();

    let token_identifier = b"TESTCOLL1-4096af".to_vec();

    tx_cache.with_account_mut(&tx_input.from, |account| {
        account.esdt.issue_token(&token_identifier);
    });

    tx_result = TxResult {
        result_values: vec![token_identifier],
        ..Default::default()
    };

    (tx_context, tx_result)
}

pub fn issue_semi_fungible(tx_context: TxContext) -> (TxContext, TxResult) {
    issue_non_fungible(tx_context)
}

pub fn issue_non_fungible(tx_context: TxContext) -> (TxContext, TxResult) {
    let tx_input = tx_context.input_ref();
    let tx_cache = tx_context.blockchain_cache();
    let tx_result: TxResult;

    if tx_input.args.len() < 2 {
        tx_result = TxResult::from_vm_error("not enough arguments");
        return (tx_context, tx_result);
    }
    let _name = tx_input.args[0].clone();
    let _ticker = tx_input.args[1].clone();

    let token_identifier = b"TESTCOLL1-4096bf".to_vec();

    tx_cache.with_account_mut(&tx_input.from, |account| {
        account.esdt.issue_token(&token_identifier);
    });

    tx_result = TxResult {
        result_values: vec![token_identifier],
        ..Default::default()
    };

    (tx_context, tx_result)
}

// todo#1: generate token identifier randomly
// - - - - - - - - - - - - - - - - - - - - - - - - -
//            code from mx-chain-go
// - - - - - - - - - - - - - - - - - - - - - - - - -

// newRandomBase := append(caller, e.eei.BlockChainHook().CurrentRandomSeed()...)
// newRandom := e.hasher.Compute(string(newRandomBase))
// newRandomForTicker := newRandom[:tickerRandomSequenceLength]

// tickerPrefix := append(ticker, []byte(tickerSeparator)...)
// newRandomAsBigInt := big.NewInt(0).SetBytes(newRandomForTicker)

// one := big.NewInt(1)
// for i := 0; i < numOfRetriesForIdentifier; i++ {
// 	encoded := fmt.Sprintf("%06x", newRandomAsBigInt)
// 	newIdentifier := append(tickerPrefix, encoded...)
// 	buff := e.eei.GetStorage(newIdentifier)
// 	if len(buff) == 0 {
// 		return newIdentifier, nil
// 	}
// 	newRandomAsBigInt.Add(newRandomAsBigInt, one)
// }

// return nil, vm.ErrCouldNotCreateNewTokenIdentifier
