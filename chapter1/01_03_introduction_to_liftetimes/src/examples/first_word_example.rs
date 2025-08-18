fn main() {
    let string = String::from("hello world from rust");
    let result;
    {
        // string exists here, so the reference to it is valid
        result = first_word(&string);
    }
    println!("The first word is: {}", result);
    
    // Additional examples
    let text: &'static str = "hello world from rust";
    let first: &str = first_word(text);
    println!("First word: '{}'", first);
    
    let single_word: &'static str = "hello";
    let result: &str = first_word(single_word);
    println!("Single word: '{}'", result);
}

// The 'a tells Rust that the returned reference will live at least as long
// as the reference passed in
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
   
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
   
    &s[..]
}
