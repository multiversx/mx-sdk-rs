use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum TestCoverageRenderError {
    InvalidInputPath(String),
    InvalidOutputPath(String),
    Cargo(String),
    InvalidLlvmCovInput,
}

impl Error for TestCoverageRenderError {}

impl Display for TestCoverageRenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            TestCoverageRenderError::InvalidInputPath(msg) => format!("Invalid input path: {msg}"),
            TestCoverageRenderError::InvalidOutputPath(msg) => {
                format!("Invalid output path: {msg}")
            },
            TestCoverageRenderError::Cargo(msg) => format!("Failed to run cargo: {msg}"),
            TestCoverageRenderError::InvalidLlvmCovInput => "Invalid llvm-cov input".into(),
        };
        write!(f, "TestCoverageRenderError: {}", message)
    }
}
