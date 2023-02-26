mod args;
mod commands;
mod github;
mod utils;

fn main() {
  let args = args::parse_args();

  if args.no_color {
    yansi::Paint::disable();
  }

  match args.command {
    args::Command::Search(args) => commands::search(args),
    args::Command::Install(args) => commands::install(args),
  }
}
