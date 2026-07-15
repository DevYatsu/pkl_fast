//! Message types for the Pkl server protocol.
//!
//! The `pkl server` subprocess communicates using MessagePack.
//! Each message is a 2-element array: `[code: int, body: object]`.

use serde::{Deserialize, Serialize};

// ── Protocol codes ──

pub const CODE_NEW_EVALUATOR: i64 = 0x20;
pub const CODE_NEW_EVALUATOR_RESPONSE: i64 = 0x21;
pub const CODE_CLOSE_EVALUATOR: i64 = 0x22;
pub const CODE_EVALUATE: i64 = 0x23;
pub const CODE_EVALUATE_RESPONSE: i64 = 0x24;
pub const CODE_EVALUATE_LOG: i64 = 0x25;
pub const CODE_EVALUATE_READ_RESOURCE: i64 = 0x26;
pub const CODE_EVALUATE_READ_RESOURCE_RESPONSE: i64 = 0x27;
pub const CODE_EVALUATE_READ_MODULE: i64 = 0x28;
pub const CODE_EVALUATE_READ_MODULE_RESPONSE: i64 = 0x29;
pub const CODE_LIST_RESOURCES_REQUEST: i64 = 0x2a;
pub const CODE_LIST_RESOURCES_RESPONSE: i64 = 0x2b;
pub const CODE_LIST_MODULES_REQUEST: i64 = 0x2c;
pub const CODE_LIST_MODULES_RESPONSE: i64 = 0x2d;
pub const CODE_INIT_MODULE_READER_REQUEST: i64 = 0x2e;
pub const CODE_INIT_MODULE_READER_RESPONSE: i64 = 0x2f;
pub const CODE_INIT_RESOURCE_READER_REQUEST: i64 = 0x30;
pub const CODE_INIT_RESOURCE_READER_RESPONSE: i64 = 0x31;
pub const CODE_CLOSE_EXTERNAL_PROCESS: i64 = 0x32;

// ── Project/Dependency types ──

#[derive(Debug, Serialize)]
pub struct ProjectOrDependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packageUri: Option<String>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectFileUri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksums: Option<Checksums>,
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub dependencies: std::collections::BTreeMap<String, ProjectOrDependency>,
}

#[derive(Debug, Serialize)]
pub struct Checksums {
    pub sha256: String,
}

// ── Outgoing messages (host → server) ──

