# Debugging with GitHub Copilot


Learn to harness GitHub Copilot as your AI debugging partner for Rust development. This hands-on course module demonstrates real debugging workflows using both inline chat (`Ctrl+I`) and the Copilot chat window (`Ctrl+Alt+I`).

## What You'll Learn

- **AI-Assisted Debugging Workflow** - Two methods to get help from GitHub Copilot
- **Common Rust Pitfalls** - Off-by-one errors, logic bugs, and ownership issues
- **Interactive Problem Solving** - Run buggy code, analyze errors, apply fixes
- **Edge Case Handling** - Build robust applications with defensive programming
- **Testing Strategies** - Use AI to identify comprehensive test scenarios

## Quick Start

### Prerequisites
- Rust installed (`rustc --version`)
- VS Code with GitHub Copilot extension
- GitHub Copilot subscription

### Clone and Run
```bash
git clone https://github.com/ashishjsharda/memory-safe-systems-programming-with-rust-and-ai-2743193.git
cd chapter_03_ai_assisted_debugging/02_debugging_with_github_copilot

# Run individual examples
cargo run --bin off_by_one
cargo run --bin logic_errors
cargo run --bin ownership_issues
cargo run --bin error_handling
cargo run --bin testing_examples


```

## Course Examples

### 1. Off-by-One Errors (`off_by_one.rs`)
**The Problem**: Array indexing that goes one element too far
```rust
// This will panic!
for i in 0..=numbers.len() {
    println!("{}", numbers[i]);
}
```

**What You'll Learn**:
- How inclusive ranges (`..=`) can cause panics
- Getting Copilot's help with `Ctrl+I` or `Ctrl+Alt+I`
- Safer alternatives using iterators

### 2. Logic Errors (`logic_errors.rs`)
**The Problem**: Code compiles but produces wrong results
```rust
fn calculate_average(numbers: &[i32]) -> f64 {
    // Returns NaN for empty vectors!
    sum / count
}
```

**What You'll Learn**:
- Identifying edge cases that break logic
- Using Copilot to handle empty input gracefully
- Defensive programming with `Option<T>` types

### 3. Ownership Issues (`ownership_issues.rs`)
**The Problem**: Borrowing conflicts that prevent compilation
```rust
let first = &data[0];    // Immutable borrow
data.push(6);            // Mutable borrow - conflict!
```

**What You'll Learn**:
- Understanding Rust's borrowing rules
- Resolving mutable/immutable borrow conflicts
- Working with the ownership system, not against it

### 4. Error Handling (`error_handling.rs`)
**The Problem**: Vague error messages that don't help debugging
```rust
// Generic error: "invalid digit found in string"
// Which line? What input caused it?
```

**What You'll Learn**:
- Creating informative error messages with context
- Handling type mismatches in error conversions
- Iterative debugging when first AI suggestions need refinement

### 5. Testing Examples (`testing_examples.rs`)
**The Problem**: Incomplete test coverage missing edge cases
```rust
fn divide_numbers(a: f64, b: f64) -> f64 {
    a / b  // What about division by zero?
}
```

**What You'll Learn**:
- Using Copilot to identify missing test cases
- Comprehensive testing for mathematical functions
- Building testing intuition with AI assistance

## The Two-Method Debugging Workflow

### Method 1: Inline Chat (`Ctrl+I`)
- **Best for**: Quick questions and immediate fixes
- **How**: Select problematic code → `Ctrl+I` → Ask question
- **Example**: "Why does this panic?"

### Method 2: Chat Window (`Ctrl+Alt+I`)
- **Best for**: Detailed explanations and learning
- **How**: Select code → `Ctrl+Alt+I` → Use slash commands
- **Example**: `/explain this borrowing error`

## The Complete Debugging Cycle

Each example follows this proven workflow:

1. **Experience the Problem** - Run buggy code, see it fail
2. **Investigate with AI** - Select code, ask Copilot for help
3. **Get the Solution** - Understand the fix and why it works
4. **Apply and Verify** - Implement fix, rerun code
5. **Success** - See the code working correctly

## Course Context

This module is part of **"Memory-Safe Systems Programming with Rust and AI"** - a comprehensive course that teaches:

- Rust fundamentals with memory safety focus
- AI-assisted development workflows
- Production-ready systems programming
- Best practices for reliable applications

## Key Takeaways

- **AI debugging is conversational** - Describe problems, ask specific questions
- **First suggestions aren't always perfect** - Iteration is normal and valuable
- **Learn patterns, not just fixes** - Each interaction builds debugging intuition
- **Verify every fix** - Always rerun code to confirm solutions work

## Video Companion

Watch the companion video (03_02) to see these examples in action with:
- Live debugging sessions
- Real-time Copilot interactions
- Before/after code comparisons
- Expert commentary on debugging strategies

## Contributing

Found a bug or have suggestions? This is a learning repository, so issues and improvements are welcome! Please open an issue or submit a PR.

