mod aws;
mod cli;
mod ec2;

use clap::{Parser};
use cli::{Cli, Commands};
use ec2::models::DescribeVolumes;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { profile, file } => {
            let json_output = if let Some(file_path) = file {
                aws::read_json_file(&file_path)
            } else {
                aws::get_volumes_json(profile.as_deref())
            };

            let parsed: DescribeVolumes = serde_json::from_str(&json_output).expect("Failed to parse JSON");

            let unused_volumes = ec2::rules::find_unattached_volumes(&parsed.volumes);

            if unused_volumes.is_empty() {
                println!("No unused volumes found.");
            } else {
                println!("Unattached EBS volumes:");

                for v in unused_volumes {
                    match v.size {
                        Some(size) => {
                            println!("- {} (Size: {} GiB)", v.volume_id, size);
                        }
                        None => {
                            println!("- {} (Size: unknown)", v.volume_id);
                        }
                    }
                }
            }
        }
    }
}