#[derive(Debug, Serialize)]
pub struct CreateEvaluator {
    pub requestId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientResourceReaders: Option<Vec<ResourceReaderSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientModuleReaders: Option<Vec<ModuleReaderSpec>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowedModules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowedResources: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputFormat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectOrDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceReaderSpec {
    pub scheme: String,
    #[serde(default)]
    pub hasHierarchicalUris: bool,
    #[serde(default)]
    pub isGlobbable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleReaderSpec {
    pub scheme: String,
    #[serde(default)]
    pub hasHierarchicalUris: bool,
    #[serde(default)]
    pub isGlobbable: bool,
    #[serde(default)]
    pub isLocal: bool,
}

#[derive(Debug, Serialize)]
pub struct EvaluateRequest {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub moduleUri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moduleText: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadResourceResponse {
    pub requestId: i64,
    pub evaluatorId: i64,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    pub contents: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadModuleResponse {
    pub requestId: i64,
    pub evaluatorId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ── Incoming messages (server → host) ──

#[derive(Debug, Deserialize)]
pub struct CreateEvaluatorResponse {
    pub requestId: i64,
    pub evaluatorId: i64,
    #[serde(default)]
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateResponse {
    pub requestId: i64,
    pub evaluatorId: i64,
    #[serde(default)]
    pub result: Vec<u8>,
    #[serde(default)]
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct LogMessage {
    pub evaluatorId: i64,
    pub level: i64,
    pub message: String,
    pub frameUri: String,
}

#[derive(Debug, Deserialize)]
pub struct ReadResourceRequest {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct ReadModuleRequest {
    pub requestId: i64,
    pub evaluatorId: i64,
    pub uri: String,
}

/// A fully decoded message from the server.
#[derive(Debug, Deserialize)]
pub struct InitializeModuleReaderRequest {
    pub requestId: i64,
    pub scheme: String,
}

#[derive(Debug, Serialize)]
pub struct InitializeModuleReaderResponse {
    pub requestId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ModuleReaderSpec>,
}

#[derive(Debug, Deserialize)]
pub struct InitializeResourceReaderRequest {
    pub requestId: i64,
    pub scheme: String,
}

#[derive(Debug, Serialize)]
pub struct InitializeResourceReaderResponse {
    pub requestId: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ResourceReaderSpec>,
}

#[derive(Debug)]
pub enum ServerMessage {
    EvaluatorCreated(CreateEvaluatorResponse),
    EvaluateDone(EvaluateResponse),
    Log(LogMessage),
    ReadResource(ReadResourceRequest),
    ReadModule(ReadModuleRequest),
    CloseExternalProcess,
}

// ── Encoding helpers ──

/// Encode an outgoing message: `[code, body]` as MessagePack.
/// Uses named (map) representation so field names are preserved for the body.
pub fn encode_msg<T: Serialize>(code: i64, body: &T) -> Vec<u8> {
    rmp_serde::to_vec_named(&(code, body)).unwrap()
}

/// Decode a raw server message body (the second element of the array).
/// The caller peeks at the code first.
pub fn decode_body<T: serde::de::DeserializeOwned>(body_bytes: &[u8]) -> Result<T, crate::error::PklError> {
    rmp_serde::from_slice(body_bytes)
        .map_err(|e| crate::error::PklError::DecodeError(format!("decode: {}", e)))
}

/// Decode a complete server message from raw MessagePack bytes.
/// Expects format: `[code: int, body: object]`
pub fn decode_message(data: &[u8]) -> Result<ServerMessage, crate::error::PklError> {
    use crate::error::PklError;

    // Manual MessagePack parsing to extract code and body bytes
    let total_len = data.len();
    if total_len < 2 {
        return Err(PklError::DecodeError("message too short".to_string()));
    }

    let mut pos = 0;
    
    // Read array header
    let marker = data[pos];
    pos += 1;
    let array_len = match marker {
        0x92 => 2,  // fixarray(2) — our protocol always uses this
        0xdc => { // array 16
            if total_len < pos + 2 { return Err(PklError::DecodeError("truncated array16".to_string())); }
            let _ = u16::from_be_bytes([data[pos], data[pos+1]]);
            pos += 2;
            0
        }
        0xdd => { // array 32
            if total_len < pos + 4 { return Err(PklError::DecodeError("truncated array32".to_string())); }
            let _ = u32::from_be_bytes([data[pos], data[pos+1], data[pos+2], data[pos+3]]);
            pos += 4;
            0
        }
        0x90..=0x9f => marker as u64 - 0x90, // fixarray n
        _ => return Err(PklError::DecodeError(format!("expected array, got 0x{:02x}", marker))),
    };
    // We only support arrays of length >= 2
    if array_len < 2 {
        return Err(PklError::DecodeError(format!("array too short: {}", array_len)));
    }

    // Read the code (positive fixint 0x00-0x7f or int8/int16/int32)
    if pos >= total_len {
        return Err(PklError::DecodeError("truncated code".to_string()));
    }
    let code_marker = data[pos];
    let code = match code_marker {
        0x00..=0x7f => {
            pos += 1;
            code_marker as i64
        }
        0xd0 => { // int8
            if total_len < pos + 2 { return Err(PklError::DecodeError("truncated int8".to_string())); }
            let val = data[pos + 1] as i8 as i64;
            pos += 2;
            val
        }
        0xd1 => { // int16
            if total_len < pos + 3 { return Err(PklError::DecodeError("truncated int16".to_string())); }
            let val = i16::from_be_bytes([data[pos+1], data[pos+2]]) as i64;
            pos += 3;
            val
        }
        0xd2 => { // int32
            if total_len < pos + 5 { return Err(PklError::DecodeError("truncated int32".to_string())); }
            let val = i32::from_be_bytes([data[pos+1], data[pos+2], data[pos+3], data[pos+4]]) as i64;
            pos += 5;
            val
        }
        _ => return Err(PklError::DecodeError(format!("unexpected code marker 0x{:02x}", code_marker))),
    };

    // Body is everything from pos to end
    let body_bytes = &data[pos..];

    match code {
        CODE_NEW_EVALUATOR_RESPONSE => {
            let resp: CreateEvaluatorResponse = rmp_serde::from_slice(body_bytes)
                .map_err(|e| PklError::DecodeError(format!("create resp: {}", e)))?;
            Ok(ServerMessage::EvaluatorCreated(resp))
        }
        CODE_EVALUATE_RESPONSE => {
            let resp: EvaluateResponse = rmp_serde::from_slice(body_bytes)
                .map_err(|e| PklError::DecodeError(format!("eval resp: {}", e)))?;
            Ok(ServerMessage::EvaluateDone(resp))
        }
        CODE_EVALUATE_LOG => {
            let log: LogMessage = rmp_serde::from_slice(body_bytes)
                .map_err(|e| PklError::DecodeError(format!("log: {}", e)))?;
            Ok(ServerMessage::Log(log))
        }
        CODE_EVALUATE_READ_RESOURCE => {
            let req: ReadResourceRequest = rmp_serde::from_slice(body_bytes)
                .map_err(|e| PklError::DecodeError(format!("read resource: {}", e)))?;
            Ok(ServerMessage::ReadResource(req))
        }
        CODE_EVALUATE_READ_MODULE => {
            let req: ReadModuleRequest = rmp_serde::from_slice(body_bytes)
                .map_err(|e| PklError::DecodeError(format!("read module: {}", e)))?;
            Ok(ServerMessage::ReadModule(req))
        }
        CODE_CLOSE_EXTERNAL_PROCESS => Ok(ServerMessage::CloseExternalProcess),
        other => Err(PklError::DecodeError(format!("unknown code: 0x{:02x}", other))),
    }
}
