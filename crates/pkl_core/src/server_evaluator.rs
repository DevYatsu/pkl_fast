use async_trait::async_trait;
use tokio::process::{Child, Command};
use tokio::io::AsyncReadExt;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};
use std::time::Duration;

use crate::decode::{self, PklDecode};
use crate::error::{PklError, PklResult};
use crate::evaluator::PklEvaluator;
use crate::module_source::ModuleSource;
use crate::msgapi::{self, ServerMessage};
use crate::project::{self as pkl_project, Project};
use crate::reader::{ModuleReader, ResourceReader};
use crate::value::Value;

// ── Options ──

/// Options for creating an `EvaluatorManager` or `ServerEvaluator`.
pub struct EvaluatorOptions {
    /// Path to the `pkl` CLI binary (default: `"pkl"`).
    pub pkl_path: String,
    /// Custom resource readers for `read("scheme:...")`.
    pub resource_readers: Vec<Box<dyn ResourceReader>>,
    /// Custom module readers for `import "scheme:..."`.
    pub module_readers: Vec<Box<dyn ModuleReader>>,
}

impl Default for EvaluatorOptions {
    fn default() -> Self {
        Self {
            pkl_path: crate::util::default_pkl_path(),
            resource_readers: Vec::new(),
            module_readers: Vec::new(),
        }
    }
}

// ── ManagerInner (shared state) ──

struct ManagerInner {
    child: Mutex<Option<Child>>,
    write_tx: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
    resource_readers: Vec<Box<dyn ResourceReader>>,
    module_readers: Vec<Box<dyn ModuleReader>>,
    create_pending: Mutex<HashMap<i64, oneshot::Sender<PklResult<ServerMessage>>>>,
    eval_pendings: Mutex<HashMap<i64, Arc<Mutex<HashMap<i64, oneshot::Sender<PklResult<ServerMessage>>>>>>>,
}

impl ManagerInner {
    async fn send(&self, bytes: Vec<u8>) -> PklResult<()> {
        self.write_tx.send(bytes)
            .map_err(|_| PklError::CliError("write channel closed".to_string()))
    }

    async fn register_create(&self, request_id: i64, tx: oneshot::Sender<PklResult<ServerMessage>>) {
        self.create_pending.lock().await.insert(request_id, tx);
    }

    async fn register_eval(&self, evaluator_id: i64, pending: Arc<Mutex<HashMap<i64, oneshot::Sender<PklResult<ServerMessage>>>>>) {
        self.eval_pendings.lock().await.insert(evaluator_id, pending);
    }

    async fn unregister_eval(&self, evaluator_id: i64) {
        self.eval_pendings.lock().await.remove(&evaluator_id);
    }
}

// ── EvaluatorManager ──

/// Manages a shared `pkl server` process. Create evaluators with `new_evaluator()`.
pub struct EvaluatorManager {
    inner: Arc<ManagerInner>,
    closed: Arc<AtomicBool>,
}

impl EvaluatorManager {
    /// Create a new manager with default options (uses `pkl` from PATH).
    pub async fn new() -> PklResult<Self> {
        Self::with_options(EvaluatorOptions::default()).await
    }

    /// Create a new manager with custom options (readers, pkl path, etc.).
    pub async fn with_options(opts: EvaluatorOptions) -> PklResult<Self> {
        let mut child = Command::new(&opts.pkl_path)
            .arg("server")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| PklError::CliError(format!("spawn pkl server: {}", e)))?;

        let child_stdin = child.stdin.take()
            .ok_or_else(|| PklError::CliError("no stdin".to_string()))?;
        let child_stdout = child.stdout.take()
            .ok_or_else(|| PklError::CliError("no stdout".to_string()))?;

