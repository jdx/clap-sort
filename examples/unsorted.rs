use clap::{Parser, Subcommand, CommandFactory};

/// This example shows INCORRECTLY sorted subcommands (will panic)
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all items
    List,

    /// Add a new item
    Add,

    /// Update an existing item
    Update,

    /// Delete an item
    Delete,
}

fn main() {
    // This will panic because commands are not sorted!
    clap_sort::assert_sorted(&Cli::command());
    println!("This line will never be reached");
}
