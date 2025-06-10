use serde::Deserialize;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};
use tracing::{info, error, instrument};

#[derive(Debug, Deserialize)]

struct SafeAIInput {
    #[serde(deserialize_with = "validate_text_length")]
    prompt: String,
    #[serde(deserialize_with = "validate_temperature")]
    temperature: f32,
}


fn validate_text_length<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>
{
    let text: String = String::deserialize(deserializer)?;
    if text.len() > 10_000 {
        return Err(serde::de::Error::custom("Input too long"));
    }
    // Additional sanitization logic here
    Ok(text)
}


fn validate_temperature<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>
{
    let temp: f32 = f32::deserialize(deserializer)?;
    if temp < 0.0 || temp > 2.0 {
        return Err(serde::de::Error::custom("Temperature must be between 0.0 and 2.0"));
    }
    Ok(temp)
}
#[derive(Error, Debug)]
enum AIWorkflowError {
    #[error("Network timeout after {timeout}ms")]
    NetworkTimeout { timeout: u64 },
    #[error("Invalid model response: {reason}")]
    InvalidResponse { reason: String },
}


async fn safe_ai_call(input: SafeAIInput) -> Result<String, AIWorkflowError> {
    // Multiple layers of safety checks and fallbacks
    let response = tokio::time::timeout(Duration::from_secs(30), call_ai_api(input))
        .await
        .map_err(|_| AIWorkflowError::NetworkTimeout { timeout: 30000 })??;
    validate_response(&response)?;
    Ok(response)
}
fn validate_response(response: &str) -> Result<(), AIWorkflowError> {
    if response.is_empty() {
        return Err(AIWorkflowError::InvalidResponse {
            reason: "Empty response".to_string(),
        });
    }
    Ok(())
}

struct AICircuitBreaker {
    _failure_count: AtomicU32,
    last_failure: AtomicU64,
    state: AtomicU8, // 0: Closed, 1: Open, 2: Half-Open
}
impl AICircuitBreaker {
    fn new() -> Self {
        Self {
            _failure_count: AtomicU32::new(0),
            last_failure: AtomicU64::new(0),
            state: AtomicU8::new(0),
        }
    }


    fn can_execute(&self) -> bool {
        match self.state.load(Ordering::Relaxed) {
            0 => true, // Circuit closed, allow execution
            1 => {     // Circuit open, check if we should try again
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                now - self.last_failure.load(Ordering::Relaxed) > 60
            }
            _ => true, // Half-open, allow limited execution
        }
    }
}

fn hash_input(input: &SafeAIInput) -> String {
    format!("{:x}", input.prompt.len() + (input.temperature * 100.0) as usize)
}
#[instrument(fields(input_hash = %hash_input(&input)))]
async fn monitored_ai_workflow(input: SafeAIInput) -> Result<String, AIWorkflowError> {
    let start = Instant::now();
   
    match safe_ai_call(input).await {
        Ok(response) => {
            info!(duration = ?start.elapsed(), "AI call succeeded");
            Ok(response)
        }
        Err(e) => {
            error!(error = %e, duration = ?start.elapsed(), "AI call failed");
            // Trigger alerts, fallback logic, etc.
            Err(e)
        }
    }
}
// Simulate an AI API call
async fn call_ai_api(input: SafeAIInput) -> Result<String, AIWorkflowError> {
    // Simulate some processing time
    tokio::time::sleep(Duration::from_millis(500)).await;
   
    // Simulate occasional failures for demonstration
    if input.prompt.contains("error") {
        return Err(AIWorkflowError::InvalidResponse {
            reason: "Test error triggered".to_string(),
        });
    }
   
    Ok(format!("AI response to: '{}' (temp: {})", input.prompt, input.temperature))
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();


    println!("🚀 Starting AI Safety Workflow Demo");


    // Test input
    let test_input = SafeAIInput {
        prompt: "Hello, how are you?".to_string(),
        temperature: 0.7,
    };


    // Test successful workflow
    match monitored_ai_workflow(test_input).await {
        Ok(response) => println!("✅ Success: {}", response),
        Err(e) => println!("❌ Error: {}", e),
    }


    // Test error handling
    let error_input = SafeAIInput {
        prompt: "This will trigger an error".to_string(),
        temperature: 1.0,
    };


    match monitored_ai_workflow(error_input).await {
        Ok(response) => println!("✅ Success: {}", response),
        Err(e) => println!("❌ Expected Error: {}", e),
    }


    // Test circuit breaker
    let circuit_breaker = AICircuitBreaker::new();
    println!("🔌 Circuit breaker can execute: {}", circuit_breaker.can_execute());


    // Test JSON deserialization with validation
    let json_input = r#"{"prompt": "Test from JSON", "temperature": 0.5}"#;
    match serde_json::from_str::<SafeAIInput>(json_input) {
        Ok(input) => {
            println!("📝 Parsed JSON input successfully");
            match monitored_ai_workflow(input).await {
                Ok(response) => println!("✅ JSON workflow success: {}", response),
                Err(e) => println!("❌ JSON workflow error: {}", e),
            }
        }
        Err(e) => println!("❌ JSON parsing error: {}", e),
    }


    // Test validation failure
    let invalid_json = r#"{"prompt": "Test", "temperature": 5.0}"#;
    match serde_json::from_str::<SafeAIInput>(invalid_json) {
        Ok(_) => println!("This shouldn't work"),
        Err(e) => println!("✅ Validation correctly rejected invalid temperature: {}", e),
    }
    println!("🎉 AI Safety Demo completed!");
    Ok(())
}
