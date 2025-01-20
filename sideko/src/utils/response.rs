use camino::Utf8PathBuf;
use regex::Regex;
use sideko_rest_api::BinaryResponse;
use std::str::FromStr;

/// Extracts filename from the content disposition header.
/// Expected format: "attachment; filename=\"some_file.txt\""
///
/// # Returns
/// - `Some(Utf8PathBuf)` if a valid filename was found and parsed
/// - `None` if the header is missing, malformed, or contains an invalid filename
pub fn extract_filename(res: &BinaryResponse) -> Option<Utf8PathBuf> {
    // Compile regex pattern - only needs to happen once in practice
    let pattern = Regex::new(r#"filename=["'](?<filename>[^"']+)["']"#)
        .expect("invalid extract_filename regex pattern");

    // Extract and convert content-disposition header to str
    let content_dispo = res.headers.get("content-disposition")?.to_str().ok()?;

    // Extract filename from content disposition using regex
    let name_match = pattern.captures(content_dispo)?.name("filename")?;

    // Convert filename to Utf8PathBuf
    Utf8PathBuf::from_str(name_match.as_str()).ok()
}
