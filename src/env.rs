use std::path::PathBuf;

pub struct Env {
    pub installation_root: PathBuf,
}

impl Env {
    pub fn new() -> Env {
        Env {
            installation_root: {
                let home_dir = std::env::var("HOME").unwrap();
                let home_dir = format!("{home_dir}/.local/opt");

                PathBuf::new().join(home_dir)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_test_instantiates_struct() {
        assert!(Env::new().installation_root.to_str().is_some());
    }
}
