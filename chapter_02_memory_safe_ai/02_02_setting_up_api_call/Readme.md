# Rust AI API Client

A simple Rust application that demonstrates how to make API calls to Groq's AI service using their free API. This project uses the Groq API (a free alternative to OpenAI) to interact with the Llama 3.1 8B model.

## Prerequisites

Before running this project, make sure you have:

- [Rust](https://rustup.rs/) installed on your system
- A free Groq API account and API key

## Setup Instructions

### 1. Clone or Download the Project

Make sure you have both files in your project directory:
- `main.rs`
- `Cargo.toml`

### 2. Get Your Groq API Key

1. Visit [Groq Console](https://console.groq.com/)
2. Sign up for a free account (if you don't have one)
3. Navigate to the API Keys section
4. Create a new API key
5. Copy the API key (keep it secure!)

### 3. Set Environment Variable

The application reads the API key from an environment variable called `GROQ_API_KEY`.

#### On Windows (PowerShell):
```powershell
$env:GROQ_API_KEY="your_actual_api_key_here"
```

#### On Windows (Command Prompt):
```cmd
set GROQ_API_KEY=your_actual_api_key_here
```

#### On macOS/Linux:
```bash
export GROQ_API_KEY="your_actual_api_key_here"
```

**Important**: Replace `your_actual_api_key_here` with your actual API key from Groq Console.

### 4. Run the Application

Navigate to your project directory and run:

```bash
cargo run
```

## What the Application Does

This application:

1. **Sets up an HTTP client** using `reqwest`
2. **Reads the API key** from the `GROQ_API_KEY` environment variable
3. **Creates an AI request** with:
   - Model: `llama-3.1-8b-instant` (fast and free)
   - System message: "You are a helpful assistant"
   - User message: "Hello! Can you tell me a fun fact about Rust programming language?"
   - Max tokens: 150
4. **Sends the request** to Groq's API
5. **Displays the response** from the AI

## Expected Output

When you run the application successfully, you should see output similar to:

```
Sending request to Groq AI API...
Using model: llama-3.1-8b-instant
Request: AIRequest {
    model: "llama-3.1-8b-instant",
    messages: [
        Message {
            role: "system",
            content: "You are a helpful assistant.",
        },
        Message {
            role: "user",
            content: "Hello! Can you tell me a fun fact about Rust programming language?",
        },
    ],
    max_tokens: 150,
}

API Response received!
Response: AIResponse {
    choices: [
        Choice {
            message: Message {
                role: "assistant",
                content: "One fun fact about the Rust programming language is that its creator, Graydon Hoare, started working on the language in 2006, but the project was initially called \"Static Rust\" or just \"Static.\" It was later renamed to Rust in 2009. The primary focus behind Rust was to provide memory safety without using a garbage collector.",
            },
        },
    ],
}

AI Response:
One fun fact about the Rust programming language is that its creator, Graydon Hoare, started working on the language in 2006, but the project was initially called "Static Rust" or just "Static." It was later renamed to Rust in 2009. The primary focus behind Rust was to provide memory safety without using a garbage collector.
```

## Troubleshooting

### Error: "GROQ_API_KEY environment variable not set"
- Make sure you've set the environment variable correctly
- Check that you're running `cargo run` in the same terminal session where you set the variable
- Verify the variable name is exactly `GROQ_API_KEY` (case-sensitive)

### Error: "Failed to send request to AI API"
- Check your internet connection
- Verify your API key is correct and valid
- Make sure you haven't exceeded any rate limits

### Error: "API request failed with status 401"
- Your API key is invalid or expired
- Double-check you copied the API key correctly
- Generate a new API key from Groq Console

### Error: "API request failed with status 429"
- You've hit the rate limit
- Wait a moment and try again
- Consider upgrading your Groq plan if you need higher limits

## Dependencies

This project uses the following Rust crates:

- `tokio` - Async runtime
- `reqwest` - HTTP client with JSON support
- `serde` - Serialization/deserialization
- `serde_json` - JSON support for serde
- `anyhow` - Error handling

## Customization

You can modify the AI request in `main.rs`:

- **Change the model**: Replace `"llama-3.1-8b-instant"` with other Groq models
- **Modify the prompt**: Change the user message content
- **Adjust max_tokens**: Increase/decrease the response length limit
- **Add more messages**: Create a conversation history


## Next Steps

Try modifying the code to:
- Accept user input from the command line
- Save responses to a file
- Create a chat loop for ongoing conversations
- Add error recovery and retry logic


