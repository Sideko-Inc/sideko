use camino::Utf8PathBuf;
use url::Url;

use crate::CliResult;

pub fn validate_url(val: &str) -> CliResult<Url> {
    url::Url::parse(val)
        .map_err(|_| crate::CliError::ArgumentError(format!("URL `{val}` is not a valid URL")))
}

pub enum PathKind {
    File,
    Dir,
}

pub fn validate_path(buf: &Utf8PathBuf, path_kind: &PathKind, allow_dne: bool) -> CliResult<()> {
    let (allowed, err_msg) = match (path_kind, allow_dne) {
        (PathKind::File, false) => (
            buf.is_file(),
            format!("Path `{buf}` must be a file or a non-existent path"),
        ),
        (PathKind::File, true) => (
            buf.is_file() || !buf.exists(),
            format!("Path `{buf}` must be a file or a non-existent path"),
        ),
        (PathKind::Dir, false) => (
            buf.is_dir(),
            format!("Path `{buf}` must be a directory or a non-existent path"),
        ),
        (PathKind::Dir, true) => (
            buf.is_dir() || !buf.exists(),
            format!("Path `{buf}` must be a directory or a non-existent path"),
        ),
    };

    if allowed {
        Ok(())
    } else {
        Err(crate::CliError::ArgumentError(err_msg))
    }
}
