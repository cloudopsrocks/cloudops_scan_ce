pub mod eips;
pub mod nat_gateways;

use crate::cli::VpcCommands;

pub fn run(command: VpcCommands, profile: Option<&str>, file: Option<&str>) {
    match command {
        VpcCommands::Eips => eips::run(profile, file),
        VpcCommands::NatGateways => nat_gateways::run(profile, file),
        VpcCommands::All => {
            eips::run(profile, file);
            nat_gateways::run(profile, file);
        }
    }
}
