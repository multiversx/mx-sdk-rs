use multiversx_sc::abi::{ContractAbi, EndpointAbi};

pub fn validate_abi(abi: &ContractAbi) -> Result<(), String> {
    check_single_constructor(abi)?;
    validate_contract_var_args(abi)?;
    Ok(())
}

fn check_single_constructor(abi: &ContractAbi) -> Result<(), String> {
    match abi.constructors.len() {
        0 => Err("Missing constructor. Add a method annotated with `#[init]`.".to_string()),
        1 => Ok(()),
        _ => Err("More than one contrctructor present. Exactly one method annotated with `#[init]` is required.".to_string()),
    }
}

fn validate_contract_var_args(abi: &ContractAbi) -> Result<(), String> {
    for endpoint_abi in abi.iter_all_functions() {
        validate_endpoint_var_args(endpoint_abi)?;
    }
    Ok(())
}

fn validate_endpoint_var_args(endpoint_abi: &EndpointAbi) -> Result<(), String> {
    let mut var_args_encountered = false;
    for arg in &endpoint_abi.inputs {
        if arg.multi_arg {
            var_args_encountered = true;
        } else {
            if var_args_encountered {
                return Err(format!(
                    "Found regular arguments after var-args in method {}. This is not allowed, because it makes it impossible to parse the arguments.",
                    &endpoint_abi.rust_method_name));
            }
        }
    }

    Ok(())
}
