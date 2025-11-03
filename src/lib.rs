//! # clap-sort
//!
//! A library to validate that clap subcommands and arguments are sorted.
//!
//! This crate provides functionality to validate that:
//! - Subcommands are sorted alphabetically
//! - Arguments are grouped and sorted by type:
//!   1. Positional arguments (order not enforced - parsing order matters)
//!   2. Flags with short options (alphabetically by short option)
//!   3. Long-only flags (alphabetically)

/// Validates that subcommands and arguments are sorted correctly.
///
/// This checks:
/// - Subcommands are sorted alphabetically
/// - Arguments are grouped and sorted by type:
///   1. Positional arguments (order not enforced)
///   2. Flags with short options (alphabetically by short option)
///   3. Long-only flags (alphabetically)
///
/// Recursively validates all subcommands.
///
/// # Panics
/// Panics if subcommands or arguments are not properly sorted.
///
/// # Example
///
/// ```rust
/// use clap::{Command, Arg};
///
/// let cmd = Command::new("mycli")
///     .arg(Arg::new("file"))  // Positional
///     .arg(Arg::new("output").short('o').long("output"))  // Short flag
///     .arg(Arg::new("config").long("config"));  // Long-only flag
///
/// clap_sort::assert_sorted(&cmd);
/// ```
pub fn assert_sorted(cmd: &clap::Command) {
    assert_sorted_with_path(cmd, vec![]);
}

fn assert_sorted_with_path(cmd: &clap::Command, parent_path: Vec<&str>) {
    let mut current_path = parent_path.clone();
    current_path.push(cmd.get_name());

    // Check subcommands
    let subcommands: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();

    if !subcommands.is_empty() {
        let mut sorted = subcommands.clone();
        sorted.sort();

        if subcommands != sorted {
            panic!(
                "Subcommands in '{}' are not sorted alphabetically!\nActual order: {:?}\nExpected order: {:?}",
                current_path.join(" "),
                subcommands,
                sorted
            );
        }
    }

    // Check arguments
    assert_arguments_sorted_with_path(cmd, &current_path);

    // Recursively check subcommands
    for subcmd in cmd.get_subcommands() {
        assert_sorted_with_path(subcmd, current_path.clone());
    }
}

/// Checks if subcommands and arguments are sorted, returning a Result instead of panicking.
///
/// This checks:
/// - Subcommands are sorted alphabetically
/// - Arguments are grouped and sorted by type
///
/// Recursively validates all subcommands.
///
/// # Example
///
/// ```rust
/// use clap::Command;
///
/// let cmd = Command::new("mycli");
/// match clap_sort::is_sorted(&cmd) {
///     Ok(()) => println!("Everything is sorted!"),
///     Err(msg) => eprintln!("Error: {}", msg),
/// }
/// ```
pub fn is_sorted(cmd: &clap::Command) -> Result<(), String> {
    is_sorted_with_path(cmd, vec![])
}

fn is_sorted_with_path(cmd: &clap::Command, parent_path: Vec<&str>) -> Result<(), String> {
    let mut current_path = parent_path.clone();
    current_path.push(cmd.get_name());

    // Check subcommands
    let subcommands: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();

    if !subcommands.is_empty() {
        let mut sorted = subcommands.clone();
        sorted.sort();

        if subcommands != sorted {
            return Err(format!(
                "Subcommands in '{}' are not sorted alphabetically!\nActual order: {:?}\nExpected order: {:?}",
                current_path.join(" "),
                subcommands,
                sorted
            ));
        }
    }

    // Check arguments
    is_arguments_sorted_with_path(cmd, &current_path)?;

    // Recursively check subcommands
    for subcmd in cmd.get_subcommands() {
        is_sorted_with_path(subcmd, current_path.clone())?;
    }

    Ok(())
}

/// Internal function to assert arguments are sorted.
fn assert_arguments_sorted_with_path(cmd: &clap::Command, path: &[&str]) {
    if let Err(msg) = is_arguments_sorted_with_path(cmd, path) {
        panic!("{}", msg);
    }
}

