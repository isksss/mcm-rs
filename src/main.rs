mod config;
mod utils;
mod commands;

use utils::show_logo;
use clap::{Args, Parser, Subcommand};
use commands::InitCommand;

#[derive(Parser)]
#[command(name="mcm")]
#[command(about="")]
struct Cli {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands{
    Init{
        
    },
    Start{

    }
}

fn main() {
    show_logo();
    let args = Cli::parse();

    let config_file = "mcm.toml";
    match args.subcmd {
        Commands::Init{} => {
            commands::InitCommand(config_file);
        },
        Commands::Start{} => {
            println!("start");
        }
    }
}
