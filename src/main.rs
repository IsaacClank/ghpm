use std::fs;

mod args;
mod commands;
mod env;
mod github;
mod utils;

fn main() {
  let env = env::Env::new();

  if !env.installation_root.is_dir() {
    fs::create_dir_all(&env.installation_root).expect("Cannot create installation root");
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
