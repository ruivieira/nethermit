# Nethermit

[![Rust CI](https://github.com/ruivieira/nethermit/actions/workflows/rust.yml/badge.svg)](https://github.com/ruivieira/nethermit/actions/workflows/rust.yml)

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

## Development

### Pre-commit Hooks

This project uses Git pre-commit hooks to ensure code quality. The hooks run:
- Code formatting checks (`cargo fmt`)
- Linting (`cargo clippy`)
- Build verification
- Unit tests

The hooks are automatically installed when you clone the repository. If you need to install them manually:

```bash
git config core.hooksPath .githooks
```

### Requirements

- Rust 1.70 or later