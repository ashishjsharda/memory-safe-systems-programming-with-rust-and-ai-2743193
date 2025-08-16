fn calculate_average(numbers: &[i32]) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0;
   
    // ISSUE: Average is always wrong - Copilot, what am I missing?
    for num in numbers {
        sum += *num as f64;
        count += 1.0;
    }
   
    // BUG: What happens when the vector is empty?
    sum / count  // This will return NaN for empty vectors!
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let avg = calculate_average(&data);
    println!("Average: {}", avg); // Works fine: 3.0
   
    // But this reveals the bug:
    let empty_data = vec![];
    let empty_avg = calculate_average(&empty_data);
    println!("Empty average: {}", empty_avg); // Prints NaN!
}
