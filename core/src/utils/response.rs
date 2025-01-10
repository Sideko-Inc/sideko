use std::str::FromStr;

use camino::Utf8PathBuf;
use regex::Regex;
use sideko_rest_api::BinaryResponse;

/// Extracts filename from the content disposition header. Expected
/// format: "attachment; filename=\"some_file.txt\""
pub fn extract_filename(res: &BinaryResponse) -> Option<Utf8PathBuf> {
    let pattern = Regex::new(r#"filename=["'](?<filename>.+)["']"#)
        .expect("invalid extract_filename regex pattern");
    if let Some(content_dispo) = &res
        .headers
        .get("content-disposition")
        .and_then(|v| v.to_str().ok())
    {
        if let Some(name_match) = pattern
            .captures(content_dispo)
            .and_then(|c| c.name("filename"))
        {
            if let Ok(path) = Utf8PathBuf::from_str(name_match.as_str()) {
                return Some(path);
            }
        }
    }

    None
}
