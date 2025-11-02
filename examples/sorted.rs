use clap::{Parser, Subcommand, CommandFactory};

/// This example shows correctly sorted subcommands
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new item
    Add,

    /// Delete an item
    Delete,

    /// List all items
    List,

    /// Update an existing item
    Update,
}

fn main() {
    // Validate that commands are sorted
    clap_sort::assert_sorted(&Cli::command());
    println!("✓ All subcommands are sorted alphabetically!");
}
