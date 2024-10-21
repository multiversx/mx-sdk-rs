mod cargo_toml_contents;
mod cargo_toml_deps;
mod version_req;

pub use cargo_toml_contents::{
    change_from_base_to_adapter_path, CargoTomlContents, CARGO_TOML_DEPENDENCIES,
    CARGO_TOML_DEV_DEPENDENCIES,
};
pub use cargo_toml_deps::{DependencyReference, GitReference};
pub use version_req::VersionReq;
