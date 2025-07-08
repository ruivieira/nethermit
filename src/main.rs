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

    nethermit::jsonnet_to_yaml(&content, path.to_string_lossy().as_ref())
}

fn process_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;

    nethermit::jsonnet_to_yaml(&buffer, "<stdin>")
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
