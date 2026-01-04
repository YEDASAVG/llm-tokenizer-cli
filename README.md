# 🔢 LLM Tokenizer CLI

A blazingly fast command-line tool to count tokens and estimate costs for Large Language Model APIs. Built with Rust for maximum performance.

## ✨ Features

- 🎯 **Accurate Token Counting** - Uses OpenAI's official tiktoken algorithm
- 💰 **Real-time Cost Estimation** - Calculate input & output costs instantly
- 📁 **Flexible Input** - Support for both file input and stdin piping
- ⚡ **Lightning Fast** - Powered by Rust's performance
- 🔍 **Multiple Models** - Support for GPT-4 and GPT-3.5-turbo

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ installed ([Install Rust](https://rustup.rs/))

### Installation

```bash
# Clone the repository
git clone https://github.com/YEDASAVG/llm-tokenizer-cli.git
cd llm-tokenizer-cli

# Build the project
cargo build --release

# The binary will be in target/release/tokenizer_cli
```

## 📖 Usage

### Basic Usage

```bash
# Count tokens from a file
cargo run -- --model gpt-4 --file input.txt

# Or use short flags
cargo run -- -m gpt-4 -f input.txt
```

### Using stdin (Pipe Input)

```bash
# From echo
echo "Hello, world!" | cargo run -- --model gpt-4

# From file
cat large-prompt.txt | cargo run -- --model gpt-3.5-turbo

# From clipboard (macOS)
pbpaste | cargo run -- --model gpt-4
```

### Compare Models

```bash
# GPT-4 (expensive, high quality)
cargo run -- --model gpt-4 --file prompt.txt

# GPT-3.5-turbo (cheaper, faster)
cargo run -- --model gpt-3.5-turbo --file prompt.txt
```

### Help

```bash
cargo run -- --help
```

## 🤖 Supported Models

| Model | Input Cost | Output Cost | Best For |
|-------|------------|-------------|----------|
| `gpt-4` | $30.00 / 1M tokens | $60.00 / 1M tokens | Complex reasoning, high accuracy |
| `gpt-3.5-turbo` | $0.50 / 1M tokens | $1.50 / 1M tokens | Fast responses, simple tasks |

*Pricing as of January 2026*

## 📊 Example Output

```bash
$ echo "Explain quantum computing in simple terms" | cargo run -- --model gpt-4

Model: gpt-4
Token count: 8
Input cost: $0.000240 (@ $30/1M tokens)
Output cost (estimated): $0.000480 (@ $60/1M tokens)
Total cost (estimated): $0.000720
```

```bash
$ cargo run -- --model gpt-3.5-turbo --file essay.txt

Model: gpt-3.5-turbo
Token count: 15,482
Input cost: $0.007741 (@ $0.5/1M tokens)
Output cost (estimated): $0.023223 (@ $1.5/1M tokens)
Total cost (estimated): $0.030964
```

## 🛠️ Tech Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[tiktoken-rs](https://github.com/zurawiki/tiktoken-rs)** - OpenAI's tokenizer (Rust port)
- **[clap](https://github.com/clap-rs/clap)** - Command-line argument parser
- **[anyhow](https://github.com/dtolnay/anyhow)** - Flexible error handling

## 💡 Use Cases

### 1. Pre-flight Cost Check
Estimate costs before making API calls to avoid surprises on your bill.

```bash
cat user-prompt.txt | cargo run -- --model gpt-4
# Check if cost is acceptable before calling API
```

### 2. Prompt Optimization
Find prompts that exceed token limits and need optimization.

```bash
# GPT-4 has 8k token context window
cargo run -- --model gpt-4 --file long-prompt.txt
# Token count: 9,234 ← Need to optimize!
```

### 3. Model Selection
Compare costs to choose the right model for your budget.

```bash
# Same prompt, different models
cargo run -- -m gpt-4 -f prompt.txt          # $0.037
cargo run -- -m gpt-3.5-turbo -f prompt.txt  # $0.001
# 37x cheaper with GPT-3.5!
```

### 4. Batch Cost Analysis
Process multiple files to estimate total project cost.

```bash
for file in prompts/*.txt; do
    echo "File: $file"
    cargo run -- -m gpt-4 -f "$file"
    echo "---"
done
```

## 🏗️ Project Structure

```
tokenizer_cli/
├── src/
│   ├── main.rs        # CLI interface & argument parsing
│   ├── tokenizer.rs   # Token counting logic
│   └── cost.rs        # Pricing calculations
├── Cargo.toml         # Dependencies
└── README.md
```

## 🔮 Roadmap

- [ ] Support for Claude models (with approximation)
- [ ] Verbose mode with token-by-token breakdown
- [ ] JSON output format for scripting
- [ ] Batch processing with summary statistics
- [ ] Custom pricing configuration file
- [ ] Support for GPT-4 Turbo and GPT-4o

## 📚 Part of Learning Series

This is **Tool 1 of 4** from Week 1 of my **6-Month AI/Inference Engineer Roadmap**.

**Week 1 Tools:**
1. ✅ **Tokenizer CLI** (this project)
2. 🔜 Prompt Formatter
3. 🔜 Cost Calculator
4. 🔜 Token Analyzer

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests

## 📝 License

MIT License - feel free to use this project for learning or production!

## 🙏 Acknowledgments

- OpenAI for the tiktoken algorithm
- Rust community for excellent tooling

---

**Built with ❤️ using Rust** | **Author:** [@YEDASAVG](https://github.com/YEDASAVG)
