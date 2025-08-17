use std::rc::Rc;

fn main() {
    // Create a reference-counted string
    let shared_data = Rc::new(String::from("Hello, shared world!"));
   
    // Clone the Rc - this creates a new reference, not a new string
    let reference1 = Rc::clone(&shared_data);
    let reference2 = Rc::clone(&shared_data);
   
    println!("Reference count: {}", Rc::strong_count(&shared_data));
   
    // All references point to the same data
    println!("Original: {}", shared_data);
    println!("Reference 1: {}", reference1);
    println!("Reference 2: {}", reference2);
   
    // When references go out of scope, the count decreases
    {
        let _temp_ref = Rc::clone(&shared_data);
        println!("Temporary reference count: {}", Rc::strong_count(&shared_data));
    } // temp_ref is dropped here
   
    println!("Final reference count: {}", Rc::strong_count(&shared_data));
}
