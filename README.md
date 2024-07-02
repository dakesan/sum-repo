## Summarize repository

This Rust program generates a text file summarizing the contents of a repository. It outputs the directory tree and the content of each file within code blocks, making it easy to share and analyze the codebase.

このプログラムを使うことで簡単にレポジトリをまるっとAIチャットに投げることができます。

You can easily throw your entire repository into any AI services using this program.

### Features

- Outputs the directory tree of the repository.
- Includes the content of each file within code blocks, preserving syntax highlighting.
- Allows excluding specific directories using the `--exclude` option.

### Usage

1. **Compile the program:**
   ```bash
   cargo build
   ```

2. **Run the program:**
   ```bash
   cargo run [--exclude <directory>]
   ```

   - Replace `<directory>` with the actual directory you want to exclude.

   **Example:** To exclude the "target" directory:
   ```bash
   cargo run -- --exclude target
   ```

3. **Output:**
   The program generates a file named `all_files.txt` in the current directory, containing the repository summary.

### Options

- `--exclude <directory>`:  Exclude a directory from processing.
- `--help`:                  Show the help message.

### License

This project is licensed under the MIT License.

### Made by

Hiro Odake
https://binfo-hodake.com/