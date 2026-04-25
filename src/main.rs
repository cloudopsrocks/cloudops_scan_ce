use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cloudops-scan-ce")]
#[command(about = "AWS waste detection CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Scan AWS Resources
    Scan {
        // Profile to use if supplied
        #[arg(long)]
        profile: Option<String>,

        // Use local JSON file instead of AWS CLI
        #[arg(long)]
        file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { profile, file } => {
            println!("Running scan ...");
            println!("Profile: {:?}", profile);
            println!("File: {:?}", file);
        }
    }
}
