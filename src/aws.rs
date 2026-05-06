use std::process::Command;

/// Read JSON from a local file
pub fn read_json_file(path: &str) -> String {
    std::fs::read_to_string(path)
        .expect("Failed to read JSON file")
}

/// Fetch EC22 volume JSON from AWS CLI
pub fn get_volumes_json(profile: Option<&str>) -> String {
    let mut cmd = Command::new("aws");

    if let Some(profile_name) = profile {
        cmd.args(["--profile", profile_name]);
    }
    let output = cmd
        .args(["ec2", "describe-volumes", "--output", "json"])
        .output()
        .expect("Failed to execute AWS CLI");

    if !output.status.success() {
        panic!("AWS CLI command failed");
    }
    String::from_utf8_lossy(&output.stdout).to_string()}
