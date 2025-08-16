fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
   
    // BUG: This will panic! Can you spot why?
    // COPILOT: Help me understand what's wrong here
    for i in 0..=numbers.len() {
        println!("Number {}: {}", i, numbers[i]);
    }
}
