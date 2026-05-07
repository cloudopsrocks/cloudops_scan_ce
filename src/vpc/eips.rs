use serde::Deserialize;

/// Top level response from EC2 describe-addresses.
#[derive(Deserialize)]
pub struct DescribeAddresses {
    #[serde(rename = "Addresses")]
    pub addresses: Vec<Address>,
}

/// A single EC2 address.
#[derive(Deserialize)]
pub struct Address {
    #[serde(rename = "AllocationId")]
    pub allocation_id: String,

    #[serde(rename = "PublicIp")]
    pub public_ip: String,

    #[serde(rename = "InstanceId", default)]
    pub instance_id: Option<String>,

    #[serde(rename = "NetworkInterfaceId", default)]
    pub network_interface_id: Option<String>,
}

use crate::vpc::models::Address;

/// Return all unattached EC1 addresses.
pub fn find_unattached_addresses(addresses: &[Address]) -> Vec<&Address> {
    addresses
        .iter()
        .filter(|a| a.instance_id.is_none() && a.network_interface_id.is_none())
        .collect()
}
