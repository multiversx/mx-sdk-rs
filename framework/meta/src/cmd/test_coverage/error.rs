use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum TestCoverageError {
    MissingDependency(String),
    FsError(String),
    Cargo(String),
    LlvmProfdata(String),
    LlvmCov(String),
}

impl Error for TestCoverageError {}

impl Display for TestCoverageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            TestCoverageError::MissingDependency(dep) => format!(
                "Missing dependency {dep}. Make sure you have {dep} installed on your system and it's available in PATH."
            ),

            TestCoverageError::FsError(msg) => format!("FS operation failed: {msg}"),
            TestCoverageError::Cargo(msg) => format!("Failed to run cargo: {msg}"),
            TestCoverageError::LlvmProfdata(msg) => format!("llvm-profdata error: {msg}"),
            TestCoverageError::LlvmCov(msg) => format!("llvm-cov error: {msg}"),
        };
        write!(f, "TestCoverageRenderError: {}", message)
    }
}
