mod aws;
mod cli;
mod ec2;
mod vpc;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ec2 { command } => {
            ec2::run(command, cli.profile.as_deref(), cli.file.as_deref());
        }

        Commands::Vpc { command } => {
            vpc::run(command, cli.profile.as_deref(), cli.file.as_deref());
        }
    }
}
