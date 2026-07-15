use async_trait::async_trait;
use crate::error::PklResult;
use crate::module_source::ModuleSource;
use crate::value::Value;

/// The core evaluation trait.
///
/// Abstracts over how Pkl modules are evaluated.
/// - `CliEvaluator` spawns the `pkl` CLI binary as a subprocess.
/// - A future `NativeEvaluator` will evaluate Pkl in-process.
#[async_trait]
pub trait PklEvaluator: Send + Sync {
    /// Evaluate a module and decode it into the requested type.
    async fn evaluate<T>(&self, source: &ModuleSource) -> PklResult<T>
    where
        T: crate::decode::PklDecode;

    /// Evaluate a module and return the raw `Value` tree.
    async fn evaluate_raw(&self, source: &ModuleSource) -> PklResult<Value>;

    /// Evaluate the `output.text` property of a module.
    async fn evaluate_text(&self, source: &ModuleSource) -> PklResult<String>;

    /// Close this evaluator and release resources.
    async fn close(&self) -> PklResult<()>;
}
