mod changes;
mod levenshtein;
use std::path::Path;

use anyhow::anyhow;
use levenshtein::levenshtein_diff;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 3 {
        return Err(anyhow!("Usage: {} <file> <file>", args[0]));
    }

    let file_one = read_file(Path::new(&args[1]))?;
    let file_two = read_file(Path::new(&args[2]))?;

    let changes = levenshtein_diff(&file_one, &file_two);

    // Print the changes according to the colors
    // red for deletion
    // green for insertion

    Ok(())
}

/// Simple function to read the files that have been passed through the arguments
fn read_file(file: &Path) -> anyhow::Result<String> {
    let file_contents = std::fs::read_to_string(file)?;

    Ok(file_contents)
}
