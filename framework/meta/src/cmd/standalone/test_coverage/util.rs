use crate::cmd::standalone::test_coverage::error::TestCoverageError;
use std::{fs, process::Command};

const DEPENDENCIES: [&str; 2] = ["llvm-cov", "llvm-profdata"];

pub fn ensure_dependencies_in_path() -> Result<(), TestCoverageError> {
    for dependency in DEPENDENCIES.iter() {
        let Ok(_) = Command::new(dependency).arg("--version").output() else {
            return Err(TestCoverageError::MissingDependency(dependency.to_string()));
        };
    }

    Ok(())
}

pub fn deep_find_files_with_ext(dir: &str, ext: &str) -> Result<Vec<String>, TestCoverageError> {
    let mut result = vec![];

    let dir_contents = fs::read_dir(dir)
        .map_err(|_| TestCoverageError::FsError(format!("failed to read dir at path {dir}")))?;

    for entry in dir_contents {
        let entry = entry.map_err(|_| {
            TestCoverageError::FsError(format!("failed to read entry in dir at path {dir}"))
        })?;
        let path = entry.path();
        if path.is_dir() {
            result.append(&mut deep_find_files_with_ext(path.to_str().unwrap(), ext)?);
        } else if path.is_file() {
            if let Some(file_ext) = path.extension() {
                if file_ext == ext {
                    result.push(path.to_str().unwrap().to_string());
                }
            }
        }
    }

    Ok(result)
}

pub fn cleanup_many_files(files: &Vec<String>) -> Result<(), TestCoverageError> {
    for file in files {
        fs::remove_file(file).map_err(|_| {
            TestCoverageError::FsError(format!("failed to remove file at path {file}"))
        })?;
    }

    Ok(())
}

pub fn cleanup_file(file: &str) -> Result<(), TestCoverageError> {
    fs::remove_file(file)
        .map_err(|_| TestCoverageError::FsError(format!("failed to remove file at path {file}")))?;

    Ok(())
}
