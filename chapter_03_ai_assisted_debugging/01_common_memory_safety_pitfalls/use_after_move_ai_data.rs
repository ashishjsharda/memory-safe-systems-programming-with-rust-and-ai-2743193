fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ai_model_data = vec![1.0, 2.0, 3.0, 4.0]; // Our AI model weights
    
    // This looks innocent enough...
    process_ai_data(ai_model_data);
    
    // But now this will cause a compile error!
    // println!("Model data: {:?}", ai_model_data);
    // Error: value borrowed here after move
    
    Ok(())
}

fn process_ai_data(data: Vec<f64>) {
    println!("Processing {} data points", data.len());
    // Data is moved here and dropped when function ends
}
