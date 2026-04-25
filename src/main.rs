use clap::{Parser, Subcommand};
use std::process::Command;

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
            let json_output = if let Some(file_path) = &file {
                std::fs::read_to_string(file_path).expect("Failed to read file")
            } else {
                let mut cmd = Command::new("aws");

                if let Some(profile_name) = &profile {
                    cmd.args(["--profile", profile_name]);
                }

                let output = cmd
                    .args(["ec2", "describe-volumes", "--output", "json"])
                    .output()
                    .expect("Failed to execute AWS CLI");

                if !output.status.success() {
                    panic!("AWS CLI command failed");
                }

                String::from_utf8_lossy(&output.stdout).to_string()
            };
            println!("Running scan ...");
            println!("Profile: {:?}", profile);
            println!("File: {:?}", file);
            println!("{}", json_output);
        }
    }
}
