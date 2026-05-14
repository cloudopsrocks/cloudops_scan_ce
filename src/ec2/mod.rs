pub mod snapshots;
pub mod volumes;

use crate::cli::Ec2Commands;

pub fn run(
    command: Ec2Commands,
    profile: Option<&str>,
    file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Ec2Commands::Volumes => {
            volumes::run(profile, file)?;
            Ok(())
        }
        Ec2Commands::Snapshots => {
            snapshots::run(profile, file)?;
            Ok(())
        }
        Ec2Commands::All => {
            volumes::run(profile, file)?;
            snapshots::run(profile, file)?;
            Ok(())
        }
    }
}
