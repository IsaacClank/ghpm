use std::path::PathBuf;

pub struct Env {
    pub installation_root: PathBuf,
}

impl Env {
    pub fn new() -> Env {
        Env {
            installation_root: {
                let home_dir = std::env::var("HOME").unwrap();
                let home_dir = format!("{home_dir}/.local/bin");

                PathBuf::new().join(home_dir)
            },
        }
    }
}