        let (write_tx, mut write_rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();

        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            let mut stdin = child_stdin;
            while let Some(bytes) = write_rx.recv().await {
                if stdin.write_all(&bytes).await.is_err() { break; }
                if stdin.flush().await.is_err() { break; }
            }
        });

        let inner = Arc::new(ManagerInner {
            child: Mutex::new(Some(child)),
            write_tx,
            resource_readers: opts.resource_readers,
            module_readers: opts.module_readers,
            create_pending: Mutex::new(HashMap::new()),
            eval_pendings: Mutex::new(HashMap::new()),
        });

        // Reader task
        let reader_inner = inner.clone();
        tokio::spawn(async move {
            read_loop(child_stdout, reader_inner).await;
        });

        Ok(Self { inner, closed: Arc::new(AtomicBool::new(false)) })
    }

    /// Create a new evaluator on the shared server process.
    pub async fn new_evaluator(&self) -> PklResult<ServerEvaluator> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(PklError::CliError("manager is closed".to_string()));
        }

        // Use a simple counter for request_id (not tied to evaluator_id)
        let request_id = {
            static NEXT_ID: AtomicI64 = AtomicI64::new(1);
            NEXT_ID.fetch_add(1, Ordering::SeqCst)
        };

        let (tx, rx) = oneshot::channel();
        self.inner.register_create(request_id, tx).await;

        // Build CreateEvaluator message
        let mut client_resource_readers = vec![
            msgapi::ResourceReaderSpec { scheme: "repl".to_string(), hasHierarchicalUris: false, isGlobbable: false },
        ];
        let mut client_module_readers = vec![
            msgapi::ModuleReaderSpec { scheme: "repl".to_string(), hasHierarchicalUris: false, isGlobbable: false, isLocal: true },
        ];
        for r in &self.inner.resource_readers {
            client_resource_readers.push(msgapi::ResourceReaderSpec {
                scheme: r.scheme().to_string(), hasHierarchicalUris: r.has_hierarchical_uris(), isGlobbable: r.is_globbable(),
            });
        }
        for r in &self.inner.module_readers {
            client_module_readers.push(msgapi::ModuleReaderSpec {
                scheme: r.scheme().to_string(), hasHierarchicalUris: r.has_hierarchical_uris(), isGlobbable: r.is_globbable(), isLocal: r.is_local(),
            });
        }

        let mut allowed_modules = vec!["file:".to_string(), "https:".to_string(), "repl:".to_string(), "modulepath:".to_string(), "pkl:".to_string()];
        let mut allowed_resources = vec!["file:".to_string(), "https:".to_string(), "env:".to_string(), "prop:".to_string(), "modulepath:".to_string()];
        for r in &self.inner.resource_readers { allowed_resources.push(format!("{}:", r.scheme())); }
        for r in &self.inner.module_readers { allowed_modules.push(format!("{}:", r.scheme())); }

        let msg = msgapi::CreateEvaluator {
            requestId: request_id,
            clientResourceReaders: Some(client_resource_readers),
            clientModuleReaders: Some(client_module_readers),
            allowedModules: Some(allowed_modules),
            allowedResources: Some(allowed_resources),
            outputFormat: Some("pkl-binary".to_string()),
            project: None,
        };

        let bytes = msgapi::encode_msg(msgapi::CODE_NEW_EVALUATOR, &msg);
        self.inner.send(bytes).await?;

        let resp = tokio::time::timeout(Duration::from_secs(10), rx).await
            .map_err(|_| PklError::CliError("timeout creating evaluator".to_string()))?
            .map_err(|_| PklError::CliError("channel closed".to_string()))??;

        let evaluator_id = match resp {
            ServerMessage::EvaluatorCreated(cr) => {
                if !cr.error.is_empty() { return Err(PklError::EvalError(cr.error)); }
                cr.evaluatorId
            }
            _ => return Err(PklError::DecodeError("expected EvaluatorCreated".to_string())),
        };

        let eval_pending: Arc<Mutex<HashMap<i64, oneshot::Sender<PklResult<ServerMessage>>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        self.inner.register_eval(evaluator_id, eval_pending.clone()).await;

        Ok(ServerEvaluator {
            inner: self.inner.clone(),
            eval_pending,
            evaluator_id,
            req_counter: AtomicI64::new(1),
            closed: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Create an evaluator with project context (dependencies, etc.).
    ///
    /// Loads the project from the given directory, then creates an evaluator
    /// configured with the project's dependencies and settings.
    pub async fn new_project_evaluator(&self, project_dir: &str) -> PklResult<ServerEvaluator> {
        let project = pkl_project::load_project(project_dir).await?;
        self.new_evaluator_with_project(&project).await
    }

    /// Create an evaluator with a pre-loaded project.
    pub async fn new_evaluator_with_project(&self, project: &Project) -> PklResult<ServerEvaluator> {
        let request_id = {
            static NEXT_ID: AtomicI64 = AtomicI64::new(1);
            NEXT_ID.fetch_add(1, Ordering::SeqCst)
        };

        let (tx, rx) = oneshot::channel();
        self.inner.register_create(request_id, tx).await;

        let mut client_resource_readers = vec![
            msgapi::ResourceReaderSpec { scheme: "repl".to_string(), hasHierarchicalUris: false, isGlobbable: false },
        ];
        let mut client_module_readers = vec![
            msgapi::ModuleReaderSpec { scheme: "repl".to_string(), hasHierarchicalUris: false, isGlobbable: false, isLocal: true },
        ];
        for r in &self.inner.resource_readers {
            client_resource_readers.push(msgapi::ResourceReaderSpec {
                scheme: r.scheme().to_string(), hasHierarchicalUris: r.has_hierarchical_uris(), isGlobbable: r.is_globbable(),
            });
        }
        for r in &self.inner.module_readers {
            client_module_readers.push(msgapi::ModuleReaderSpec {
                scheme: r.scheme().to_string(), hasHierarchicalUris: r.has_hierarchical_uris(), isGlobbable: r.is_globbable(), isLocal: r.is_local(),
            });
        }

        let mut allowed_modules = vec!["file:".to_string(), "https:".to_string(), "repl:".to_string(), "modulepath:".to_string(), "pkl:".to_string()];
        let mut allowed_resources = vec!["file:".to_string(), "https:".to_string(), "env:".to_string(), "prop:".to_string(), "modulepath:".to_string()];
        for r in &self.inner.resource_readers { allowed_resources.push(format!("{}:", r.scheme())); }
        for r in &self.inner.module_readers { allowed_modules.push(format!("{}:", r.scheme())); }

        let project_msg = pkl_project::project_to_message(project);

        let msg = msgapi::CreateEvaluator {
            requestId: request_id,
            clientResourceReaders: Some(client_resource_readers),
            clientModuleReaders: Some(client_module_readers),
            allowedModules: Some(allowed_modules),
            allowedResources: Some(allowed_resources),
            outputFormat: Some("pkl-binary".to_string()),
            project: Some(project_msg),
        };

        let bytes = msgapi::encode_msg(msgapi::CODE_NEW_EVALUATOR, &msg);
        self.inner.send(bytes).await?;

        let resp = tokio::time::timeout(Duration::from_secs(10), rx).await
            .map_err(|_| PklError::CliError("timeout creating evaluator".to_string()))?
            .map_err(|_| PklError::CliError("channel closed".to_string()))??;

        let evaluator_id = match resp {
            ServerMessage::EvaluatorCreated(cr) => {
                if !cr.error.is_empty() { return Err(PklError::EvalError(cr.error)); }
                cr.evaluatorId
            }
            _ => return Err(PklError::DecodeError("expected EvaluatorCreated".to_string())),
        };

        let eval_pending: Arc<Mutex<HashMap<i64, oneshot::Sender<PklResult<ServerMessage>>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        self.inner.register_eval(evaluator_id, eval_pending.clone()).await;

        Ok(ServerEvaluator {
            inner: self.inner.clone(),
            eval_pending,
            evaluator_id,
            req_counter: AtomicI64::new(1),
            closed: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Close the manager and kill the server process.
    pub async fn close(&self) -> PklResult<()> {
        self.closed.store(true, Ordering::SeqCst);
        let mut child = self.inner.child.lock().await;
        if let Some(ref mut c) = *child { c.kill().await.ok(); c.wait().await.ok(); }
        *child = None;
        Ok(())
    }
}

// ── ServerEvaluator ──

/// An evaluator connected to a shared `pkl server` process.
///
/// Created via [`EvaluatorManager::new_evaluator`], or standalone with
/// [`ServerEvaluator::new`] (which creates its own manager internally).
pub struct ServerEvaluator {
    inner: Arc<ManagerInner>,
    eval_pending: Arc<Mutex<HashMap<i64, oneshot::Sender<PklResult<ServerMessage>>>>>,
    evaluator_id: i64,
    req_counter: AtomicI64,
    closed: Arc<AtomicBool>,
}

impl ServerEvaluator {
    /// Standalone: spawns a dedicated server process for this evaluator.
    pub async fn new() -> PklResult<Self> {
        let mgr = EvaluatorManager::new().await?;
        mgr.new_evaluator().await
    }

    /// Standalone with custom options.
    pub async fn with_options(opts: EvaluatorOptions) -> PklResult<Self> {
        let mgr = EvaluatorManager::with_options(opts).await?;
        mgr.new_evaluator().await
    }

    async fn evaluate_request(&self, source: &ModuleSource, expr: Option<String>) -> PklResult<Value> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(PklError::CliError("evaluator closed".to_string()));
        }
        let req_id = self.req_counter.fetch_add(1, Ordering::SeqCst);
        let msg = msgapi::EvaluateRequest {
            requestId: req_id,
            evaluatorId: self.evaluator_id,
            moduleUri: source.uri.clone(),
            moduleText: source.contents.clone(),
            expr,
        };
        let bytes = msgapi::encode_msg(msgapi::CODE_EVALUATE, &msg);

        let (tx, rx) = oneshot::channel();
        {
            let mut p = self.eval_pending.lock().await;
            p.insert(req_id, tx);
        }
        self.inner.send(bytes).await?;

        let resp = tokio::time::timeout(Duration::from_secs(30), rx).await
            .map_err(|_| PklError::CliError("timeout".to_string()))?
            .map_err(|_| PklError::CliError("channel closed".to_string()))??;

        match resp {
            ServerMessage::EvaluateDone(er) => {
                if !er.error.is_empty() { return Err(PklError::EvalError(er.error)); }
                decode::decode_response(&er.result)
            }
            _ => Err(PklError::DecodeError("expected EvaluateDone".to_string())),
        }
    }
}

#[async_trait]
impl PklEvaluator for ServerEvaluator {
    async fn evaluate<T: PklDecode>(&self, source: &ModuleSource) -> PklResult<T> {
        let value = self.evaluate_request(source, None).await?;
        T::decode(value)
    }

    async fn evaluate_raw(&self, source: &ModuleSource) -> PklResult<Value> {
        self.evaluate_request(source, None).await
    }

    async fn evaluate_text(&self, source: &ModuleSource) -> PklResult<String> {
        let value = self.evaluate_request(source, Some("output.text".to_string())).await?;
        match value {
            Value::String(s) => Ok(s),
            other => Ok(format!("{:?}", other)),
        }
    }

    async fn close(&self) -> PklResult<()> {
        self.closed.store(true, Ordering::SeqCst);
        self.inner.unregister_eval(self.evaluator_id).await;
        Ok(())
    }
}

// ── Message protocol helpers ──

fn try_decode_message(buf: &[u8]) -> Option<(usize, i64, Vec<u8>)> {
    use rmpv::decode::read_value;
    let mut cursor = std::io::Cursor::new(buf);
    let val = read_value(&mut cursor).ok()?;
    let consumed = cursor.position() as usize;
    let arr = val.as_array()?;
    if arr.len() < 2 { return None; }
    let code = arr[0].as_i64()?;

    let mut pos = 0;
    let marker = buf.get(pos)?;
    match marker {
        0x92 => pos += 1,
        0x90..=0x9f => pos += 1,
        0xdc => pos += 3,
        0xdd => pos += 5,
        _ => return None,
    }
    let cm = buf.get(pos)?;
    match cm {
        0x00..=0x7f => { pos += 1; }
        0xd0 => { pos += 2; }
        0xd1 => { pos += 3; }
        0xd2 => { pos += 5; }
        0xd3 => { pos += 9; }
        _ => return None,
    }
    Some((consumed, code, buf[pos..consumed].to_vec()))
}

async fn read_loop(stdout: tokio::process::ChildStdout, inner: Arc<ManagerInner>) {
    let mut reader = tokio::io::BufReader::new(stdout);
    let mut buf = Vec::new();
    let mut chunk = vec![0u8; 8192];

    loop {
        match reader.read(&mut chunk).await {
            Ok(0) => break,
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
            Err(e) => { eprintln!("pkl server read error: {}", e); break; }
        }
        loop {
            match try_decode_message(&buf) {
                Some((consumed, code, body_bytes)) => {
                    let _ = handle_message(code, &body_bytes, &inner).await;
                    buf.drain(..consumed);
                }
                None => break,
            }
            if buf.is_empty() { break; }
        }
    }
}

async fn handle_message(code: i64, body_bytes: &[u8], inner: &Arc<ManagerInner>) -> PklResult<()> {
    match code {
        msgapi::CODE_NEW_EVALUATOR_RESPONSE => {
            let resp: msgapi::CreateEvaluatorResponse = msgapi::decode_body(body_bytes)?;
            dispatch(&inner.create_pending, resp.requestId, Ok(ServerMessage::EvaluatorCreated(resp))).await;
        }
        msgapi::CODE_EVALUATE_RESPONSE => {
            let resp: msgapi::EvaluateResponse = msgapi::decode_body(body_bytes)?;
            // Route to the correct evaluator's pending map
            let eval_pendings = inner.eval_pendings.lock().await;
            if let Some(pending) = eval_pendings.get(&resp.evaluatorId) {
                dispatch(pending, resp.requestId, Ok(ServerMessage::EvaluateDone(resp))).await;
            }
        }
        msgapi::CODE_EVALUATE_LOG => { let _: msgapi::LogMessage = msgapi::decode_body(body_bytes)?; }
        msgapi::CODE_EVALUATE_READ_RESOURCE => {
            let req: msgapi::ReadResourceRequest = msgapi::decode_body(body_bytes)?;
            let result = handle_read_resource(&req, inner);
            let resp = msgapi::ReadResourceResponse {
                requestId: req.requestId, evaluatorId: req.evaluatorId,
                contents: result.contents, error: result.error,
            };
            inner.send(msgapi::encode_msg(msgapi::CODE_EVALUATE_READ_RESOURCE_RESPONSE, &resp)).await.ok();
        }
        msgapi::CODE_EVALUATE_READ_MODULE => {
            let req: msgapi::ReadModuleRequest = msgapi::decode_body(body_bytes)?;
            let result = handle_read_module(&req, inner);
            let resp = msgapi::ReadModuleResponse {
                requestId: req.requestId, evaluatorId: req.evaluatorId,
                contents: result.contents, error: result.error,
            };
            inner.send(msgapi::encode_msg(msgapi::CODE_EVALUATE_READ_MODULE_RESPONSE, &resp)).await.ok();
        }
        msgapi::CODE_INIT_MODULE_READER_REQUEST => {
            let req: msgapi::InitializeModuleReaderRequest = msgapi::decode_body(body_bytes)?;
            let resp = msgapi::InitializeModuleReaderResponse { requestId: req.requestId, spec: None };
            inner.send(msgapi::encode_msg(msgapi::CODE_INIT_MODULE_READER_RESPONSE, &resp)).await.ok();
        }
        msgapi::CODE_INIT_RESOURCE_READER_REQUEST => {
            let req: msgapi::InitializeResourceReaderRequest = msgapi::decode_body(body_bytes)?;
            let resp = msgapi::InitializeResourceReaderResponse { requestId: req.requestId, spec: None };
            inner.send(msgapi::encode_msg(msgapi::CODE_INIT_RESOURCE_READER_RESPONSE, &resp)).await.ok();
        }
        _ => {}
    }
    Ok(())
}

struct ReadResourceOut { contents: Option<Vec<u8>>, error: Option<String> }
fn handle_read_resource(req: &msgapi::ReadResourceRequest, inner: &ManagerInner) -> ReadResourceOut {
    for r in &inner.resource_readers {
        if req.uri.starts_with(&format!("{}:", r.scheme())) {
            return match r.read(&req.uri) {
                Ok(b) => ReadResourceOut { contents: Some(b), error: None },
                Err(e) => ReadResourceOut { contents: None, error: Some(e) },
            };
        }
    }
    ReadResourceOut { contents: None, error: Some(format!("No resource reader for {}", req.uri)) }
}

struct ReadModuleOut { contents: Option<String>, error: Option<String> }
fn handle_read_module(req: &msgapi::ReadModuleRequest, inner: &ManagerInner) -> ReadModuleOut {
    for r in &inner.module_readers {
        if req.uri.starts_with(&format!("{}:", r.scheme())) {
            return match r.read(&req.uri) {
                Ok(s) => ReadModuleOut { contents: Some(s), error: None },
                Err(e) => ReadModuleOut { contents: None, error: Some(e) },
            };
        }
    }
    ReadModuleOut { contents: None, error: None }
}

async fn dispatch(
    pending: &Mutex<HashMap<i64, oneshot::Sender<PklResult<ServerMessage>>>>,
    request_id: i64,
    msg: PklResult<ServerMessage>,
) {
    let mut map = pending.lock().await;
    if let Some(tx) = map.remove(&request_id) {
        tx.send(msg).ok();
    }
}
