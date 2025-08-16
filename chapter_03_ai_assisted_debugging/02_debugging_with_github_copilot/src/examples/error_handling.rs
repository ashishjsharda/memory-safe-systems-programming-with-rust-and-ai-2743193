use std::num::ParseIntError;

fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut numbers = Vec::new();
   
    for line in input.lines() {
        // COPILOT: This error message isn't helpful - how can I improve it?
        let num = line.parse::<i32>()?;
        numbers.push(num);
    }
    Ok(numbers)
}

fn main() {
    let input = "10\n20\ninvalid\n30";
    match parse_numbers(input) {
        Ok(nums) => println!("Numbers: {:?}", nums),
        Err(e) => println!("Error: {}", e), // Vague error message
    }
}
