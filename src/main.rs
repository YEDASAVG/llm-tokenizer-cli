mod cost;
mod tokenizer;
use std::io::{ Read,};
use anyhow::Result;

use anyhow::Ok;
use clap::Parser;


#[derive(Parser)]
#[command(name = "tokenizer")]
#[command(about = "Count tokens and estimate costs for LLM API's")]

struct Args {
    #[arg(short, long)]
    model: String,

    #[arg(short, long)]
    file: Option<String>,
}


fn main() -> Result<()> {
    let args = Args::parse();

    let text = if let Some(file_path) = args.file {
        std::fs::read_to_string(file_path)?
    } else {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    };

    let token_count = tokenizer::count_tokens(&text, &args.model)?;
    let (input_cost_per_1m, output_cost_per_1m) = cost::get_pricing(&args.model);

    let input_cost = (token_count as f64 / 1_000_000.0) * input_cost_per_1m;
    let output_cost = (token_count as f64 / 1_000_000.0) * output_cost_per_1m;
    let total_cost = input_cost + output_cost;
    println!("Model: {}", args.model);
    println!("Token count: {}", token_count);
    println!("Input cost: ${:.6}", input_cost);
    println!("Output cost (estimated): ${:.6}", output_cost);
    println!("Total cost (estimated): ${:.6}", total_cost);

    Ok(())
}
