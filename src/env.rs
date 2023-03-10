use std::path::PathBuf;

pub fn installation_root() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap();
    let home_dir = format!("{home_dir}/.local/opt");

    PathBuf::new().join(home_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn installation_root_test() {
        assert!(installation_root().to_str().is_some());
    }
}
