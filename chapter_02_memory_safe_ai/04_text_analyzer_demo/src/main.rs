
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

// Core data structures - showing Rust's type safety
#[derive(Debug, Serialize, Deserialize)]
pub struct TextAnalysis {
    pub sentiment: SentimentScore,
    pub key_topics: Vec<Topic>,
    pub confidence: f64,
    pub processing_time: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentimentScore {
    pub positive: f64,
    pub negative: f64,
    pub neutral: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub name: String,
    pub relevance: f64,
    pub category: String,
}

// Error handling - explicit and safe
#[derive(Debug)]
pub enum AnalysisError {
    EmptyInput,
    InputTooLarge,
    ApiError(String),
    CircuitBreakerOpen,
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AnalysisError::EmptyInput => write!(f, "Input text is empty"),
            AnalysisError::InputTooLarge => write!(f, "Input text too large"),
            AnalysisError::ApiError(msg) => write!(f, "API error: {}", msg),
            AnalysisError::CircuitBreakerOpen => write!(f, "Circuit breaker open"),
        }
    }
}

impl std::error::Error for AnalysisError {}

// Circuit breaker for safety
pub struct CircuitBreaker {
    failure_count: u32,
    threshold: u32,
}

impl CircuitBreaker {
    pub fn new() -> Self {
        Self { failure_count: 0, threshold: 3 }
    }

    pub fn can_execute(&self) -> bool {
        self.failure_count < self.threshold
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
    }

    pub fn record_success(&mut self) {
        self.failure_count = 0;
    }
}

// Main analyzer - production patterns in action
pub struct TextAnalyzer {
    circuit_breaker: CircuitBreaker,
}

impl TextAnalyzer {
    pub fn new() -> Result<Self, AnalysisError> {
        Ok(Self {
            circuit_breaker: CircuitBreaker::new(),
        })
    }

    // Input validation - Rust's type system at work
    pub fn sanitize_input(&self, input: &str) -> Result<String, AnalysisError> {
        if input.is_empty() {
            return Err(AnalysisError::EmptyInput);
        }

        if input.len() > 10_000 {
            return Err(AnalysisError::InputTooLarge);
        }

        // Safe character filtering
        let cleaned: String = input
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?'-".contains(*c))
            .collect();

        Ok(cleaned)
    }

    // Main analysis function - all safety patterns together
    pub async fn analyze_text(&mut self, input: &str) -> Result<TextAnalysis, AnalysisError> {
        let start_time = Instant::now();

        // Step 1: Input validation
        let sanitized = self.sanitize_input(input)?;

        // Step 2: Circuit breaker check
        if !self.circuit_breaker.can_execute() {
            return Err(AnalysisError::CircuitBreakerOpen);
        }

        // Step 3: Safe AI processing (mocked for demo)
        match self.mock_ai_call(&sanitized).await {
            Ok(mut analysis) => {
                self.circuit_breaker.record_success();
                analysis.processing_time = start_time.elapsed();
                Ok(analysis)
            }
            Err(e) => {
                self.circuit_breaker.record_failure();
                Err(e)
            }
        }
    }

    // Mock AI call demonstrating the patterns
    async fn mock_ai_call(&self, text: &str) -> Result<TextAnalysis, AnalysisError> {
        // Simulate API call delay
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Simple sentiment analysis for demo
        let text_lower = text.to_lowercase();
        let positive_words = ["love", "amazing", "great", "excellent"];
        let negative_words = ["hate", "terrible", "awful", "bad"];

        let positive_count = positive_words.iter()
            .filter(|&word| text_lower.contains(word))
            .count() as f64;
        let negative_count = negative_words.iter()
            .filter(|&word| text_lower.contains(word))
            .count() as f64;

        let total = positive_count + negative_count;
        let (pos, neg, neu) = if total > 0.0 {
            (positive_count / total, negative_count / total, 0.1)
        } else {
            (0.2, 0.2, 0.6)
        };

        let sentiment = SentimentScore {
            positive: pos,
            negative: neg,
            neutral: neu,
        };

        let topics = vec![
            Topic {
                name: "product feedback".to_string(),
                relevance: 0.9,
                category: "business".to_string(),
            }
        ];

        Ok(TextAnalysis {
            sentiment,
            key_topics: topics,
            confidence: 0.85,
            processing_time: Duration::from_millis(0), // Will be set by caller
        })
    }

    // Batch processing for performance demo
    pub async fn analyze_batch(&mut self, texts: &[&str]) -> Vec<Result<TextAnalysis, AnalysisError>> {
        let mut results = Vec::new();
        
        for text in texts {
            let result = self.analyze_text(text).await;
            results.push(result);
        }
        
        results
    }
}

// Demo function showing everything in action
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Rust AI Text Analyzer Demo");
    println!("===============================\n");

    let mut analyzer = TextAnalyzer::new()?;

    // Sample texts for demonstration
    let sample_texts = vec![
        "I absolutely love this new product! It's amazing!",
        "The service was okay, nothing special.",
        "This is terrible. I want my money back.",
        "The documentation could be improved, but the core functionality is solid."
    ];

    let start_time = Instant::now();

    println!("📊 Batch Analysis Results:");
    println!("==========================");

    for (i, text) in sample_texts.iter().enumerate() {
        println!("\n--- Text {} ---", i + 1);
        
        match analyzer.analyze_text(text).await {
            Ok(analysis) => {
                println!("📝 Text: {}", text);
                println!("🎭 Sentiment:");
                println!("   Positive: {:.1}%", analysis.sentiment.positive * 100.0);
                println!("   Negative: {:.1}%", analysis.sentiment.negative * 100.0);
                println!("   Neutral:  {:.1}%", analysis.sentiment.neutral * 100.0);
                println!("🎯 Confidence: {:.1}%", analysis.confidence * 100.0);
                println!("⏱️  Processing: {:?}", analysis.processing_time);
                
                let emoji = if analysis.sentiment.positive > 0.5 { "😊" } 
                           else if analysis.sentiment.negative > 0.5 { "😞" } 
                           else { "😐" };
                println!("📊 Overall: {}", emoji);
            }
            Err(e) => {
                println!("❌ Analysis failed: {}", e);
            }
        }
    }

    let total_time = start_time.elapsed();
    println!("\n🏁 Performance Summary:");
    println!("=======================");
    println!("Total time: {:?}", total_time);
    println!("Average per text: {:?}", total_time / sample_texts.len() as u32);
    println!("Texts processed: {}", sample_texts.len());

    println!("\n✅ Safety Patterns Demonstrated:");
    println!("=================================");
    println!("🛡️  Input validation and sanitization");
    println!("⚡ Comprehensive error handling");
    println!("🔌 Circuit breaker protection");
    println!("📊 Performance monitoring");
    println!("🦀 Memory-safe Rust implementation");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_input_validation() {
        let analyzer = TextAnalyzer::new().unwrap();
        
        // Test empty input
        assert!(matches!(
            analyzer.sanitize_input(""),
            Err(AnalysisError::EmptyInput)
        ));
        
        // Test valid input
        let result = analyzer.sanitize_input("Hello, world!");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_analysis() {
        let mut analyzer = TextAnalyzer::new().unwrap();
        let result = analyzer.analyze_text("I love this!").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new();
        assert!(breaker.can_execute());
        
        // Simulate failures
        for _ in 0..3 {
            breaker.record_failure();
        }
        assert!(!breaker.can_execute());
        
        // Recovery
        breaker.record_success();
        assert!(breaker.can_execute());
    }
}
