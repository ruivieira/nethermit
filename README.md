# Nethermit

A simple CLI tool to convert Jsonnet files to YAML.

## Installation

```bash
cargo install --path .
```

## Usage

The tool accepts input either from a file or from standard input:

```bash
# Read from a file
nethermit config.jsonnet > output.yaml

# Read from stdin (pipe)
cat config.jsonnet | nethermit > output.yaml

# Read from stdin (explicit)
nethermit - < config.jsonnet > output.yaml

# Read from stdin (interactive)
nethermit
# Type or paste your Jsonnet code
# Press Ctrl+D (Unix) or Ctrl+Z (Windows) to finish
```

## Features

- Converts Jsonnet files to YAML format
- Supports standard Jsonnet imports
- Accepts input from files or stdin
- Outputs to stdout for easy piping

## Requirements

- Rust 1.70 or later