//use crate::vpc::models::Address;
use serde::Deserialize;

use crate::aws;

pub fn run(profile: Option<&str>, file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let json_output = if let Some(file_path) = file {
        aws::read_json_file(file_path)
    } else {
        aws::get_addresses_json(profile)
    };

    let parsed: DescribeAddresses = serde_json::from_str(&json_output)?;
    let unattached_ips = find_unattached_addresses(&parsed.addresses);

    for a in unattached_ips {
        println!("Elastic IP {} is unattached", a.allocation_id);
    }
    Ok(())
}

/// Top level response from EC2 describe-addresses.
#[derive(Deserialize)]
pub struct DescribeAddresses {
    #[serde(rename = "Addresses")]
    pub addresses: Vec<Address>,
}

/// A single Elastic address.
#[derive(Deserialize)]
pub struct Address {
    #[serde(rename = "AllocationId")]
    pub allocation_id: String,

    #[serde(rename = "AssociationId", default)]
    pub association_id: Option<String>,

    #[serde(rename = "PublicIp")]
    pub public_ip: String,

    #[serde(rename = "InstanceId", default)]
    pub instance_id: Option<String>,

    #[serde(rename = "NetworkInterfaceId", default)]
    pub network_interface_id: Option<String>,
}

/// Return all unattached EC2 addresses.
fn find_unattached_addresses(addresses: &[Address]) -> Vec<&Address> {
    addresses
        .iter()
        .filter(|a| a.association_id.is_none())
        .collect()
}
