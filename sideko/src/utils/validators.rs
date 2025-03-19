use std::str::FromStr;

use camino::Utf8PathBuf;

// ------------- PATH VALIDATORS -------------
#[derive(Clone, Debug)]
pub enum PathKind {
    File,
    Dir,
}
/// Validates path kind & if it exists (optionally)
pub fn validate_path(
    raw_path: &str,
    path_kind: PathKind,
    allow_dne: bool,
) -> Result<Utf8PathBuf, String> {
    let path =
        Utf8PathBuf::from_str(raw_path).map_err(|_e| format!("ill-formed path: {raw_path}"))?;

    let (allowed, err_msg) = match (path_kind, allow_dne) {
        (PathKind::File, false) => (
            path.is_file(),
            format!("path `{path}` must be an existing file"),
        ),
        (PathKind::File, true) => (
            path.is_file() || !path.exists(),
            format!("path `{path}` must be a file or a non-existent path"),
        ),
        (PathKind::Dir, false) => (
            path.is_dir(),
            format!("path `{path}` must be an existing directory"),
        ),
        (PathKind::Dir, true) => (
            path.is_dir() || !path.exists(),
            format!("path `{path}` must be a directory or a non-existent path"),
        ),
    };

    if allowed {
        Ok(path)
    } else {
        Err(err_msg)
    }
}

pub fn validate_file_with_extension(
    raw_path: &str,
    allow_dne: bool,
    extensions: &[&str],
) -> Result<Utf8PathBuf, String> {
    let path = validate_path(raw_path, PathKind::File, allow_dne)?;
    let extension = format!(".{}", path.extension().unwrap_or_default());
    if !extensions.contains(&extension.as_str()) {
        Err(format!(
            "path has incorrect extension, only {extensions:?} are permitted"
        ))
    } else {
        Ok(path)
    }
}

/// Validates path exists and is a file
pub(crate) fn validate_file(arg: &str) -> Result<Utf8PathBuf, String> {
    validate_path(arg, PathKind::File, false)
}
/// Validates file path exists and is either a json or yaml
pub(crate) fn validate_file_json_yaml(arg: &str) -> Result<Utf8PathBuf, String> {
    validate_file_with_extension(arg, false, &[".json", ".yml", ".yaml"])
}
/// Validates file path exists and is either a json or yaml (does not exist is allowed)
pub(crate) fn validate_file_json_yaml_allow_dne(arg: &str) -> Result<Utf8PathBuf, String> {
    validate_file_with_extension(arg, true, &[".json", ".yml", ".yaml"])
}
/// Validates file path has yaml extension
pub(crate) fn validate_file_yaml(arg: &str) -> Result<Utf8PathBuf, String> {
    validate_file_with_extension(arg, false, &[".yml", ".yaml"])
}
/// Validates yaml extension (does not exist is allowed)
pub(crate) fn validate_file_yaml_allow_dne(arg: &str) -> Result<Utf8PathBuf, String> {
    validate_file_with_extension(arg, true, &[".yml", ".yaml"])
}

#[allow(unused)]
/// Validates path is a file or does not exist
pub(crate) fn validate_file_allow_dne(arg: &str) -> Result<Utf8PathBuf, String> {
    validate_path(arg, PathKind::File, true)
}
/// Validates path exists and is a directory
pub(crate) fn validate_dir(arg: &str) -> Result<Utf8PathBuf, String> {
    validate_path(arg, PathKind::Dir, true)
}
/// Validates path is a directory or does not exist
pub(crate) fn validate_dir_allow_dne(arg: &str) -> Result<Utf8PathBuf, String> {
    validate_path(arg, PathKind::Dir, true)
}