/// Checks if arguments are sorted correctly, returning a Result.
fn is_arguments_sorted_with_path(cmd: &clap::Command, path: &[&str]) -> Result<(), String> {
    let args: Vec<_> = cmd.get_arguments().collect();

    let mut positional = Vec::new();
    let mut with_short = Vec::new();
    let mut long_only = Vec::new();

    for arg in &args {
        if arg.is_positional() {
            positional.push(*arg);
        } else if arg.get_short().is_some() {
            with_short.push(*arg);
        } else if arg.get_long().is_some() {
            long_only.push(*arg);
        }
    }

    // Note: We don't check if positional args are sorted - their order matters for parsing

    // Check short flags are sorted by short option
    let with_short_shorts: Vec<char> = with_short
        .iter()
        .filter_map(|a| a.get_short())
        .collect();
    let mut sorted_shorts = with_short_shorts.clone();
    sorted_shorts.sort_by(|a, b| {
        let a_lower = a.to_ascii_lowercase();
        let b_lower = b.to_ascii_lowercase();
        match a_lower.cmp(&b_lower) {
            std::cmp::Ordering::Equal => {
                // Lowercase before uppercase for same letter
                if a.is_lowercase() && b.is_uppercase() {
                    std::cmp::Ordering::Less
                } else if a.is_uppercase() && b.is_lowercase() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            other => other,
        }
    });

    if with_short_shorts != sorted_shorts {
        let current: Vec<String> = with_short
            .iter()
            .map(|a| format!("-{}", a.get_short().unwrap()))
            .collect();
        let mut sorted_args = with_short.clone();
        sorted_args.sort_by(|a, b| {
            let a_char = a.get_short().unwrap();
            let b_char = b.get_short().unwrap();
            let a_lower = a_char.to_ascii_lowercase();
            let b_lower = b_char.to_ascii_lowercase();
            match a_lower.cmp(&b_lower) {
                std::cmp::Ordering::Equal => {
                    if a_char.is_lowercase() && b_char.is_uppercase() {
                        std::cmp::Ordering::Less
                    } else if a_char.is_uppercase() && b_char.is_lowercase() {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }
                other => other,
            }
        });
        let expected: Vec<String> = sorted_args
            .iter()
            .map(|a| format!("-{}", a.get_short().unwrap()))
            .collect();

        return Err(format!(
            "Flags with short options in '{}' are not sorted!\nActual: {:?}\nExpected: {:?}",
            path.join(" "),
            current,
            expected
        ));
    }

    // Check long-only flags are sorted
    let long_only_longs: Vec<&str> = long_only
        .iter()
        .filter_map(|a| a.get_long())
        .collect();
    let mut sorted_longs = long_only_longs.clone();
    sorted_longs.sort_unstable();

    if long_only_longs != sorted_longs {
        let current: Vec<String> = long_only_longs
            .iter()
            .map(|l| format!("--{}", l))
            .collect();
        let expected: Vec<String> = sorted_longs.iter().map(|l| format!("--{}", l)).collect();

        return Err(format!(
            "Long-only flags in '{}' are not sorted!\nActual: {:?}\nExpected: {:?}",
            path.join(" "),
            current,
            expected
        ));
    }

    // Check that groups appear in correct order
    let arg_ids: Vec<&str> = args.iter().map(|a| a.get_id().as_str()).collect();

    let mut expected_order = Vec::new();
    expected_order.extend(positional.iter().map(|a| a.get_id().as_str()));
    expected_order.extend(with_short.iter().map(|a| a.get_id().as_str()));
    expected_order.extend(long_only.iter().map(|a| a.get_id().as_str()));

    if arg_ids != expected_order {
        return Err(format!(
            "Arguments in '{}' are not in correct group order!\nExpected: [positional, short flags, long-only flags]\nActual: {:?}\nExpected: {:?}",
            path.join(" "),
            arg_ids,
            expected_order
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Command, CommandFactory, Parser, Subcommand};

    #[test]
    fn test_sorted_subcommands() {
        let cmd = Command::new("test")
            .subcommand(Command::new("add"))
            .subcommand(Command::new("delete"))
            .subcommand(Command::new("list"));

        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "are not sorted alphabetically")]
    fn test_unsorted_subcommands() {
        let cmd = Command::new("test")
            .subcommand(Command::new("list"))
            .subcommand(Command::new("add"))
            .subcommand(Command::new("delete"));

        assert_sorted(&cmd);
    }

    #[test]
    fn test_is_sorted_ok() {
        let cmd = Command::new("test")
            .subcommand(Command::new("add"))
            .subcommand(Command::new("delete"))
            .subcommand(Command::new("list"));

        assert!(is_sorted(&cmd).is_ok());
    }

    #[test]
    fn test_is_sorted_err() {
        let cmd = Command::new("test")
            .subcommand(Command::new("list"))
            .subcommand(Command::new("add"));

        assert!(is_sorted(&cmd).is_err());
    }

    #[test]
    fn test_no_subcommands() {
        let cmd = Command::new("test");
        assert_sorted(&cmd);
        assert!(is_sorted(&cmd).is_ok());
    }

    #[test]
    fn test_with_derive_sorted() {
        #[derive(Parser)]
        struct Cli {
            #[command(subcommand)]
            command: Commands,
        }

        #[derive(Subcommand)]
        enum Commands {
            Add,
            Delete,
            List,
        }

        let cmd = Cli::command();
        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "are not sorted alphabetically")]
    fn test_with_derive_unsorted() {
        #[derive(Parser)]
        struct Cli {
            #[command(subcommand)]
            command: Commands,
        }

        #[derive(Subcommand)]
        enum Commands {
            List,
            Add,
            Delete,
        }

        let cmd = Cli::command();
        assert_sorted(&cmd);
    }

    // Tests for argument sorting

    #[test]
    fn test_arguments_correctly_sorted() {
        use clap::{Arg, ArgAction};

        let cmd = Command::new("test")
            .arg(Arg::new("file")) // Positional
            .arg(Arg::new("debug").short('d').long("debug").action(ArgAction::SetTrue))
            .arg(Arg::new("output").short('o').long("output"))
            .arg(Arg::new("verbose").short('v').long("verbose").action(ArgAction::SetTrue))
            .arg(Arg::new("config").long("config"))
            .arg(Arg::new("no-color").long("no-color").action(ArgAction::SetTrue));

        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "Flags with short options")]
    fn test_short_flags_unsorted() {
        use clap::Arg;

        let cmd = Command::new("test")
            .arg(Arg::new("verbose").short('v').long("verbose"))
            .arg(Arg::new("debug").short('d').long("debug"));

        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "Long-only flags")]
    fn test_long_only_unsorted() {
        use clap::{Arg, ArgAction};

        let cmd = Command::new("test")
            .arg(Arg::new("zebra").long("zebra").action(ArgAction::SetTrue))
            .arg(Arg::new("alpha").long("alpha").action(ArgAction::SetTrue));

        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "not in correct group order")]
    fn test_wrong_group_order() {
        use clap::Arg;

        // Long-only flag before short flag
        let cmd = Command::new("test")
            .arg(Arg::new("config").long("config"))
            .arg(Arg::new("verbose").short('v').long("verbose"));

        assert_sorted(&cmd);
    }

    #[test]
    fn test_positional_order_not_enforced() {
        // Positional arguments can be in any order since their order matters for parsing
        let cmd = Command::new("test")
            .arg(clap::Arg::new("second"))
            .arg(clap::Arg::new("first"));

        assert_sorted(&cmd);
    }

    #[test]
    fn test_is_sorted_ok_with_args() {
        use clap::Arg;

        let cmd = Command::new("test")
            .arg(Arg::new("file"))
            .arg(Arg::new("output").short('o').long("output"))
            .arg(Arg::new("config").long("config"))
            .subcommand(Command::new("add"))
            .subcommand(Command::new("delete"));

        assert!(is_sorted(&cmd).is_ok());
    }

    #[test]
    fn test_is_sorted_err_args() {
        use clap::Arg;

        let cmd = Command::new("test")
            .arg(Arg::new("zebra").short('z').long("zebra"))
            .arg(Arg::new("alpha").short('a').long("alpha"));

        assert!(is_sorted(&cmd).is_err());
    }

    #[test]
    fn test_recursive_subcommand_args() {
        use clap::{Arg, ArgAction};

        let cmd = Command::new("test")
            .arg(Arg::new("verbose").short('v').long("verbose").action(ArgAction::SetTrue))
            .subcommand(
                Command::new("sub")
                    .arg(Arg::new("debug").short('d').long("debug").action(ArgAction::SetTrue))
                    .arg(Arg::new("output").short('o').long("output")),
            );

        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "Flags with short options")]
    fn test_recursive_subcommand_args_fails() {
        use clap::Arg;

        let cmd = Command::new("test")
            .subcommand(
                Command::new("sub")
                    .arg(Arg::new("output").short('o').long("output"))
                    .arg(Arg::new("debug").short('d').long("debug")),
            );

        assert_sorted(&cmd);
    }

    #[test]
    fn test_global_flags_not_checked_in_subcommands() {
        use clap::{Arg, ArgAction};

        // Global flag 'v' should not interfere with subcommand's 'd' flag ordering
        let cmd = Command::new("test")
            .arg(Arg::new("verbose").short('v').long("verbose").global(true).action(ArgAction::SetTrue))
            .subcommand(
                Command::new("sub")
                    .arg(Arg::new("debug").short('d').long("debug").action(ArgAction::SetTrue))
                    .arg(Arg::new("output").short('o').long("output")),
            );

        // Should not panic - we only check non-global args in subcommands
        assert_sorted(&cmd);
    }

    #[test]
    fn test_global_flags_dont_appear_in_subcommand_args() {
        use clap::{Arg, ArgAction};

        // Verify that global flags don't appear in subcommand's get_arguments()
        // This confirms there's no bug with global flags interfering with sorting
        let cmd = Command::new("test")
            .arg(Arg::new("verbose").short('v').long("verbose").global(true).action(ArgAction::SetTrue))
            .subcommand(
                Command::new("sub")
                    .arg(Arg::new("debug").short('d').long("debug").action(ArgAction::SetTrue))
                    .arg(Arg::new("output").short('o').long("output")),
            );

        let subcmd = cmd.find_subcommand("sub").unwrap();
        let args: Vec<_> = subcmd.get_arguments().collect();

        // Subcommand should only see its own 2 args, not the global flag
        assert_eq!(args.len(), 2);

        // Verify neither arg is global
        for arg in &args {
            assert!(!arg.is_global_set(), "Subcommand arg {} should not be global", arg.get_id());
        }

        // Should pass - subcommand args are sorted and global flag doesn't interfere
        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "Flags with short options")]
    fn test_uppercase_before_lowercase_same_letter() {
        use clap::Arg;

        // Uppercase I before lowercase i - should fail
        let cmd = Command::new("test")
            .arg(Arg::new("index").short('I').long("index"))
            .arg(Arg::new("inject").short('i').long("inject"));

        assert_sorted(&cmd);
    }

    #[test]
    fn test_lowercase_before_uppercase_same_letter() {
        use clap::Arg;

        // Lowercase i before uppercase I - should pass
        let cmd = Command::new("test")
            .arg(Arg::new("inject").short('i').long("inject"))
            .arg(Arg::new("index").short('I').long("index"));

        assert_sorted(&cmd);
    }

    #[test]
    #[should_panic(expected = "Flags with short options")]
    fn test_task_docs_flags_unsorted() {
        use clap::Arg;

        // Reproduces the task_docs issue: -I before -i
        let cmd = Command::new("generate")
            .subcommand(
                Command::new("task-docs")
                    .arg(Arg::new("index").short('I').long("index"))
                    .arg(Arg::new("inject").short('i').long("inject"))
                    .arg(Arg::new("multi").short('m').long("multi"))
                    .arg(Arg::new("output").short('o').long("output"))
                    .arg(Arg::new("root").short('r').long("root"))
                    .arg(Arg::new("style").short('s').long("style")),
            );

        assert_sorted(&cmd);
    }

    #[test]
    fn test_error_message_shows_full_command_path() {
        use clap::Arg;

        // Test that error message shows the full command path
        let cmd = Command::new("parent-has-no-flags")
            .subcommand(
                Command::new("child-has-unsorted-flags")
                    .arg(Arg::new("zebra").short('z').long("zebra"))
                    .arg(Arg::new("alpha").short('a').long("alpha")),
            );

        let result = is_sorted(&cmd);
        assert!(result.is_err());
        let err = result.unwrap_err();

        // Should show full path: 'parent-has-no-flags child-has-unsorted-flags'
        assert!(err.contains("parent-has-no-flags child-has-unsorted-flags"),
            "Error message should contain full path, got: {}", err);
    }

    #[test]
    fn test_error_with_derive_api_nested_subcommands() {
        use clap::{Args, Parser, Subcommand};

        #[derive(Parser)]
        struct Cli {
            #[command(subcommand)]
            command: Commands,
        }

        #[derive(Subcommand)]
        enum Commands {
            /// Generate command
            Generate(GenerateArgs),
        }

        #[derive(Args)]
        struct GenerateArgs {
            #[command(subcommand)]
            command: GenerateCommands,
        }

        #[derive(Subcommand)]
        enum GenerateCommands {
            /// Task docs subcommand
            TaskDocs(TaskDocsArgs),
        }

        #[derive(Args)]
        struct TaskDocsArgs {
            /// task flag
            #[arg(short, long)]
            task: Option<String>,

            /// output flag
            #[arg(short, long)]
            output: Option<String>,
        }

        let cmd = Cli::command();
        let result = is_sorted(&cmd);

        if let Err(e) = result {
            // The error should show the full path
            eprintln!("Error message: {}", e);
            // Since the root command is the Cli, and we have Generate, then TaskDocs,
            // the full path would be something like "Cli Generate TaskDocs" but clap
            // converts names to kebab-case, so it would be "cli generate task-docs"
            // Actually, let me just check it contains both generate and task-docs
            assert!(e.contains("task-docs"),
                "Error should mention 'task-docs'. Got: {}", e);
            assert!(e.contains("[\"-t\", \"-o\"]"),
                "Error should show the actual unsorted flags. Got: {}", e);
        } else {
            panic!("Expected error for unsorted flags");
        }
    }
}
