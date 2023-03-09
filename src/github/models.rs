use serde::Deserialize;
use yansi::Paint;

#[derive(Deserialize)]
pub struct Repository {
    #[serde(rename = "full_name")]
    pub name: String,

    #[serde(rename = "html_url")]
    pub url: String,

    #[serde(rename = "description")]
    pub description: Option<String>,
}

impl Repository {
    pub fn description_or_default(&self) -> &str {
        match self.description.as_ref() {
            Some(description) => &description,
            None => "No description",
        }
    }
}

#[derive(Deserialize)]
pub struct RepositorySearchResult {
    pub items: Vec<Repository>,
}

impl RepositorySearchResult {
    pub fn to_string_long(&self) -> String {
        self.items
            .iter()
            .map(|item| {
                format!(
                    "{}\n  {}",
                    Paint::green(&item.name),
                    Paint::default(item.description_or_default()).dimmed()
                )
            })
            .reduce(|reduction, item_string| format!("{reduction}\n{item_string}"))
            .unwrap()
    }

    pub fn to_string_short(&self) -> String {
        self.items
            .iter()
            .map(|item| format!("{}", Paint::green(&item.name)))
            .reduce(|reduction, item_string| format!("{reduction}\n{item_string}"))
            .unwrap()
    }
}

#[derive(Deserialize)]
pub struct RepositoryRelease {
    pub assets: Vec<RepositoryAsset>,
}

impl RepositoryRelease {
    pub fn print_download_urls(&self) {
        self.assets
            .iter()
            .enumerate()
            .for_each(|(index, asset)| println!("{index}. {}", asset.download_url));
    }

    pub fn get_asset_by_index(&self, index: usize) -> &RepositoryAsset {
        &self.assets[index]
    }
}

#[derive(Deserialize)]
pub struct RepositoryAsset {
    pub name: String,

    #[serde(rename = "browser_download_url")]
    pub download_url: String,
}
