use clap::{Parser, Subcommand};

/// Command-Line interface definition
#[derive(Parser)]
#[command(name = "cloudops-scan-ce")]
#[command(about = "AWS wastedetection cli", long_about = None)]

pub struct Cli {
 /// the Subcommand
 #[command(subcommand)]
 pub command: Commands,
}

/// Supported subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Scan AWS Resources
    Scan {
        /// Optional AWS profile
        #[arg(long)]
        profile: Option<String>,

        /// Optional local JSON file
        #[arg(long)]
        file: Option<String>,
    },
}
