fn main() {
    let text = "hello world from rust";
    
    // Both functions work the same way due to lifetime elision
    let word1 = first_word_implicit(text);
    let word2 = first_word_explicit(text);
    
    println!("Implicit lifetimes: {}", word1);
    println!("Explicit lifetimes: {}", word2);
}

// These two function signatures are equivalent due to lifetime elision rules
// The compiler automatically infers the lifetime
fn first_word_implicit(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// This is what the compiler actually sees internally
fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Demonstration of the three lifetime elision rules:
// Rule 1: Each parameter that is a reference gets its own lifetime parameter
// Rule 2: If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters  
// Rule 3: If there are multiple input lifetime parameters, but one of them is &self or &mut self, 
//         the lifetime of self is assigned to all output lifetime parameters
