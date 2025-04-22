#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;
    use crate::command_args::Args;
    use crate::io_operations::StdinOperations;
    use crate::{process_file, process_stdin};
    use mockall::predicate::*;
    use mockall::*;

    // Create mock for StdinOperations
    mock! {
        StdinReader {}
        impl StdinOperations for StdinReader {
            fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize>;
        }
    }

    // Helper function to create a test file
    fn create_test_file() -> (NamedTempFile, String) {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "Hello, world!\nThis is a test file.\nThird line.\n";
        write!(temp_file, "{}", content).unwrap();
        (temp_file, content.to_string())
    }

    // PART 1: Tests for process_file function
    #[test]
    fn test_process_file_bytes() {
        let (temp_file, content) = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        
        let args = Args {
            bytes: true,
            lines: false,
            words: false,
            chars: false,
            file: Some(file_path.to_string()),
        };
        
        let output = process_file(&args, file_path, false);
        
        assert!(output.contains(&format!("{:8}", content.len())));
        assert!(output.contains(file_path));
    }

    #[test]
    fn test_process_file_lines() {
        let (temp_file, content) = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        let line_count = content.lines().count();
        
        let args = Args {
            bytes: false,
            lines: true,
            words: false,
            chars: false,
            file: Some(file_path.to_string()),
        };
        
        let output = process_file(&args, file_path, false);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(file_path));
    }

    #[test]
    fn test_process_file_words() {
        let (temp_file, content) = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        let word_count = content.split_whitespace().count();
        
        let args = Args {
            bytes: false,
            lines: false,
            words: true,
            chars: false,
            file: Some(file_path.to_string()),
        };
        
        let output = process_file(&args, file_path, false);
        
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(file_path));
    }

    #[test]
    fn test_process_file_chars() {
        let (temp_file, content) = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        let char_count = content.chars().count();
        
        let args = Args {
            bytes: false,
            lines: false,
            words: false,
            chars: true,
            file: Some(file_path.to_string()),
        };
        
        let output = process_file(&args, file_path, false);
        
        assert!(output.contains(&format!("{:8}", char_count)));
        assert!(output.contains(file_path));
    }

    #[test]
    fn test_process_file_default_mode() {
        let (temp_file, content) = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        let line_count = content.lines().count();
        let word_count = content.split_whitespace().count();
        let byte_count = content.len();
        
        let args = Args {
            bytes: false,
            lines: false,
            words: false,
            chars: false,
            file: Some(file_path.to_string()),
        };
        
        let output = process_file(&args, file_path, true);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        assert!(output.contains(file_path));
    }

    #[test]
    fn test_process_file_multiple_flags() {
        let (temp_file, content) = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        let line_count = content.lines().count();
        let byte_count = content.len();
        
        let args = Args {
            bytes: true,
            lines: true,
            words: false,
            chars: false,
            file: Some(file_path.to_string()),
        };
        
        let output = process_file(&args, file_path, false);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        assert!(output.contains(file_path));
    }
    
    #[test]
    fn test_process_file_nonexistent() {
        let file_path = "nonexistent_file.txt";
        
        let args = Args {
            bytes: false,
            lines: false, 
            words: false,
            chars: false,
            file: Some(file_path.to_string()),
        };
        
        let output = process_file(&args, file_path, true);
        
        assert!(output.contains(&format!("Could not read file: {}", file_path)));
    }
    
    #[test]
    fn test_process_file_all_flags() {
        let (temp_file, content) = create_test_file();
        let file_path = temp_file.path().to_str().unwrap();
        let line_count = content.lines().count();
        let word_count = content.split_whitespace().count();
        let byte_count = content.len();
        let char_count = content.chars().count();
        
        let args = Args {
            bytes: true,
            lines: true,
            words: true,
            chars: true,
            file: Some(file_path.to_string()),
        };
        
        let output = process_file(&args, file_path, false);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        assert!(output.contains(&format!("{:8}", char_count)));
        assert!(output.contains(file_path));
    }
    
    // PART 2: Tests for process_stdin function using mockall

    // Test default mode (no flags set)
    #[test]
    fn test_stdin_default_mode() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let line_count = test_content.lines().count();
        let word_count = test_content.split_whitespace().count();
        let byte_count = test_content.len();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: false,
            lines: false,
            words: false,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, true, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
    }
    
    // Test lines flag only
    #[test]
    fn test_stdin_lines_only() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let line_count = test_content.lines().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: false,
            lines: true,
            words: false,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        // Just check that the output has the expected count, don't check exact format
        assert_eq!(output.trim(), format!("{:8}", line_count).trim());
    }
    
    // Test words flag only
    #[test]
    fn test_stdin_words_only() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let word_count = test_content.split_whitespace().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: false,
            lines: false,
            words: true,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", word_count)));
        // Just check that the output has the expected count, don't check exact format
        assert_eq!(output.trim(), format!("{:8}", word_count).trim());
    }
    
    // Test bytes flag only
    #[test]
    fn test_stdin_bytes_only() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let byte_count = test_content.len();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: true,
            lines: false,
            words: false,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", byte_count)));
        // Just check that the output has the expected count, don't check exact format
        assert_eq!(output.trim(), format!("{:8}", byte_count).trim());
    }
    
    // Test chars flag only
    #[test]
    fn test_stdin_chars_only() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let char_count = test_content.chars().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: false,
            lines: false,
            words: false,
            chars: true,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", char_count)));
        // Just check that the output has the expected count, don't check exact format
        assert_eq!(output.trim(), format!("{:8}", char_count).trim());
    }
    
    // Test all flags together
    #[test]
    fn test_stdin_all_flags() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let line_count = test_content.lines().count();
        let word_count = test_content.split_whitespace().count();
        let byte_count = test_content.len();
        let char_count = test_content.chars().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: true,
            lines: true,
            words: true,
            chars: true,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        assert!(output.contains(&format!("{:8}", char_count)));
    }

    // Test with various flag combinations
    
    // Lines + Words
    #[test]
    fn test_stdin_lines_and_words() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let line_count = test_content.lines().count();
        let word_count = test_content.split_whitespace().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: false,
            lines: true,
            words: true,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        // Check content but not exact format
        let expected = format!("{:8}{:8}", line_count, word_count);
        assert_eq!(output.replace(" ", ""), expected.replace(" ", ""));
    }
    
    // Lines + Bytes
    #[test]
    fn test_stdin_lines_and_bytes() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let line_count = test_content.lines().count();
        let byte_count = test_content.len();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: true,
            lines: true,
            words: false,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        // Check content but not exact format
        let expected = format!("{:8}{:8}", line_count, byte_count);
        assert_eq!(output.replace(" ", ""), expected.replace(" ", ""));
    }
    
    // Words + Bytes
    #[test]
    fn test_stdin_words_and_bytes() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let word_count = test_content.split_whitespace().count();
        let byte_count = test_content.len();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: true,
            lines: false,
            words: true,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        // Check content but not exact format
        let expected = format!("{:8}{:8}", word_count, byte_count);
        assert_eq!(output.replace(" ", ""), expected.replace(" ", ""));
    }
    
    // Lines + Words + Bytes
    #[test]
    fn test_stdin_lines_words_and_bytes() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let line_count = test_content.lines().count();
        let word_count = test_content.split_whitespace().count();
        let byte_count = test_content.len();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: true,
            lines: true,
            words: true,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        // Check content but not exact format
        let expected = format!("{:8}{:8}{:8}", line_count, word_count, byte_count);
        assert_eq!(output.replace(" ", ""), expected.replace(" ", ""));
    }
    
    // Test with empty input
    #[test]
    fn test_stdin_empty_input() {
        let test_content = "";
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(0)
            });
        
        let args = Args {
            bytes: true,
            lines: true,
            words: true,
            chars: true,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", 0))); // lines
        assert!(output.contains(&format!("{:8}", 0))); // words
        assert!(output.contains(&format!("{:8}", 0))); // bytes
        assert!(output.contains(&format!("{:8}", 0))); // chars
    }
    
    // Test with Unicode content to verify character count
    #[test]
    fn test_stdin_unicode_content() {
        let test_content = "Hello, 世界!\nThis is a 测试 file.\nThird 行.\n";
        let line_count = test_content.lines().count();
        let word_count = test_content.split_whitespace().count();
        let byte_count = test_content.len();
        let char_count = test_content.chars().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: true,
            lines: true,
            words: true,
            chars: true,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        assert!(output.contains(&format!("{:8}", char_count)));
        
        // For Unicode, byte count and char count should differ
        assert_ne!(byte_count, char_count);
    }
    
    // Test with multiline input
    #[test]
    fn test_stdin_multiline_input() {
        let test_content = "First line\nSecond line\nThird line\nFourth line\nFifth line\n";
        let line_count = test_content.lines().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: false,
            lines: true,
            words: false,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        // Just check that the output has the expected count, don't check exact format
        assert_eq!(output.trim(), format!("{:8}", line_count).trim());
        assert_eq!(line_count, 5);
    }
    
    // Test with input that has no newlines
    #[test]
    fn test_stdin_no_newlines() {
        let test_content = "This is a single line with no newline characters";
        let line_count = test_content.lines().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: false,
            lines: true,
            words: false,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        // Just check that the output has the expected count, don't check exact format
        assert_eq!(output.trim(), format!("{:8}", line_count).trim());
        assert_eq!(line_count, 1); // Should be 1 line
    }
    
    // Test with large input
    #[test]
    fn test_stdin_large_input() {
        // Generate a large content string
        let mut test_content = String::new();
        for i in 0..1000 {
            test_content.push_str(&format!("Line {} with some words for testing\n", i));
        }
        
        let line_count = test_content.lines().count();
        let word_count = test_content.split_whitespace().count();
        let byte_count = test_content.len();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        let args = Args {
            bytes: true,
            lines: true,
            words: true,
            chars: false,
            file: None,
        };
        
        let output = process_stdin(&args, false, mock);
        
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        assert_eq!(line_count, 1000);
    }
    
    // Test read error handling
    #[test]
    #[should_panic]
    fn test_stdin_read_error() {
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(|_| {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Test error"))
            });
        
        let args = Args {
            bytes: true,
            lines: false,
            words: false,
            chars: false,
            file: None,
        };
        
        // This should panic due to the unwrap() on the error result
        process_stdin(&args, false, mock);
    }
    
    // Test with a mixture of flags and default mode
    #[test]
    fn test_stdin_mixed_flags_with_default_mode() {
        let test_content = "Hello, world!\nThis is a test file.\nThird line.\n";
        let line_count = test_content.lines().count();
        let word_count = test_content.split_whitespace().count();
        let byte_count = test_content.len();
        let char_count = test_content.chars().count();
        
        let mut mock = MockStdinReader::new();
        mock.expect_read_to_end()
            .times(1)
            .returning(move |buf| {
                buf.extend_from_slice(test_content.as_bytes());
                Ok(test_content.len())
            });
        
        // Even though we have some flags set, we're passing default_mode=true
        // which should include lines, words, and bytes regardless
        let args = Args {
            bytes: false,
            lines: true,
            words: false,
            chars: true,
            file: None,
        };
        
        let output = process_stdin(&args, true, mock);
        
        // Should include lines (from flag), words (from default), bytes (from default), and chars (from flag)
        assert!(output.contains(&format!("{:8}", line_count)));
        assert!(output.contains(&format!("{:8}", word_count)));
        assert!(output.contains(&format!("{:8}", byte_count)));
        assert!(output.contains(&format!("{:8}", char_count)));
    }
}