use crate::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Package {
    installation_path: PathBuf,
}

impl Package {
    pub fn new(name: &str) -> Package {
        Package {
            installation_path: PathBuf::new()
                .join(env::Env::new().installation_root)
                .join(name),
        }
    }

    pub fn is_installed(&self) -> bool {
        self.installation_path.is_dir()
    }

    pub fn installation_path_as_str(&self) -> &str {
        &self.installation_path.to_str().unwrap()
    }
}
