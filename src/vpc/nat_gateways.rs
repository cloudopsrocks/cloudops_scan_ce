use serde::Deserialize;

use crate::aws;

pub fn run(profile: Option<&str>, file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let json_output = if let Some(file_path) = file {
        aws::read_json_file(file_path)
    } else {
        aws::get_nat_gateways_json(profile)
    };

    let parsed: DescribeNatGateways = serde_json::from_str(&json_output)?;

    let active_nat_gateways = find_active_nat_gateways(&parsed.nat_gateways);

    if active_nat_gateways.is_empty() {
        println!("No active NAT Gateways found.");
    } else {
        println!("Active NAT Gateways:");

        for nat in active_nat_gateways {
            println!(
                "NAT Gateway {} is active in VPC {}, subnet {} and should be reviewed for usage",
                nat.nat_gateway_id, nat.vpc_id, nat.subnet_id
            );
        }
    }

    Ok(())
}

/// Top level response from EC2 describe-nat-gateways.
#[derive(Deserialize)]
pub struct DescribeNatGateways {
    #[serde(rename = "NatGateways")]
    pub nat_gateways: Vec<NatGateway>,
}

/// A single NAT Gateway.
#[derive(Deserialize)]
pub struct NatGateway {
    #[serde(rename = "NatGatewayId")]
    pub nat_gateway_id: String,

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "VpcId")]
    pub vpc_id: String,

    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
}

/// Return NAT Gateways that are active and generating potential cost.
fn find_active_nat_gateways(nat_gateways: &[NatGateway]) -> Vec<&NatGateway> {
    nat_gateways
        .iter()
        .filter(|nat| nat.state == "available")
        .collect()
}
