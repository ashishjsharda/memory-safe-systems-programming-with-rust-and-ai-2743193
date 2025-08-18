// This struct holds a reference to a string slice
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
   
    let excerpt = Excerpt {
        part: first_sentence,
    };
   
    println!("Excerpt: {}", excerpt.part);
    
    // excerpt can't outlive the data it references
    {
        let temp_string = String::from("This is temporary text.");
        let temp_excerpt = Excerpt {
            part: &temp_string,
        };
        println!("Temporary excerpt: {}", temp_excerpt.part);
        // temp_excerpt goes out of scope here, along with temp_string
    }
    
    // This would not compile if we tried to use temp_excerpt here
    // because temp_string would be dropped
}

