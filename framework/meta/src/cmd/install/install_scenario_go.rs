use core::time;
use serde_json::Value;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    thread::sleep,
};

use multiversx_sc_meta_lib::print_util::println_green;

use super::system_info::{get_system_info, SystemInfo};

const USER_AGENT: &str = "multiversx-sc-meta";
const SCENARIO_CLI_RELEASES_BASE_URL: &str =
    "https://api.github.com/repos/multiversx/mx-chain-scenario-cli-go/releases";
const CARGO_HOME: &str = env!("CARGO_HOME");

#[derive(Clone, Debug)]
pub struct ScenarioGoRelease {
    #[allow(dead_code)]
    pub tag_name: String,
    pub download_url: String,
}

#[derive(Clone, Debug)]
pub struct ScenarioGoInstaller {
    tag: Option<String>,
    zip_name: String,
    user_agent: String,
    temp_dir_path: PathBuf,
    cargo_bin_folder: PathBuf,
}

fn select_zip_name() -> String {
    match get_system_info() {
        SystemInfo::Linux => "mx_scenario_go_linux_amd64.zip".to_string(),
        SystemInfo::MacOs => "mx_scenario_go_darwin_amd64.zip".to_string(),
    }
}

impl ScenarioGoInstaller {
    pub fn new(tag: Option<String>) -> Self {
        let cargo_home = PathBuf::from(CARGO_HOME);
        let cargo_bin_folder = cargo_home.join("bin");
        ScenarioGoInstaller {
            tag,
            zip_name: select_zip_name(),
            user_agent: USER_AGENT.to_string(),
            temp_dir_path: std::env::temp_dir(),
            cargo_bin_folder,
        }
    }

    pub async fn install(&self) {
        let release_raw = self
            .get_scenario_go_release_json()
            .await
            .expect("couldn't retrieve mx-chain-scenario-cli-go release");

        assert!(
            !release_raw.contains("\"message\": \"Not Found\""),
            "release not found: {release_raw}"
        );

        let release = self.parse_scenario_go_release(&release_raw);
        self.download_zip(&release)
            .await
            .expect("could not download artifact");

        self.unzip_binaries();
        self.delete_temp_zip();
    }

    fn release_url(&self) -> String {
        if let Some(tag) = &self.tag {
            format!("{SCENARIO_CLI_RELEASES_BASE_URL}/tags/{tag}")
        } else {
            format!("{SCENARIO_CLI_RELEASES_BASE_URL}/latest")
        }
    }

    async fn get_scenario_go_release_json(&self) -> Result<String, reqwest::Error> {
        let release_url = self.release_url();
        println_green(format!("Retrieving release info: {release_url}"));

        loop {
            let response = reqwest::Client::builder()
                .user_agent(&self.user_agent)
                .build()?
                .get(&release_url)
                .send()
                .await?
                .text()
                .await?;

            if !response.contains("API rate limit exceeded for") {
                return Ok(response);
            }

            println!("API rate limit exceeded, retrying...");
            sleep(time::Duration::from_secs(5));
        }
    }

    fn parse_scenario_go_release(&self, raw_json: &str) -> ScenarioGoRelease {
        let parsed: Value = serde_json::from_str(raw_json).unwrap();

        let tag_name = parsed
            .get("tag_name")
            .unwrap_or_else(|| panic!("tag name not found in response: {raw_json}"))
            .as_str()
            .expect("malformed json");

        let assets = parsed
            .get("assets")
            .expect("assets not found in release")
            .as_array()
            .expect("malformed json");

        let zip_asset = assets
            .iter()
            .find(|asset| self.asset_is_zip(asset))
            .expect("executable zip asset not found in release");

        let download_url = zip_asset
            .get("browser_download_url")
            .expect("asset download url not found")
            .as_str()
            .expect("asset download url not a string");

        ScenarioGoRelease {
            tag_name: tag_name.to_string(),
            download_url: download_url.to_string(),
        }
    }

    fn asset_is_zip(&self, asset: &Value) -> bool {
        let name = asset
            .get("name")
            .expect("asset name not found")
            .as_str()
            .expect("asset name not a string");
        name == self.zip_name
    }

    fn zip_temp_path(&self) -> PathBuf {
        self.temp_dir_path.join(&self.zip_name)
    }

    async fn download_zip(&self, release: &ScenarioGoRelease) -> Result<(), reqwest::Error> {
        println_green(format!("Downloading binaries: {}", &release.download_url));
        let response = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .build()?
            .get(&release.download_url)
            .send()
            .await?
            .bytes()
            .await?;
        if response.len() < 10000 {
            panic!(
                "Could not download artifact: {}",
                String::from_utf8_lossy(&response)
            );
        }

        println_green(format!("Saving to: {}", self.zip_temp_path().display()));
        let mut file = match File::create(self.zip_temp_path()) {
            Err(why) => panic!("couldn't create {why}"),
            Ok(file) => file,
        };
        file.write_all(&response).unwrap();
        Ok(())
    }

    fn unzip_binaries(&self) {
        println_green(format!("Unzipping to: {}", self.cargo_bin_folder.display()));
        let file = File::open(self.zip_temp_path()).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        zip.extract(Path::new(&self.cargo_bin_folder))
            .expect("Could not unzip artifact");
    }

    fn delete_temp_zip(&self) {
        println_green(format!(
            "Deleting temporary download: {}",
            self.zip_temp_path().display()
        ));
        std::fs::remove_file(self.zip_temp_path()).unwrap();
    }
}
