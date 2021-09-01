#![no_std]

use elrond_wasm::HexCallDataSerializer;

mod call_async;
mod call_sync;
mod call_transf_exec;
mod contract_change_owner;
mod contract_deploy;
mod contract_update;
mod esdt;
mod nft;
mod roles;
mod sft;
mod storage;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopDecode, TopEncode)]
pub enum DeployArguments {
    None,
    DeployContract {
        contract_code: BoxedBytes,
    },
    AsyncCall {
        dest_address: Address,
        function: BoxedBytes,
    },
    SyncCall {
        dest_address: Address,
        function: BoxedBytes,
    },
}

/// Test contract for investigating contract calls.
#[elrond_wasm::contract]
pub trait Forwarder:
    call_sync::ForwarderSyncCallModule
    + call_async::ForwarderAsyncCallModule
    + call_transf_exec::ForwarderTransferExecuteModule
    + contract_change_owner::ChangeOwnerModule
    + contract_deploy::DeployContractModule
    + contract_update::UpgradeContractModule
    + esdt::ForwarderEsdtModule
    + sft::ForwarderSftModule
    + nft::ForwarderNftModule
    + roles::ForwarderRolesModule
    + storage::ForwarderStorageModule
{
    #[init]
    fn init(
        &self,
        #[var_args] opt_deploy_arg: OptionalArg<DeployArguments>,
        #[var_args] call_arguments: VarArgs<BoxedBytes>,
    ) -> SCResult<()> {
        if let OptionalArg::Some(deploy_arg) = opt_deploy_arg {
            match deploy_arg {
                DeployArguments::None => (),
                DeployArguments::DeployContract { contract_code } => {
                    let opt_address = self.send().deploy_contract(
                        self.blockchain().get_gas_left() / 2,
                        &self.types().big_uint_zero(),
                        &contract_code,
                        CodeMetadata::DEFAULT,
                        &call_arguments.as_slice().into(),
                    );

                    let _ = opt_address.ok_or("Deploy failed")?;
                },
                DeployArguments::AsyncCall {
                    dest_address,
                    function,
                } => {
                    let mut serializer = HexCallDataSerializer::new(function.as_slice());
                    for arg in call_arguments.into_vec() {
                        serializer.push_argument_bytes(arg.as_slice());
                    }

                    self.send().async_call_raw(
                        &dest_address,
                        &self.types().big_uint_zero(),
                        serializer.as_slice(),
                    );
                },
                DeployArguments::SyncCall {
                    dest_address,
                    function,
                } => {
                    let _ = self.send().execute_on_dest_context_raw(
                        self.blockchain().get_gas_left() / 2,
                        &dest_address,
                        &self.types().big_uint_zero(),
                        function.as_slice(),
                        &call_arguments.as_slice().into(),
                    );
                },
            };
        }

        Ok(())
    }

    #[endpoint]
    fn send_egld(
        &self,
        to: &Address,
        amount: &BigUint,
        #[var_args] opt_data: OptionalArg<BoxedBytes>,
    ) {
        let data = match &opt_data {
            OptionalArg::Some(data) => data.as_slice(),
            OptionalArg::None => &[],
        };
        self.send().direct_egld(to, amount, data);
    }
}
