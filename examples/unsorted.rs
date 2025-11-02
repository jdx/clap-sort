use clap::Subcommand;

/// This example shows INCORRECTLY sorted subcommands (will fail validation)
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
    println!("This is an example of incorrectly sorted subcommands");
}
