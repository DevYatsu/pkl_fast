use std::process::{Command, Stdio};
use std::io::{Write, Read};
use pkl_core::msgapi::{self, ServerMessage};

/// Test the raw server protocol synchronously.
#[test]
fn test_server_protocol_raw() {
    if Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }

    // Encode CreateEvaluator message the same way pkl-go does it
    // [0x20, {requestId: 1, allowedModules: [...], allowedResources: [...], outputFormat: "pkl-binary"}]
    let msg = msgapi::CreateEvaluator {
        requestId: 1,
        clientResourceReaders: None,
        clientModuleReaders: None,
        allowedModules: Some(vec!["file:".into(), "repl:".into()]),
        allowedResources: Some(vec!["file:".into(), "env:".into()]),
        outputFormat: Some("pkl-binary".into()),
        project: None,
    };
    let bytes = msgapi::encode_msg(msgapi::CODE_NEW_EVALUATOR, &msg);
    println!("Sending {} bytes: {:02x?}", bytes.len(), bytes);

    // Also show what rmp_serde produces as a tuple
    let tuple_bytes = rmp_serde::to_vec(&(0x20i64, &serde_json::json!({"requestId": 1}))).unwrap();
    println!("Tuple test: {:02x?}", tuple_bytes);

    // Start server
    let mut child = Command::new("pkl")
        .arg("server")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn pkl server");
    
    println!("Child PID: {}", child.id());

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(&bytes).unwrap();
    stdin.flush().unwrap();
    drop(stdin);

    // Read stderr first (in case server writes error there)
    let mut stderr = child.stderr.take().unwrap();
    let mut err_buf = String::new();
    let stderr_handle = std::thread::spawn(move || {
        let mut reader = std::io::BufReader::new(stderr);
        let mut stderr_output = String::new();
        reader.read_to_string(&mut stderr_output).ok();
        stderr_output
    });

    // Read stdout
    let mut stdout = child.stdout.take().unwrap();
    let mut buf = vec![0u8; 8192];
    let n = stdout.read(&mut buf).expect("read stdout");
    println!("Got {} bytes from server stdout", n);
    if n > 0 {
        println!("Hex: {:02x?}", &buf[..n.min(64)]);
        match pkl_core::msgapi::decode_message(&buf[..n]) {
            Ok(msg) => println!("Decoded: {:?}", msg),
            Err(e) => println!("Decode error: {}", e),
        }
    }

    child.wait().ok();

    let stderr_output = stderr_handle.join().unwrap();
    if !stderr_output.is_empty() {
        let first_line = stderr_output.lines().next().unwrap_or("");
        println!("Stderr first line: {}", first_line);
        if stderr_output.len() > 200 {
            println!("Stderr truncated: {}...", &stderr_output[..200]);
        }
    }
}
