use elrond_wasm::abi::ContractAbi;


pub fn update_git_version(abi: &mut ContractAbi) {
    abi.build_info.contract_crate.git_version = git_describe();
}

fn git_describe() -> String {
    // if !self.build_args.abi_git_version {
    //     return String::new();
    // }

    Command::new("git")
        .args(["describe"])
        .output()
        .map(git_describe_process_output)
        .unwrap_or_default()
}

fn git_describe_process_output(output: Output) -> String {
    if output.status.success() {
        let mut result = String::from_utf8(output.stdout).unwrap_or_default();
        if result.ends_with('\n') {
            // for some reason we get a trailing newline
            let _ = result.pop();
        }
        result
    } else {
        String::new()
    }
}