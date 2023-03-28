use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateMetadata {
    pub name: String,
    pub rename_pairs: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed_metadata() -> TemplateMetadata {
        toml::from_str(
            r#"
            name = "contract-name"
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
        assert_eq!(parsed.name, "contract-name");
        assert_eq!(parsed.rename_pairs.len(), 1);
        assert_eq!(parsed.rename_pairs[0].0, "original");
        assert_eq!(parsed.rename_pairs[0].1, "new");
    }
}
