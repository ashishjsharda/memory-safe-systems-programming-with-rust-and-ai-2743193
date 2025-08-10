use std::fs;
use std::io;


fn main() {
    let filenames = ["sample.txt", "backup.txt", "default.txt"];
   
    let mut success = false;
   
    for filename in &filenames {
        match read_and_process_file(filename) {
            Ok(content) => {
                println!("Successfully read: {}", filename);
                display_file_stats(&content);
                success = true;
                break;
            }
            Err(e) => {
                eprintln!("Failed to read {}: {}", filename, e);
            }
        }
    }
   
    if !success {
        eprintln!("Could not read any of the specified files!");
    }
   
    // Demonstrate memory safety concepts
    demonstrate_safety();
}


fn read_and_process_file(filename: &str) -> Result<String, io::Error> {
    // Read the entire file into a String
    let contents = fs::read_to_string(filename)?;
   
    // Process the content (remove extra whitespace)
    let processed = contents
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("\n");
   
    Ok(processed)
}


fn display_file_stats(content: &String) {
    let line_count = content.lines().count();
    let word_count = content
        .split_whitespace()
        .count();
    let char_count = content.chars().count();
   
    println!("File Statistics:");
    println!("  Lines: {}", line_count);
    println!("  Words: {}", word_count);
    println!("  Characters: {}", char_count);
   
    // Let's also find the longest line
    if let Some(longest_line) = find_longest_line(content) {
        println!("  Longest line: {} characters", longest_line.len());
    }
}


fn find_longest_line(content: &String) -> Option<&str> {
    content
        .lines()
        .max_by_key(|line| line.len())
}


fn demonstrate_safety() {
    let content = String::from("Hello, Rust!");
   
    // This borrows content immutably
    let _stats_result = analyze_content(&content);
   
    // content is still available here because we only borrowed it
    println!("Original content: {}", content);
   
    // Multiple immutable borrows are allowed
    let word_count = count_words(&content);
    let char_count = count_chars(&content);
   
    println!("Words: {}, Chars: {}", word_count, char_count);
   
    // content is automatically cleaned up when it goes out of scope
}


fn analyze_content(text: &str) -> usize {
    text.len()
    // No cleanup needed - we don't own the data
}


fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}


fn count_chars(text: &str) -> usize {
    text.chars().count()
}


