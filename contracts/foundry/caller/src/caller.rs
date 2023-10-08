#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait CallerContract {
    #[init]
    fn init(&self) {}

    #[endpoint]
    #[payable("EGLD")]
    fn call_other(
        &self,
        dest: ManagedAddress,
        func: ManagedBuffer,
        value: i64,
        to_send: BigUint,
    ) -> ManagedBuffer {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(value);

        let result =
            self.send_raw()
                .direct_egld_execute(&dest, &to_send, 5000000, &func, &arg_buffer);

        match result {
            Result::Err(_) => ManagedBuffer::from("failed"),
            Result::Ok(_) => ManagedBuffer::from("done"),
        }
    }

    #[endpoint]
    #[payable("EGLD")]
    fn call_other_exec_on_dest_ctx(
        &self,
        dest: ManagedAddress,
        func: ManagedBuffer,
        value: i64,
        to_send: BigUint,
    ) -> ManagedBuffer {
        let mut arg_buffer = ManagedArgBuffer::new();
        arg_buffer.push_arg(value);

        let result = self.send_raw().execute_on_dest_context_raw(
            5000000,
            &dest,
            &to_send,
            &func,
            &arg_buffer,
        );

        require!(
            result.len() == 1,
            "ExecuteOnDestContext result data is empty"
        );

        ManagedBuffer::from("done")
    }
}
