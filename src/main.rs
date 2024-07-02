use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check for "--help" option
    if args.contains(&"--help".to_string()) {
        print_help();
        return Ok(());
    }

    let exclude_dir = get_exclude_dir(&args);

    let mut output = String::new();

    // Output the directory tree
    output.push_str("## Directory Tree\n\n");
    print_directory_tree(".", &mut output, 0, exclude_dir.as_deref())?;

    // Output the content of each file within code blocks
    output.push_str("\n## File Contents\n\n");
    for entry in walkdir::WalkDir::new(".") {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();

            if let Some(exclude_dir) = &exclude_dir {
                if path.starts_with(exclude_dir) {
                    continue; // Skip processing if path starts with the excluded directory
                }
            }

            let file_name = path.file_name().unwrap().to_str().unwrap();

            output.push_str(&format!("### {}\n\n", file_name));
            output.push_str(&format!("```{}\n", get_extension(path).unwrap_or("")));

            match fs::read_to_string(path) {
                Ok(content) => output.push_str(&content),
                Err(e) => {
                    eprintln!("Error reading file {}: {}", path.display(), e);
                }
            }
            output.push_str("\n```\n\n");
        }
    }

    fs::write("all_files.txt", output)?;

    Ok(())
}

fn get_exclude_dir(args: &[String]) -> Option<String> {
    // Find the index of the "--exclude" option
    let exclude_index = args.iter().position(|arg| arg == "--exclude");

    // If "--exclude" is found, return the next argument as the directory to exclude
    exclude_index.and_then(|index| args.get(index + 1).cloned())
}

fn print_directory_tree(
    dir: &str,
    output: &mut String,
    level: usize,
    exclude_dir: Option<&str>,
) -> io::Result<()> {
    let indent = "  ".repeat(level);
    output.push_str(&format!("{}{}/\n", indent, dir));

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(exclude_dir) = exclude_dir {
            if path.starts_with(exclude_dir) {
                continue; // Skip processing if path starts with the excluded directory
            }
        }

        if path.is_dir() {
            print_directory_tree(path.to_str().unwrap(), output, level + 1, exclude_dir)?;
        } else {
            output.push_str(&format!("{}  {}\n", indent, path.file_name().unwrap().to_str().unwrap()));
        }
    }

    Ok(())
}

fn get_extension(path: &Path) -> Option<&str> {
    path.extension().and_then(|ext| ext.to_str())
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run [--exclude <directory>] [--help]");
    println!("");
    println!("Options:");
    println!("  --exclude <directory>  Exclude a directory from processing.");
    println!("  --help                  Show this help message.");
}