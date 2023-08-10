use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateMetadata {
    pub name: String,
    pub contract_trait: String,
    pub src_file: String,
    pub rename_pairs: Vec<(String, String)>,
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
    }
}
