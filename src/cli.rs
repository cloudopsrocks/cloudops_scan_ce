use clap::{Parser, Subcommand};

/// Command-Line interface definition
#[derive(Parser)]
#[command(name = "cloudops-scan-ce")]
#[command(about = "AWS wastedetection cli", long_about = None)]

pub struct Cli {
    /// AWS profile to use
    #[arg(long)]
    pub profile: Option<String>,

    /// Optional input file override
    #[arg(long)]
    pub file: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Ec2(Ec2Commands),
    Vpc(VpcCommands),
}

#[derive(Subcommand)]
pub enum Ec2Commands {
    Volumes,
    Snapshots,
    All,
}

#[derive(Subcommand)]
pub enum VpcCommands {
    Eips,
    NatGateways,
    All,
}
