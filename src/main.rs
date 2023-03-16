mod levenshtein;

use std::path::Path;

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 3 {
        return Err(anyhow!("Usage: {} <file> <file>", args[0]));
    }

    let file_one = read_file(Path::new(&args[1]))?;
    let file_two = read_file(Path::new(&args[2]))?;

    eprintln!("file contents : {file_two}");
    Ok(())
}

fn read_file(file: &Path) -> anyhow::Result<String> {
    // Read the file

    let file_contents = std::fs::read_to_string(file)?;

    Ok(file_contents)
}
