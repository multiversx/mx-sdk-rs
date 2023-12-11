use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateMetadata {
    pub name: String,
    pub contract_trait: String,
    pub src_file: String,
    pub rename_pairs: Vec<(String, String)>,
    pub files_include: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed_metadata() -> TemplateMetadata {
        toml::from_str(
            r#"
            name = "my-contract"
            contract_trait = "MyContract"
            src_file = "my_contract.rs"
            rename_pairs = [
                ["original", "new"]
            ]
            files_include = [
                "/meta",
                "/src",
                "/wasm",
                "/Cargo.toml"
            ]
        "#,
        )
        .unwrap()
    }

    #[test]
    fn test_template_metadata_parse() {
        let parsed = parsed_metadata();
        assert_eq!(parsed.name, "my-contract");
        assert_eq!(parsed.contract_trait, "MyContract");
        assert_eq!(parsed.src_file, "my_contract.rs");
        assert_eq!(parsed.rename_pairs.len(), 1);
        assert_eq!(parsed.rename_pairs[0].0, "original");
        assert_eq!(parsed.rename_pairs[0].1, "new");
        assert_eq!(parsed.files_include.len(), 4);
        assert_eq!(parsed.files_include[0], "/meta");
        assert_eq!(parsed.files_include[1], "/src");
        assert_eq!(parsed.files_include[2], "/wasm");
        assert_eq!(parsed.files_include[3], "/Cargo.toml");
    }
}
