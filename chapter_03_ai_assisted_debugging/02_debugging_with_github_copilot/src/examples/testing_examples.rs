fn divide_numbers(a: f64, b: f64) -> f64 {
    a / b  // BUG: No check for division by zero
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_divide_numbers() {
        // COPILOT: What edge cases should I test for division?
        assert_eq!(divide_numbers(10.0, 2.0), 5.0);
       
        // Copilot will likely suggest testing:
        // - Division by zero
        // - Negative numbers  
        // - Very large/small numbers
        // - NaN and infinity cases
    }
}
