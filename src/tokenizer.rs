use anyhow::Result;
use tiktoken_rs::get_bpe_from_model; 

pub fn count_tokens(text: &str, model: &str) -> Result<usize> {
    let bpe = get_bpe_from_model(model)?;
    let tokens = bpe.encode_with_special_tokens(text);
    Ok(tokens.len())
}