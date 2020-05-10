use std::path::Path;
use std::path::PathBuf;

mod stack;

#[cfg(test)]
mod assertion;

#[cfg(test)]
mod file_helpers;

#[cfg(test)]
pub use file_helpers::*;

#[cfg(test)]
pub use assertion::*;

use crate::Error;
pub use stack::*;

pub fn expand<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == PathBuf::from("/") {
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

pub fn resolve_path_with_current_module_path(
    current_module_path: String,
    path: String,
) -> Result<String, Error> {
    let path = crate::utils::expand(path)
        .ok_or_else(|| Error::generic_execution_error("Cannot expand path."))?
        .to_str()
        .ok_or_else(|| Error::generic_execution_error("Cannot expand path."))?
        .to_string();

    if current_module_path == "" {
        return Ok(path);
    }

    let mut resolved_path = PathBuf::from(current_module_path);
    let mut path = PathBuf::from(path);

    if path.is_absolute() {
        let path = path
            .to_str()
            .ok_or_else(|| {
                Error::generic_execution_error("Cannot resolve path.")
            })?
            .to_string();

        return Ok(path);
    }

    resolved_path.pop();
    resolved_path.push(path);

    let resolved_path = resolved_path
        .to_str()
        .ok_or_else(|| Error::generic_execution_error("Cannot resolve path."))?
        .to_string();

    Ok(resolved_path)
}
