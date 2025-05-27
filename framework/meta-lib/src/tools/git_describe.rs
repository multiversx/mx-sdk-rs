use std::process::{Command, Output};

pub fn git_describe() -> String {
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
