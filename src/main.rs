use std::fs;

mod args;
mod commands;
mod env;
mod github;
mod package;
mod utils;

fn main() {
    let installation_root = env::installation_root();

    if installation_root.is_dir() {
        fs::create_dir_all(&installation_root.to_str().unwrap())
            .expect("Cannot create installation root");
    }

    let args = args::parse_args();

    if args.no_color {
        yansi::Paint::disable();
    }

    match args.command {
        args::Command::Search(args) => commands::search(args),
        args::Command::Install(args) => commands::install(args),
    }
}
