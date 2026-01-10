use anyhow::Result;
use tiktoken_rs::get_bpe_from_model;

pub fn count_tokens(text: &str, model: &str) -> Result<usize> {
    let bpe = get_bpe_from_model(model)?;
    let tokens = bpe.encode_with_special_tokens(text);
    Ok(tokens.len())
}

#[cfg(test)]

mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_count_tokens() -> Result<()> {
        let text = "Hello World";
        let model = "gpt-4";
        let count = count_tokens(text, model)?;

        assert!(count > 0);
        Ok(())
    }

    #[test]
    fn test_empty_text() -> Result<()> {
        let text = "";
        let model = "gpt-4";
        let empty_count = count_tokens(text, model)?;

        assert!(empty_count == 0);
        Ok(())
    }

    #[test]
    fn test_invalid_model() {
        let text = "Hello World";
        let model = "fake_model";

        let result = count_tokens(text, model);

        assert!(result.is_err());
    }
}
