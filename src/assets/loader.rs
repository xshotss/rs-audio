/// Loads an asset (ASCII art or raw text) from the assets folder.
pub fn load_asset(filename: &str) -> String {
    std::fs::read_to_string(format!("src/assets/{filename}"))
        .unwrap_or_else(|_| panic!("Failed to load {filename}"))
}

