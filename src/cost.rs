pub fn get_pricing(model: &str) -> (f64, f64) {
    match model {
        // Returns input cost per 1M token , output cost per 1M token
        "gpt-4" => (30.0, 60.0),
        "gpt-3.5-turbo" => (0.50, 1.50),
        _ => (0.0, 0.0), // unknown models
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpt4_pricing() {
        let (input, output) = get_pricing("gpt-4");
        assert_eq!(input, 30.0);
        assert_eq!(output, 60.0);
    }
    #[test]
    fn test_gpt35_pricing() {
        let (input, output) = get_pricing("gpt-3.5-turbo");
        assert_eq!(input, 0.50);
        assert_eq!(output, 1.50);
    }
    #[test]
    fn test_unknown_model() {
        let (input, output) = get_pricing("fake-model");
        assert_eq!(input, 0.0);
        assert_eq!(output, 0.0);
    }
}