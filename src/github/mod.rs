mod endpoints;
mod models;

pub use models::*;

use endpoints::Endpoint;
use std::env::temp_dir;
use std::process::Command;

pub fn search_repo(query: &str, limit: Option<u16>, offset: Option<u16>) -> RepositorySearchResult {
    let url = Endpoint::SearchRepositories(query.to_string(), limit, offset)
        .to_full_url()
        .unwrap();

    let response: String = ureq::get(&url).call().unwrap().into_string().unwrap();

    serde_json::from_str(&response).unwrap()
}

pub fn get_latest_release(repo: &str) -> RepositoryRelease {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let response: String = ureq::get(&url).call().unwrap().into_string().unwrap();

    serde_json::from_str(&response).unwrap()
}

pub fn download(url: &str) {
    println!("Downloading release asset ...");

    Command::new("curl")
        .arg("--output-dir")
        .arg(temp_dir().to_str().unwrap())
        .arg("-LO")
        .arg(url)
        .output()
        .expect("Failed to download asset");
}
