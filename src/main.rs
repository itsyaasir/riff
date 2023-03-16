mod changes;
mod levenshtein;
use std::path::Path;

use anyhow::anyhow;
use changes::Change;
use colored::Colorize;
use levenshtein::levenshtein_diff;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 3 {
        return Err(anyhow!("Usage: {} <file> <file>", args[0]));
    }

    let file_one = read_file(Path::new(&args[1]))?;
    let file_two = read_file(Path::new(&args[2]))?;

    let changes = levenshtein_diff(&file_one, &file_two)?;

    if changes.is_empty() {
        println!(
            "{} {}",
            args[0].red(),
            format!("{} {}", "Files are identical".green(), "✓",).green(),
        );
    }

    // TODO:: Improve the output of the program to show the changes in a more readable way
    for change in changes {
        match change {
            Change::Insertion(c, pos) => {
                println!(
                    "{} {}",
                    "Insertion".green(),
                    format!("'{}' at position {}", c, pos).green()
                );
            }
            Change::Deletion(c, pos) => {
                println!(
                    "{} {}",
                    "Deletion".red(),
                    format!("'{}' at position {}", c, pos).red()
                );
            }
            Change::Substitution(c1, c2, pos) => {
                println!(
                    "{} {}",
                    "Substitution".yellow(),
                    format!("'{}' with '{}' at position {}", c1, c2, pos).yellow()
                );
            }
        }
    }

    Ok(())
}

/// Simple function to read the files that have been passed through the arguments
fn read_file(file: &Path) -> anyhow::Result<String> {
    // TODO:: Improve the error handling of this function
    let file_contents = std::fs::read_to_string(file)?;

    Ok(file_contents)
}
