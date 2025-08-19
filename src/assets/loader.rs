/**
Loads a bundled asset (ASCII art, text or JSON (`coming soon`)) that's compiled into the binary.

# Panics
Panics if the asset name is invalid (should never happen in release builds).<br>
If this happens please report it at the GitHub repository.
*/
pub fn load_asset(filename: &str) -> &'static str {
    match filename {
        // loads assets
        "warning_ascii.txt" => include_str!("../assets/warning_ascii.txt"),
        "warning_volume.txt" => include_str!("../assets/warning_volume.txt"),
        _ => panic!(
            "Internal error: Unknown asset '{}'.\n\
            This is a bug - please report at:\n\
            https://github.com/xshotss/rs-audio/issues",
            filename
        ),
    }
}
