pub fn get_pricing(model: &str) -> (f64, f64) {
    match model {
        // Returns input cost per 1M token , output cost per 1M token
        "gpt-4" => (30.0, 60.0),
        "gpt-3.5-turbo" => (0.50, 1.50),
        _ => (0.0, 0.0), // unknown models
    }
}
