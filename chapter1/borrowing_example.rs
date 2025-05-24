fn main() {
    let s1 = String::from("hello");
   
    // We're borrowing s1 here, not transferring ownership
    let len = calculate_length(&s1);
   
    // s1 is still valid here!
    println!("The length of '{}' is {}.", s1, len);
}


fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope here, but since it's just a reference,
    // nothing happens to the original value
}
