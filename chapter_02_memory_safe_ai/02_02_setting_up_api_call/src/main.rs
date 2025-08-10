use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::env;

#[derive(Debug, Serialize)]
struct AIRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

async fn call_ai_api(client: &Client, api_key: &str, request: AIRequest) -> Result<AIResponse> {
    // Using Groq API instead of OpenAI (free alternative)
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to send request to AI API")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!("API request failed with status {}: {}", status, error_text);
    }

    let ai_response: AIResponse = response
        .json()
        .await
        .context("Failed to parse AI API response")?;

    Ok(ai_response)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment variable ( using Groq)
    let api_key = env::var("GROQ_API_KEY")
        .context("GROQ_API_KEY environment variable not set. Get a free API key from https://console.groq.com/")?;

    // Create HTTP client 
    let client = Client::new();

    // Create a sample AI request (using Groq's fast models)
    let request = AIRequest {
        model: "llama-3.1-8b-instant".to_string(), 
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "Hello! Can you tell me a fun fact about Rust programming language?".to_string(),
            },
        ],
        max_tokens: 150,
    };

    println!("Sending request to Groq AI API...");
    println!("Using model: {}", request.model);
    println!("Request: {:#?}", request);

    // Make the API call
    match call_ai_api(&client, &api_key, request).await {
        Ok(response) => {
            println!("\nAPI Response received!");
            println!("Response: {:#?}", response);
            
            // Extract and display the AI's message
            if let Some(choice) = response.choices.first() {
                println!("\nAI Response:");
                println!("{}", choice.message.content);
            }
        }
        Err(e) => {
            eprintln!("Error calling AI API: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

