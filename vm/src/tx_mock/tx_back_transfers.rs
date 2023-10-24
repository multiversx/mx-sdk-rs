use num_bigint::BigUint;
use num_traits::Zero;

use super::{TxResult, TxTokenTransfer};

#[derive(Default)]
pub struct BackTransfers {
    pub call_value: BigUint,
    pub esdt_transfers: Vec<TxTokenTransfer>,
}

impl BackTransfers {
    pub fn empty() -> Self {
        BackTransfers::default()
    }

    pub fn append_from_result(&mut self, result: &TxResult) {
        if result.result_status != 0 {
            return;
        }

        for call in &result.all_calls {
            if call.call_value > BigUint::zero() {
                self.call_value += call.call_value;
            }
            let tx_data = 
        }
    }
}

// func (host *vmHost) addNewBackTransfersFromVMOutput(vmOutput *vmcommon.VMOutput, parent, child []byte) {
// 	if vmOutput == nil || vmOutput.ReturnCode != vmcommon.Ok {
// 		return
// 	}
// 	callerOutAcc, ok := vmOutput.OutputAccounts[string(parent)]
// 	if !ok {
// 		return
// 	}

// 	for _, transfer := range callerOutAcc.OutputTransfers {
// 		if !bytes.Equal(transfer.SenderAddress, child) {
// 			continue
// 		}
// 		if transfer.CallType == vm.AsynchronousCallBack {
// 			continue
// 		}

// 		if transfer.Value.Cmp(vmhost.Zero) > 0 {
// 			if len(transfer.Data) == 0 {
// 				host.managedTypesContext.AddValueOnlyBackTransfer(transfer.Value)
// 			}
// 			continue
// 		}

// 		esdtTransfers, isWithoutExec := host.isESDTTransferWithoutExecution(transfer.Data, parent, child)
// 		if !isWithoutExec {
// 			continue
// 		}

// 		host.managedTypesContext.AddBackTransfers(esdtTransfers.ESDTTransfers)
// 	}
// }
