use elrond_wasm::abi::ContractAbi;

pub fn validate_abi(abi: &ContractAbi) -> Result<(), &'static str> {
    if abi.constructor.is_none() {
        return Err("Missing constructor. Add a method annotated with `#[init]`.");
    }
    Ok(())
}
