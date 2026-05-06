use serde::Deserialize;

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

    #[serde(rename = "Size")]
    pub size: Option<i32>,

    #[serde(rename = "Attachments", default)]
    pub attachments: Vec<Attachment>,

}

/// An EBS volume attachment.
#[derive(Deserialize)]
pub struct Attachment {}
