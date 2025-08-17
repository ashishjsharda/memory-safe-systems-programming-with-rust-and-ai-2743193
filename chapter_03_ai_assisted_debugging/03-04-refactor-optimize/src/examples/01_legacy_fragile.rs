// LEGACY CODE - Simple but fragile
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn count_words_in_files(folder: String) -> Result<String, String> {

    let entries = match fs::read_dir(&folder) {
        Ok(e) => e,
        Err(e) => return Err(format!("Failed to read directory '{}': {}", folder, e)),
    };
    let mut total = 0;

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to read directory entry: {}", e);
                continue;
            }
        };
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "txt" {
                let file = match File::open(&path) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed to open file '{}': {}", path.display(), e);
                        continue;
                    }
                };
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(l) => total += l.split_whitespace().count(),
                        Err(e) => {
                            eprintln!("Failed to read line in '{}': {}", path.display(), e);
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(format!("Total words: {}", total))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_dir(dir: &str, files: &[(&str, &str)]) {
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();
        for (name, content) in files {
            let file_path = format!("{}/{}", dir, name);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    }

    #[test]
    fn test_error_directory_not_exist() {
        let result = count_words_in_files("nonexistent_dir".to_string());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to read directory"));
    }

    #[test]
    fn test_successful_word_count_mixed_file_types() {
        let test_dir = "test_docs_mixed";
        setup_test_dir(
            test_dir,
            &[
                ("file1.txt", "hello world"),
                ("file2.txt", "rust is awesome"),
                ("file3.md", "should not count"),
                ("file4.txt", "one"),
                ("file5.rs", "skip me"),
            ],
        );
        let result = count_words_in_files(test_dir.to_string()).unwrap();
        // file1.txt: 2, file2.txt: 3, file4.txt: 1
        assert_eq!(result, "Total words: 6");
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_files_processed_and_skipped_correctly() {
        let test_dir = "test_docs_skip";
        setup_test_dir(
            test_dir,
            &[
                ("a.txt", "word"),
                ("b.md", "not counted"),
                ("c.txt", "two words"),
                ("d.log", "skip"),
            ],
        );
        let result = count_words_in_files(test_dir.to_string()).unwrap();
        // a.txt: 1, c.txt: 2
        assert_eq!(result, "Total words: 3");
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_empty_txt_files() {
        let test_dir = "test_docs_empty";
        setup_test_dir(
            test_dir,
            &[
                ("empty1.txt", ""),
                ("empty2.txt", ""),
                ("notempty.txt", "something here"),
            ],
        );
        let result = count_words_in_files(test_dir.to_string()).unwrap();
        // empty1.txt: 0, empty2.txt: 0, notempty.txt: 2
        assert_eq!(result, "Total words: 2");
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_no_txt_files() {
        let test_dir = "test_docs_no_txt";
        setup_test_dir(
            test_dir,
            &[
                ("file1.md", "markdown"),
                ("file2.rs", "rust"),
            ],
        );
        let result = count_words_in_files(test_dir.to_string()).unwrap();
        assert_eq!(result, "Total words: 0");
        let _ = fs::remove_dir_all(test_dir);
    }
}


fn main() {
    match count_words_in_files("./docs".to_string()) {
        Ok(result) => println!("{}", result),
        Err(e) =>{ eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    }
}
