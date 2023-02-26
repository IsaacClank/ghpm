mod endpoints;
mod models;

use endpoints::Endpoint;
pub use models::*;

pub fn search_repo(
  query: &str,
  limit: Option<u16>,
  offset: Option<u16>,
) -> RepositorySearchResult {
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
  use std::process::Command;

  Command::new("curl")
    .arg("--output-dir")
    .arg("/tmp")
    .arg("-LO")
    .arg(url)
    .output()
    .unwrap();
}
