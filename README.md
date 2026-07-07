# CloudOps Scan CE

CloudOps Scan CE is a command-line utility for discovering common sources of wasted AWS resources. It uses your existing AWS CLI configuration and **never stores AWS credentials**.

The Community Edition is intended to help AWS administrators and engineers quickly identify resources that may no longer be in use.

---

## Prerequisites

Before running CloudOps Scan CE, ensure you have:

* AWS CLI v2 installed
* An AWS IAM user or role with **read-only** permissions
* An AWS CLI profile configured (recommended)

Verify your AWS CLI installation:

```bash
aws --version
```

List configured profiles:

```bash
aws configure list-profiles
```

---

## Installation

Download the appropriate binary for your operating system from the Releases page.

Example (Linux/macOS):

```bash
chmod +x cloudops-scan-ce
```

Run the executable:

```bash
./cloudops-scan-ce
```

Windows:

```powershell
cloudops-scan-ce.exe
```

---

## Using AWS Profiles

CloudOps Scan CE uses your existing AWS CLI credentials.

To scan using your default AWS profile:

```bash
cloudops-scan-ce ec2 volumes
```

To scan using a named profile:

```bash
cloudops-scan-ce --profile production ec2 volumes
```

Example using a profile named `cli-scan`:

```bash
cloudops-scan-ce --profile cli-scan ec2 volumes
```

---

## Offline Mode

If you already have exported AWS CLI JSON, you can scan it without connecting to AWS.

Example:

```bash
cloudops-scan-ce --file volumes.json ec2 volumes
```

This is useful for:

* Testing
* Sharing sanitized data
* Running scans without AWS credentials
* CI/CD pipelines

---

## Available Commands

Scan all EC2 checks:

```bash
cloudops-scan-ce ec2 all
```

Scan orphaned EBS volumes:

```bash
cloudops-scan-ce ec2 volumes
```

Scan EBS snapshots:

```bash
cloudops-scan-ce ec2 snapshots
```

Scan all VPC checks:

```bash
cloudops-scan-ce vpc all
```

Scan unattached Elastic IP addresses:

```bash
cloudops-scan-ce vpc eips
```

Scan NAT Gateways:

```bash
cloudops-scan-ce vpc nat-gateways
```

---

## Example Output

```text
Unattached EBS Volumes

Volume: vol-0123456789abcdef
Size:   100 GiB

Volume: vol-0fedcba987654321
Size:   20 GiB
```

---

## Security

CloudOps Scan CE is designed to operate with **read-only AWS permissions**.

The tool:

* Does not modify AWS resources
* Does not delete resources
* Does not create resources
* Does not store AWS credentials

It only retrieves metadata needed to identify potential waste.

---

## Roadmap

Community Edition focuses on identifying common sources of AWS waste.

Planned scan targets include:

* Orphaned EBS Volumes
* Old EBS Snapshots
* Unattached Elastic IPs
* Idle NAT Gateways

Future editions will include additional scan types, enhanced reporting, and a terminal user interface (TUI).

---

## Feedback

Bug reports, feature requests, and pull requests are welcome.

If CloudOps Scan CE helps you reduce AWS costs, consider starring the repository and sharing it with others.

