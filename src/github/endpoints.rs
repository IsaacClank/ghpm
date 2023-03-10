const GITHUB_API: &str = "https://api.github.com";

pub enum Endpoint {
    SearchRepositories(String, Option<u16>, Option<u16>),
}

impl Endpoint {
    #[allow(irrefutable_let_patterns)]
    pub fn to_full_url(&self) -> Result<String, ()> {
        if let Endpoint::SearchRepositories(query, limit, offset) = self {
            let mut url = format!("{GITHUB_API}/search/repositories?q={query}");

            if let Some(value) = limit {
                url = format!("{url}&per_page={value}");
            }

            if let Some(value) = offset {
                url = format!("{url}&page={value}");
            }

            return Ok(url);
        }

        Err(())
    }
}
