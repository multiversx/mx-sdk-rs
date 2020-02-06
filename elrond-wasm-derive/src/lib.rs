

#![allow(dead_code)]
#![allow(stable_features)]

// ensure we don't run out of macro stack
#![recursion_limit="1024"]

extern crate proc_macro;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

mod contract;
mod gen;
mod gen_arg;
mod gen_event;
mod gen_finish;
mod gen_payable;

fn wasm32_mode() -> bool {
  // this checks if we set --release or not in the command line
  // we should always set --release when building sc wasm and never when running the debugger, so this works
  let debug_mode = cfg!(debug_assertions);
  !debug_mode

  // this is supposed to check whether or not the target starts with "wasm32-...
  // for some reason this no longer works, TODO: investigate
  //cfg!(target_arch = "wasm32")

  // when debugging the macro output, the above methods don't seem to work
  // so just temporarily hardcode while bugfixing
  //true
}

#[proc_macro_attribute]
pub fn contract(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

  contract::process_contract(args, input)

}
