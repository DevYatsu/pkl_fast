use async_trait::async_trait;
use serde::de::DeserializeOwned;
use crate::error::PklResult;
use crate::module_source::ModuleSource;
use crate::value::Value;

/// The core evaluation trait.
///
/// Abstracts over how Pkl modules are evaluated.
/// - `CliEvaluator` spawns the `pkl` CLI binary as a subprocess.
/// - `ServerEvaluator` connects to a persistent `pkl server`.
#[async_trait]
pub trait PklEvaluator: Send + Sync {
    /// Evaluate a module and decode it into the requested type.
    ///
    /// The target type must implement `serde::Deserialize`.
    async fn evaluate<T: DeserializeOwned>(&self, source: &ModuleSource) -> PklResult<T> {
        let value = self.evaluate_raw(source).await?;
        serde::Deserialize::deserialize(&value)
            .map_err(|e| crate::error::PklError::DecodeError(format!("serde: {}", e)))
    }

    /// Evaluate a module and return the raw `Value` tree.
    async fn evaluate_raw(&self, source: &ModuleSource) -> PklResult<Value>;

    /// Evaluate the `output.text` property of a module.
    async fn evaluate_text(&self, source: &ModuleSource) -> PklResult<String>;

    /// Close this evaluator and release resources.
    async fn close(&self) -> PklResult<()>;
}
