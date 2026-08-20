use serde::Deserialize;

use crate::aws;

pub fn run(profile: Option<&str>, file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let json_output = if let Some(file_path) = file {
        aws::read_json_file(file_path)
    } else {
        aws::get_volumes_json(profile)
    };

    let parsed: DescribeVolumes = serde_json::from_str(&json_output)?;
    let unattached_volumes = find_unattached_volumes(&parsed.volumes);

    for v in unattached_volumes {
        match v.size {
            Some(size) => println!("Volume {} is unattached (size: {})", v.volume_id, size),
            None => println!("Volume {} is unattached (size: unknown)", v.volume_id),
        }
    }
    Ok(())
}

/// Top level response from EC2 describe-volumes.
#[derive(Deserialize)]
pub struct DescribeVolumes {
    #[serde(rename = "Volumes")]
    pub volumes: Vec<Volume>,
}

/// A single EBS Volume
#[derive(Deserialize)]
pub struct Volume {
    #[serde(rename = "VolumeId")]
    pub volume_id: String,

    #[serde(rename = "Size", default)]
    pub size: Option<i64>,

    #[serde(rename = "Attachments", default)]
    pub attachments: Vec<Attachment>,
}

/// An EBS volume attachment.
#[derive(Deserialize)]
pub struct Attachment {}

fn find_unattached_volumes(volumes: &[Volume]) -> Vec<&Volume> {
    volumes
        .iter()
        .filter(|v| v.attachments.is_empty())
        .collect()
}
