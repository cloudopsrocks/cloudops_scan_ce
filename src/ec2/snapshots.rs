use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::aws;

const SNAPSHOT_AGE_DAYS: i64 = 90;

pub fn run(profile: Option<&str>, file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let json_output = if let Some(file_path) = file {
        aws::read_json_file(file_path)
    } else {
        aws::get_snapshots_json(profile)
    };

    let parsed: DescribeSnapshots = serde_json::from_str(&json_output)?;

    let old_snapshots = find_old_snapshots(&parsed.snapshots);

    if old_snapshots.is_empty() {
        println!("No snapshots older than {} days found.", SNAPSHOT_AGE_DAYS);
    } else {
        println!("Snapshots older than {} days:", SNAPSHOT_AGE_DAYS);

        for snapshot in old_snapshots {
            println!(
                "Snapshot {} Created {} Description {}",
                snapshot.snapshot_id, snapshot.start_time, snapshot.description
            );
        }
    }

    Ok(())
}

/// Top level response from EC2 describe-snapshots.
#[derive(Deserialize)]
pub struct DescribeSnapshots {
    #[serde(rename = "Snapshots")]
    pub snapshots: Vec<Snapshot>,
}

/// A single EBS snapshot.
#[derive(Deserialize)]
pub struct Snapshot {
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,

    #[serde(rename = "VolumeId")]
    pub volume_id: String,

    #[serde(rename = "StartTime")]
    pub start_time: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "VolumeSize")]
    pub volume_size: i32,
}

/// Return snapshots older than the defined review period.
fn find_old_snapshots(snapshots: &[Snapshot]) -> Vec<&Snapshot> {
    let cutoff = Utc::now() - chrono::Duration::days(SNAPSHOT_AGE_DAYS);

    snapshots
        .iter()
        .filter(|snapshot| {
            DateTime::parse_from_rfc3339(&snapshot.start_time)
                .map(|date| date.with_timezone(&Utc) < cutoff)
                .unwrap_or(false)
        })
        .collect()
}
