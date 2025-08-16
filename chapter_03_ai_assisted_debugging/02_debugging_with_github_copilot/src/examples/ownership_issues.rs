fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
   
    // COPILOT: This won't compile - help me fix the borrowing issue
    let first = &data[0];           // Immutable borrow
    data.push(6);                   // Mutable borrow - conflict!
    println!("First element: {}", first);
}
