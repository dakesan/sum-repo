use std::env;
use std::fs;
use std::io;
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--help".to_string()) {
        print_help();
        return Ok(());
    }

    let exclude_dir = get_exclude_dir(&args);
    let ignore_patterns = get_ignore_patterns();

    let mut output = String::new();

    output.push_str("## Directory Tree\n\n");
    print_directory_tree(".", &mut output, 0, exclude_dir.as_deref(), &ignore_patterns)?;

    output.push_str("\n## File Contents\n\n");
    for entry in walkdir::WalkDir::new(".") {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();

            if should_ignore(path, exclude_dir.as_deref(), &ignore_patterns) {
                continue;
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
    let exclude_index = args.iter().position(|arg| arg == "--exclude");
    exclude_index.and_then(|index| args.get(index + 1).cloned())
}

fn get_ignore_patterns() -> Vec<Regex> {
    vec![
        Regex::new(r"\.pyc$").unwrap(),
        Regex::new(r"\.pyo$").unwrap(),
        Regex::new(r"\.pyd$").unwrap(),
        Regex::new(r"__pycache__").unwrap(),
        Regex::new(r"\.pytest_cache").unwrap(),
        Regex::new(r"\.mypy_cache").unwrap(),
        Regex::new(r"\.ipynb_checkpoints").unwrap(),
        Regex::new(r"venv").unwrap(),
        Regex::new(r"\.venv").unwrap(),
        Regex::new(r"\.env").unwrap(),
        Regex::new(r"\.idea").unwrap(),
        Regex::new(r"\.vscode").unwrap(),
        Regex::new(r"\.DS_Store").unwrap(),
        Regex::new(r"\.git").unwrap(),
    ]
}

fn should_ignore(path: &Path, exclude_dir: Option<&str>, ignore_patterns: &[Regex]) -> bool {
    if let Some(exclude_dir) = exclude_dir {
        if path.starts_with(exclude_dir) {
            return true;
        }
    }

    let path_str = path.to_str().unwrap_or("");
    ignore_patterns.iter().any(|pattern| pattern.is_match(path_str))
}

fn print_directory_tree(
    dir: &str,
    output: &mut String,
    level: usize,
    exclude_dir: Option<&str>,
    ignore_patterns: &[Regex],
) -> io::Result<()> {
    let indent = "  ".repeat(level);
    output.push_str(&format!("{}{}/\n", indent, dir));

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if should_ignore(&path, exclude_dir, ignore_patterns) {
            continue;
        }

        if path.is_dir() {
            print_directory_tree(path.to_str().unwrap(), output, level + 1, exclude_dir, ignore_patterns)?;
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
    println!("  --help                 Show this help message.");
}