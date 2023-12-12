mod commands;
mod config;
mod utils;

use clap::{Args, Parser, Subcommand};
use utils::show_logo;

#[derive(Parser)]
#[command(name = "mcm")]
#[command(about = "")]
struct Cli {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init {},
    Start {},
}

fn main() {
    show_logo();
    let args = Cli::parse();

    let config_file = "mcm.toml";
    match args.subcmd {
        Commands::Init {} => {
            let config = commands::init_command(config_file);
        }
        Commands::Start {} => {
            commands::start_command(config_file);
        }
    }
}
