# Prefix Aggregator

A command-line tool for efficiently aggregating IPv4 prefixes into the smallest possible set of CIDR blocks.

## Features

- Aggregates adjacent and overlapping IPv4 CIDR blocks
- Optimizes for the smallest number of resulting prefixes
- Works with standard input/output for easy integration with other tools
- Cross-platform support (Linux, macOS, Windows)

## Installation

### Download Binary

Pre-built binaries are available for various platforms on the [Releases page](https://github.com/ugwis/prefix-aggregator/releases).

1. Download the appropriate binary for your platform:
   - Linux: `prefix-aggregator-x86_64-unknown-linux-gnu.zip`
   - Windows: `prefix-aggregator-x86_64-pc-windows-gnu.zip`
   - macOS (Intel): `prefix-aggregator-x86_64-apple-darwin.zip`
   - macOS (Apple Silicon): `prefix-aggregator-aarch64-apple-darwin.zip`

2. Extract the ZIP file and place the binary in a directory included in your PATH.

### macOS Security Warning

When running the application on macOS, you might see a security warning: "Apple could not verify this app is free from malware."

To resolve this:

1. Right-click (or Control-click) on the `prefix-aggregator` binary
2. Select "Open" from the context menu
3. Click "Open" in the dialog that appears
4. The app will now be saved as an exception to your security settings

### Building from Source

If you have Rust installed, you can build from source:

```bash
git clone https://github.com/ugwis/prefix-aggregator.git
cd prefix-aggregator
cargo build --release
```

The binary will be available at `target/release/prefix-aggregator`.

## Usage

### Basic Usage

Provide a list of IP prefixes via standard input:

```bash
prefix-aggregator < list_of_prefixes.txt
```

Or use a here document:

```bash
prefix-aggregator << EOF
192.168.1.0/24
192.168.2.0/24
192.168.3.0/24
EOF
```

### Examples

#### Example 1: Aggregate Adjacent Prefixes

Input:
```
10.0.0.0/24
10.0.1.0/24
10.0.2.0/24
```

Output:
```
10.0.0.0/23
10.0.2.0/24
```

The tool automatically combines `10.0.0.0/24` and `10.0.1.0/24` into `10.0.0.0/23`.

#### Example 2: Handle Overlapping Prefixes

Input:
```
10.0.0.0/8
10.0.0.0/16
10.0.1.0/16
```

Output:
```
10.0.0.0/8
```

The tool recognizes that `10.0.0.0/8` already includes the other prefixes.

#### Example 3: Real-world Application

Aggregating AWS IP ranges:

```bash
curl -s https://ip-ranges.amazonaws.com/ip-ranges.json | jq -r '.prefixes[].ip_prefix' | sort -V | uniq | wc -l
# 4314 prefixes before aggregation

curl -s https://ip-ranges.amazonaws.com/ip-ranges.json | jq -r '.prefixes[].ip_prefix' | sort -V | uniq | prefix-aggregator | wc -l
# 1101 prefixes after aggregation
```

This example shows how the tool can reduce 4314 AWS IP prefixes to just 1101 prefixes.

## How It Works

The prefix-aggregator uses an algorithm that:

1. Sorts the input prefixes
2. Identifies adjacent prefixes that can be combined
3. Merges overlapping prefixes
4. Recursively applies these operations until no further aggregation is possible

## License

[MIT License](LICENSE)
