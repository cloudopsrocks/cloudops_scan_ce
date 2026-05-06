use crate::ec2::models::Volume;

/// Return all unattached EBS volumes.
pub fn find_unattached_volumes(volumes: &[Volume]) -> Vec<&Volume> {
    volumes
        .iter()
        .filter(|v| v.attachments.is_empty())
        .collect()
}
