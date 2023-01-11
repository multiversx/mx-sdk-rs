use multiversx_sc::abi::ContractAbi;

fn validate_abi_constructor(abi: &ContractAbi) -> Result<(), &'static str> {
    match abi.constructors.len() {
        0 => Err("Missing constructor. Add a method annotated with `#[init]`."),
        1 => Ok(()),
        _ => Err("More than one contrctructor present. Exactly one method annotated with `#[init]` is required."),
    }
}

pub fn validate_abi(abi: &ContractAbi) -> Result<(), &'static str> {
    validate_abi_constructor(abi)?;
    Ok(())
}
