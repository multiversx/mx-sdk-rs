use std::{fs, io::Write, path::Path};

use serde::{Deserialize, Serialize};

use crate::serde_raw::StepRaw;

/// Mapped 1-on-1 with the JSON. No complex logic here, just a basic interface with the JSON.
/// The conversion to `Scenario` adds all additional functionality.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_gas: Option<bool>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_schedule: Option<String>,

    pub steps: Vec<StepRaw>,
}

impl ScenarioRaw {
    pub fn from_json_str(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }

    pub fn load_from_file<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(path).unwrap();
        Self::from_json_str(contents.as_str())
    }

    pub fn to_json_string(&self) -> String {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        self.serialize(&mut ser).unwrap();
        let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
        serialized.push('\n');
        serialized
    }

    pub fn save_to_file<P>(&self, path: P)
    where
        P: AsRef<Path>,
    {
        let json_string = self.to_json_string();
        let path_parent = path.as_ref().parent().unwrap();
        fs::create_dir_all(path_parent).unwrap();
        let mut file = fs::File::create(path).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
    }
}
