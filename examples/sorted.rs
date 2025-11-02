use clap::Subcommand;

/// This example shows correctly sorted subcommands
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
    println!("This is an example of correctly sorted subcommands");
}
