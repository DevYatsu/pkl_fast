use async_trait::async_trait;
use tokio::process::Command;
use tokio::io::AsyncWriteExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::decode::{self, PklDecode};
use crate::error::{PklError, PklResult};
use crate::evaluator::PklEvaluator;
use crate::module_source::ModuleSource;
use crate::value::Value;

/// Evaluates Pkl modules by spawning the `pkl` CLI binary as a subprocess.
///
/// Communicates using Pkl's binary format (`--format pkl-binary`), which
/// preserves all type information (Duration, DataSize, Map, Set, etc.).
pub struct CliEvaluator {
    pkl_path: String,
    closed: Arc<AtomicBool>,
}

impl CliEvaluator {
    /// Create a new CLI evaluator that will invoke `pkl` (found in PATH).
    pub fn new() -> Self {
        Self {
            pkl_path: crate::util::default_pkl_path(),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Create a new CLI evaluator with a custom path to the `pkl` binary.
    pub fn with_pkl_path(pkl_path: impl Into<String>) -> Self {
        Self {
            pkl_path: pkl_path.into(),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Default for CliEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PklEvaluator for CliEvaluator {
    async fn evaluate<T: PklDecode>(&self, source: &ModuleSource) -> PklResult<T> {
        let value = self.evaluate_raw(source).await?;
        T::decode(value)
    }

    async fn evaluate_raw(&self, source: &ModuleSource) -> PklResult<Value> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(PklError::CliError("evaluator is closed".to_string()));
        }

        // Use binary format to preserve all Pkl type information
        let mut cmd = Command::new(&self.pkl_path);
        cmd.arg("eval")
            .arg("--format")
            .arg("pkl-binary");

        if let Some(ref text) = source.contents {
            // Inline text: pipe via stdin
            cmd.arg("-");
            let mut child = cmd
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| PklError::CliError(format!("failed to spawn pkl: {}", e)))?;

            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(text.as_bytes()).await.map_err(|e| {
                    PklError::CliError(format!("failed to write to pkl stdin: {}", e))
                })?;
            }
            // Drop stdin to signal EOF
            drop(child.stdin.take());

            let output = child.wait_with_output().await.map_err(|e| {
                PklError::CliError(format!("pkl process error: {}", e))
            })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(PklError::EvalError(format!(
                    "pkl exited with {}: {}",
                    output.status,
                    stderr.trim()
                )));
            }

            decode::decode_response(&output.stdout)
        } else {
            // File/URI: pass as argument
            cmd.arg(&source.uri);
            let output = cmd
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| PklError::CliError(format!("failed to spawn pkl: {}", e)))?
                .wait_with_output()
                .await
                .map_err(|e| PklError::CliError(format!("pkl process error: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(PklError::EvalError(format!(
                    "pkl exited with {}: {}",
                    output.status,
                    stderr.trim()
                )));
            }

            decode::decode_response(&output.stdout)
        }
    }

    async fn evaluate_text(&self, source: &ModuleSource) -> PklResult<String> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(PklError::CliError("evaluator is closed".to_string()));
        }

        // Use JSON format for text output
        let mut cmd = Command::new(&self.pkl_path);
        cmd.arg("eval")
            .arg("--format")
            .arg("json")
            .arg("-x")
            .arg("output.text");

        if let Some(ref text) = source.contents {
            cmd.arg("-");
            let mut child = cmd
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| PklError::CliError(format!("failed to spawn pkl: {}", e)))?;

            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(text.as_bytes()).await.map_err(|e| {
                    PklError::CliError(format!("failed to write to pkl stdin: {}", e))
                })?;
            }
            drop(child.stdin.take());

            let output = child.wait_with_output().await.map_err(|e| {
                PklError::CliError(format!("pkl process error: {}", e))
            })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(PklError::EvalError(format!(
                    "pkl exited with {}: {}",
                    output.status,
                    stderr.trim()
                )));
            }

            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            cmd.arg(&source.uri);
            let output = cmd
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| PklError::CliError(format!("failed to spawn pkl: {}", e)))?
                .wait_with_output()
                .await
                .map_err(|e| PklError::CliError(format!("pkl process error: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(PklError::EvalError(format!(
                    "pkl exited with {}: {}",
                    output.status,
                    stderr.trim()
                )));
            }

            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        }
    }

    async fn close(&self) -> PklResult<()> {
        self.closed.store(true, Ordering::SeqCst);
        Ok(())
    }
}
