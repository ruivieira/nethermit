use anyhow::{Context, Result};
use clap::Parser;
use std::{path::{PathBuf, Path}, fs, io::{self, Read}};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the Jsonnet file, or '-' to read from stdin
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

/// Read file contents and convert to YAML
fn process_jsonnet_file(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    
    nethermit::jsonnet_to_yaml(&content, path.to_string_lossy().as_ref())
}

/// Read from stdin and convert to YAML
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
    
    println!("{}", yaml);
    Ok(())
}
