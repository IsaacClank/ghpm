use clap::{command, ArgAction, Args, Parser, Subcommand};

pub fn parse_args() -> Cli {
    Cli::parse()
}

#[derive(Parser)]
#[command(
    about,
    disable_help_subcommand = true,
    long_about = None,
    version,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, action = ArgAction::SetTrue)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Command {
    Search(SearchArgs),
    Install(InstallArgs),
}

#[derive(Args)]
pub struct SearchArgs {
    pub query: String,

    #[arg(long, short, default_value = "false")]
    pub short: bool,

    #[arg(long, short, default_value = "10")]
    pub limit: Option<u16>,

    #[arg(long, short, default_value = "1")]
    pub offset: Option<u16>,
}

#[derive(Args)]
pub struct InstallArgs {
    pub repo: String,
}
