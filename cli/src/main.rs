use clap::Parser;
use clap_sort::validate_file_path;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "clap-sort")]
#[command(about = "Validate that clap Subcommand enums are sorted alphabetically")]
struct Cli {
    /// Rust source files to validate
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        eprintln!("No files specified");
        std::process::exit(1);
    }

    let mut had_errors = false;

    for file in &cli.files {
        match validate_file_path(file) {
            Ok(()) => {
                println!("✓ {}: All Subcommand enums are sorted", file.display());
            }
            Err(errors) => {
                had_errors = true;
                eprintln!("✗ {}: Found {} error(s)", file.display(), errors.len());
                for error in errors {
                    eprintln!("  {}", error.message);
                }
            }
        }
    }

    if had_errors {
        std::process::exit(1);
    }

    Ok(())
}
