use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    file: Option<PathBuf>,
}

fn process_jsonnet_file(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    // Add the parent directory of the input file to the import paths
    let import_paths = if let Some(parent) = path.parent() {
        vec![parent.to_path_buf()]
    } else {
        vec![]
    };

    nethermit::jsonnet_to_yaml(&content, &import_paths)
}

fn process_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;

    // When reading from stdin, use current directory for imports
    let import_paths = vec![PathBuf::from(".")];
    nethermit::jsonnet_to_yaml(&buffer, &import_paths)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let yaml = match cli.file {
        Some(ref path) if path == Path::new("-") => process_stdin()?,
        Some(ref path) => process_jsonnet_file(path)?,
        None => process_stdin()?,
    };

    println!("{yaml}");
    Ok(())
}
