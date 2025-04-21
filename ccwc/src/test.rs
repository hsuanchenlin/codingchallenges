#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::io::Write;
    use std::fs::File;
    use tempfile::NamedTempFile;

    #[test]
    fn test_ccwc_with_file() {
        // Create a temporary file with known content
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "Hello, world!\nThis is a test file.\n";
        write!(temp_file, "{}", content).unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Run the ccwc command
        let output = Command::new("./target/debug/ccwc")
            .arg("-c")  // Count bytes
            .arg(file_path)
            .output()
            .expect("Failed to execute command");

        // Check the output
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(&format!("{:8}", content.len())));
        assert!(stdout.contains(file_path));
    }
}