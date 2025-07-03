use reqwest::Client;
use serde_json::Value;
use std::error::Error;

use crate::files;

pub async fn fetch_template(file_type: &files::FileType, identifier: &str) -> Result<String, Box<dyn Error>> {
    let base_url = "https://api.github.com";

    let client = Client::new();
    let (url, header_value, attribute): (String, &str, &str) = match file_type {
        files::FileType::GitIgnore => (
            format!("{}/gitignore/templates/{}", base_url, identifier),
            "rust_gitignore_fetcher",
            "source"
        ),
        files::FileType::License => (
            format!("{}/licenses/{}", base_url, identifier),
            "rust_license_fetcher",
            "body"
        ),
        files::FileType::EnvExample | files::FileType::Readme => {
            return Err("Unsupported file type for remote fetch.".into());
        }
    };

    let response: Value = client
        .get(url)
        .header("User-Agent", header_value)
        .send()
        .await?
        .json()
        .await?;

    if let Some(body) = response.get(attribute).and_then(|v| v.as_str()) {
        Ok(body.to_string())
    } else {
        Err(format!("{} file for '{}' not found or has no content.", file_type.to_string(), identifier).into())
    }
}
