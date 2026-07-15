//! Cross-platform utility functions for locating the `pkl` CLI binary.

use std::path::PathBuf;
use crate::error::{PklError, PklResult};

/// The default name of the `pkl` CLI binary.
const PKL_BIN: &str = "pkl";

/// Find the `pkl` CLI binary on the system.
///
/// Searches `PATH` using the `which` crate. Returns the full path
/// or a descriptive error if not found.
///
/// On success, the returned string is suitable for passing to
/// `Command::new()`. If desired, a custom path can override this.
pub fn find_pkl() -> PklResult<String> {
    find_pkl_inner(PKL_BIN)
}

fn find_pkl_inner(bin_name: &str) -> PklResult<String> {
    match which::which(bin_name) {
        Ok(path) => Ok(path.to_string_lossy().into_owned()),
        Err(_) => {
            let msg = format!(
                "could not find `{bin_name}` CLI binary\n\
                 \n\
                 pkl-bindgen requires the `pkl` CLI to be installed.\n\
                 \n\
                 Install it from: https://pkl-lang.org\n\
                 Or set a custom path via `EvaluatorOptions::pkl_path`\n\
                 \n\
                 Searched in PATH: $PATH"
            );
            Err(PklError::CliError(msg))
        }
    }
}

/// Returns the default `pkl` binary name, resolved via `which` if available,
/// otherwise just the literal `"pkl"` string.
///
/// Unlike `find_pkl`, this never errors — it's a best-effort resolution
/// suitable for `Default` implementations.
pub fn default_pkl_path() -> String {
    which::which(PKL_BIN)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| PKL_BIN.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pkl() {
        // This will skip if pkl is not installed
        if std::process::Command::new("pkl").arg("--version").output().is_err() {
            eprintln!("Skipping: pkl CLI not found in PATH");
            return;
        }
        let result = find_pkl();
        assert!(result.is_ok(), "should find pkl when installed: {:?}", result);
        let path = result.unwrap();
        assert!(!path.is_empty(), "path should not be empty");
        assert!(path.contains("pkl"), "path should contain 'pkl': {}", path);
    }

    #[test]
    fn test_find_pkl_not_found() {
        // This should always fail gracefully
        let result = find_pkl_inner("nonexistent-pkl-binary-xyz");
        assert!(result.is_err(), "should error when binary not found");
    }

    #[test]
    fn test_default_pkl_path() {
        let path = default_pkl_path();
        assert!(!path.is_empty(), "default path should not be empty");
    }
}
