use serde::{Deserialize, Deserializer};
use yansi::Paint;

#[derive(Deserialize)]
pub struct Repo {
  #[serde(rename = "full_name")]
  pub name: String,

  #[serde(rename = "html_url")]
  pub url: String,

  #[serde(rename = "description", deserialize_with = "parse_nullable_string")]
  pub description: String,
}

fn parse_nullable_string<'de, D>(data: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  Deserialize::deserialize(data).map(|value: Option<String>| {
    value.unwrap_or(String::from("No description"))
  })
}

#[derive(Deserialize)]
pub struct RepoSearchResult {
  pub items: Vec<Repo>,
}

impl RepoSearchResult {
  pub fn to_string_long(&self) -> String {
    self
      .items
      .iter()
      .map(|item| {
        format!(
          "{}\n  {}",
          Paint::green(&item.name),
          Paint::default(&item.description).dimmed()
        )
      })
      .reduce(|reduction, item_string| format!("{reduction}\n{item_string}"))
      .unwrap()
  }

  pub fn to_string_short(&self) -> String {
    self
      .items
      .iter()
      .map(|item| format!("{}", Paint::green(&item.name)))
      .reduce(|reduction, item_string| format!("{reduction}\n{item_string}"))
      .unwrap()
  }
}

#[derive(Deserialize)]
pub struct Release {
  pub assets: Vec<ReleaseAsset>,
}

impl Release {
  pub fn print_download_urls(&self) {
    self
      .assets
      .iter()
      .enumerate()
      .for_each(|(index, asset)| println!("{index}. {}", asset.download_url));
  }

  pub fn get_download_url(&self, index: usize) -> &str {
    &self.assets[index].download_url
  }
}

#[derive(Deserialize)]
pub struct ReleaseAsset {
  #[serde(rename = "browser_download_url")]
  pub download_url: String,
}

pub fn search_repo(
  query: &str,
  per_page: Option<u16>,
  page: Option<u16>,
) -> RepoSearchResult {
  let mut url = format!("https://api.github.com/search/repositories?q={query}");

  if let Some(value) = per_page {
    url = format!("{url}&per_page={value}");
  }

  if let Some(value) = page {
    url = format!("{url}&page={value}");
  }

  let response: String = ureq::get(&url).call().unwrap().into_string().unwrap();
  serde_json::from_str(&response).unwrap()
}

pub fn get_latest_release(repo: &str) -> Release {
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
