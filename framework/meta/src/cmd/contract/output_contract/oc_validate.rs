use multiversx_sc::abi::{ContractAbi, EndpointAbi};

use super::OutputContract;

pub fn validate_output_contract(output_contract: &OutputContract) -> Result<(), String> {
    check_single_constructor(output_contract)?;
    validate_contract_var_args(&output_contract.abi)?;
    Ok(())
}

fn check_single_constructor(output_contract: &OutputContract) -> Result<(), String> {
    match output_contract.abi.constructors.len() {
            0 => Err("Missing constructor. Add a method annotated with `#[init]`.".to_string()),
            1 => Ok(()),
            _ => Err("More than one contrctructor present. Exactly one method annotated with `#[init]` is required.".to_string()),
        }
}

/// Note: promise callbacks not included, since they have `#[call_value]` arguments, that are currently not modelled.
fn validate_contract_var_args(abi: &ContractAbi) -> Result<(), String> {
    for endpoint_abi in abi.constructors.iter().chain(abi.endpoints.iter()) {
        validate_endpoint_var_args(endpoint_abi)?;
    }
    Ok(())
}

fn validate_endpoint_var_args(endpoint_abi: &EndpointAbi) -> Result<(), String> {
    let num_var_args = endpoint_abi
        .inputs
        .iter()
        .filter(|input| input.multi_arg)
        .count();
    if num_var_args > 1usize && !endpoint_abi.allow_multiple_var_args {
        return Err(format!(
        "Multiple var args found in {}. Use #[allow_multiple_var_args] if you want to enable this feature",
        &endpoint_abi.rust_method_name));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use multiversx_sc::abi::{InputAbi, TypeName};

    use super::*;

    #[test]
    fn validate_endpoint_var_args_test() {
        let mut endpoint_def = EndpointAbi::default();
        let var_arg_1 = InputAbi {
            arg_name: "arg_1",
            type_name: TypeName::new(),
            multi_arg: true,
        };
        let var_arg_2 = InputAbi {
            arg_name: "arg_2",
            type_name: TypeName::new(),
            multi_arg: true,
        };
        endpoint_def.inputs.push(var_arg_1);
        endpoint_def.inputs.push(var_arg_2);

        assert!(!endpoint_def.allow_multiple_var_args);
        assert_eq!(Err(format!(
        "Multiple var args found in {}. Use #[allow_multiple_var_args] if you want to enable this feature",
        &endpoint_def.rust_method_name)), validate_endpoint_var_args(&endpoint_def));

        endpoint_def.allow_multiple_var_args = true;
        assert_eq!(Ok(()), validate_endpoint_var_args(&endpoint_def));
    }
}
