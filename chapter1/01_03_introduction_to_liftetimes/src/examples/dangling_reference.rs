fn main() {
    // This would cause a compile error - demonstrating the problem lifetimes solve
    // let reference_to_nothing = dangle();
    
    // The correct way - return ownership instead of a reference
    let valid_string = no_dangle();
    println!("Valid string: {}", valid_string);
}

// This function would fail to compile due to missing lifetime specifier
// fn dangle() -> &String { // Error: missing lifetime specifier
//     let s = String::from("hello");
//     &s // We're returning a reference to s, which will be dropped when the function ends
// } // s goes out of scope and is dropped. Its memory is freed!

// The correct solution - return ownership, not a reference
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Return ownership of s to the calling function
}
