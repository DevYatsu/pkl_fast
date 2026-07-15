use pkl_core::*;

// ── CliEvaluator tests ──

#[tokio::test]
async fn test_evaluate_raw() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let v = e.evaluate_raw(&ModuleSource::from_text("x = 42")).await.unwrap();
    let obj = v.as_object().unwrap();
    assert_eq!(obj.get("x").unwrap().as_int().unwrap(), 42);
}

// Ported from pkl-go: EvaluateOutputText
#[tokio::test]
async fn test_evaluate_text() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let text = e.evaluate_text(&ModuleSource::from_text(
        r#"output { text = "hello" }"#,
    )).await.unwrap();
    assert!(text.contains("hello"));
}

// Ported from pkl-go: EvaluateOutputText with YAML renderer
#[tokio::test]
async fn test_evaluate_text_yaml() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    // evaluate_text evaluates `output.text` expression
    let text = e.evaluate_text(&ModuleSource::from_text(
        r#"name = "Pigeon"
output {
  renderer = new YamlRenderer {}
  value = name
}
"#,
    )).await.unwrap();
    assert!(text.contains("Pigeon"));
}

// Ported from pkl-go: Evaluate into typed struct
#[tokio::test]
async fn test_evaluate_typed() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }

    #[derive(PklDecode, Debug, PartialEq)]
    struct Person {
        name: String,
        age: i64,
    }

    let e = CliEvaluator::new();
    let p: Person = e.evaluate(&ModuleSource::from_text(
        r#"name = "Pidgeon"
age = 2
"#,
    )).await.unwrap();

    assert_eq!(p.name, "Pidgeon");
    assert_eq!(p.age, 2);
}

// Ported from pkl-go: Evaluate after close should error
#[tokio::test]
async fn test_evaluate_after_close() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    e.close().await.unwrap();
    let result = e.evaluate_raw(&ModuleSource::from_text("x = 1")).await;
    assert!(result.is_err(), "evaluating after close should error");
}

// Ported from pkl-go: Evaluate expression
#[tokio::test]
async fn test_evaluate_expression() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    // Evaluate with `read("env:HOME")` expression
    let v = e.evaluate_raw(&ModuleSource::from_text(
        r#"home = read("env:HOME")"#,
    )).await.unwrap();
    let obj = v.as_object().unwrap();
    let home = obj.get("home").unwrap().as_str().unwrap();
    assert!(!home.is_empty(), "HOME should be set");
    assert!(home.starts_with("/"), "HOME should be an absolute path");
}

// Ported from pkl-go: Nullable reads
#[tokio::test]
async fn test_evaluate_nullable_read() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let v = e.evaluate_raw(&ModuleSource::from_text(
        r#"port = read?("env:NONEXISTENT_PORT")?.toInt() ?? 8080"#,
    )).await.unwrap();
    let obj = v.as_object().unwrap();
    assert_eq!(obj.get("port").unwrap().as_int().unwrap(), 8080);
}

// ── ServerEvaluator tests ──

#[tokio::test]
async fn test_server_basic() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = ServerEvaluator::new().await.unwrap();
    let v = e.evaluate_raw(&ModuleSource::from_text("x = 42")).await.unwrap();
    let obj = v.as_object().unwrap();
    assert_eq!(obj.get("x").unwrap().as_int().unwrap(), 42);
    e.close().await.unwrap();
}

#[tokio::test]
async fn test_server_typed() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }

    #[derive(PklDecode, Debug, PartialEq)]
    struct Cfg {
        name: String,
        count: i64,
    }

    let e = ServerEvaluator::new().await.unwrap();
    let cfg: Cfg = e.evaluate(&ModuleSource::from_text(
        r#"name = "test"
count = 99
"#,
    )).await.unwrap();
    assert_eq!(cfg.name, "test");
    assert_eq!(cfg.count, 99);
    e.close().await.unwrap();
}

#[tokio::test]
async fn test_server_multiple_evaluations() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = ServerEvaluator::new().await.unwrap();
    for i in 0..10 {
        let v = e.evaluate_raw(&ModuleSource::from_text(&format!("n = {}", i))).await.unwrap();
        assert_eq!(v.as_object().unwrap().get("n").unwrap().as_int().unwrap(), i);
    }
    e.close().await.unwrap();
}

#[tokio::test]
async fn test_server_text() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = ServerEvaluator::new().await.unwrap();
    let text = e.evaluate_text(&ModuleSource::from_text(
        r#"output { text = "hello server" }"#,
    )).await.unwrap();
    assert!(text.contains("hello server"));
    e.close().await.unwrap();
}

#[tokio::test]
async fn test_server_after_close() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = ServerEvaluator::new().await.unwrap();
    e.close().await.unwrap();
    let result = e.evaluate_raw(&ModuleSource::from_text("x = 1")).await;
    assert!(result.is_err(), "evaluating after close should error");
}

// ── Boolean tests ──

#[tokio::test]
async fn test_evaluate_bool() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let v = e.evaluate_raw(&ModuleSource::from_text(
        r#"a = true
b = false
c = true && false
d = true || false
"#,
    )).await.unwrap();
    let obj = v.as_object().unwrap();
    assert_eq!(obj.get("a").unwrap().as_bool(), Some(true));
    assert_eq!(obj.get("b").unwrap().as_bool(), Some(false));
    assert_eq!(obj.get("c").unwrap().as_bool(), Some(false));
    assert_eq!(obj.get("d").unwrap().as_bool(), Some(true));
}

// ── Collection evaluation tests ──

#[tokio::test]
async fn test_evaluate_list() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let v = e.evaluate_raw(&ModuleSource::from_text(
        r#"items = List(1, 2, 3, 4, 5)"#,
    )).await.unwrap();
    let obj = v.as_object().unwrap();
    let items = obj.get("items").unwrap().as_list().unwrap();
    assert_eq!(items.len(), 5);
    assert_eq!(items[0].as_int().unwrap(), 1);
    assert_eq!(items[4].as_int().unwrap(), 5);
}

#[tokio::test]
async fn test_evaluate_set() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let v = e.evaluate_raw(&ModuleSource::from_text(
        r#"items = Set(1, 2, 3, 3, 2)"#,
    )).await.unwrap();
    let obj = v.as_object().unwrap();
    match obj.get("items").unwrap() {
        Value::Set(items) => assert_eq!(items.len(), 3),
        other => panic!("Expected Set, got {:?}", other),
    }
}

#[tokio::test]
async fn test_evaluate_map() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let v = e.evaluate_raw(&ModuleSource::from_text(
        r#"m = Map("a", 1, "b", 2, "c", 3)"#,
    )).await.unwrap();
    let obj = v.as_object().unwrap();
    match obj.get("m").unwrap() {
        Value::Map(pairs) => {
            assert_eq!(pairs.len(), 3);
        }
        other => panic!("Expected Map, got {:?}", other),
    }
}

// ── Error handling tests —─

#[tokio::test]
async fn test_evaluate_type_error() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let result = e.evaluate_raw(&ModuleSource::from_text(
        r#"
class Bird { name: String }
b = new Bird { name = 42 }
"#,
    )).await;
    assert!(result.is_err(), "type error should error");
}

#[tokio::test]
async fn test_evaluate_syntax_error() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let e = CliEvaluator::new();
    let result = e.evaluate_raw(&ModuleSource::from_text(
        r#"this is not valid pkl syntax @@@"#,
    )).await;
    assert!(result.is_err(), "syntax error should error");
}

// ── External reader tests ──

use std::collections::HashMap;
use pkl_core::reader::{ResourceReader, ModuleReader, PathElement};

struct TestResourceReader {
    data: HashMap<String, Vec<u8>>,
}

impl ResourceReader for TestResourceReader {
    fn scheme(&self) -> &str { "testres" }
    fn read(&self, uri: &str) -> Result<Vec<u8>, String> {
        let key = uri.strip_prefix("testres:").unwrap_or(uri);
        self.data.get(key).cloned().ok_or_else(|| format!("resource not found: {}", key))
    }
    fn is_globbable(&self) -> bool { false }
    fn has_hierarchical_uris(&self) -> bool { false }
    fn list_elements(&self, _uri: &str) -> Result<Vec<PathElement>, String> {
        Err("not supported".to_string())
    }
}

struct TestModuleReader {
    modules: HashMap<String, String>,
}

impl ModuleReader for TestModuleReader {
    fn scheme(&self) -> &str { "testmod" }
    fn read(&self, uri: &str) -> Result<String, String> {
        let key = uri.strip_prefix("testmod:").unwrap_or(uri);
        self.modules.get(key).cloned().ok_or_else(|| format!("module not found: {}", key))
    }
    fn is_local(&self) -> bool { true }
    fn is_globbable(&self) -> bool { false }
    fn has_hierarchical_uris(&self) -> bool { false }
    fn list_elements(&self, _uri: &str) -> Result<Vec<PathElement>, String> {
        Err("not supported".to_string())
    }
}

#[tokio::test]
async fn test_external_resource_reader() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }

    let mut data = HashMap::new();
    data.insert("greeting".to_string(), b"Hello, World!".to_vec());

    let opts = pkl_core::EvaluatorOptions {
        resource_readers: vec![Box::new(TestResourceReader { data })],
        ..Default::default()
    };

    let e = ServerEvaluator::with_options(opts).await.unwrap();
    let v = e.evaluate_raw(&ModuleSource::from_text(
        r#"greeting = read("testres:greeting").text"#,
    )).await.unwrap();
    let obj = v.as_object().unwrap();
    assert_eq!(obj.get("greeting").unwrap().as_str().unwrap(), "Hello, World!");
    e.close().await.unwrap();
}

// ── EvaluatorManager tests ──

#[tokio::test]
async fn test_manager_multiple_evaluators() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let mgr = EvaluatorManager::new().await.unwrap();

    let ev1 = mgr.new_evaluator().await.unwrap();
    let ev2 = mgr.new_evaluator().await.unwrap();

    let v1 = ev1.evaluate_raw(&ModuleSource::from_text("x = 1")).await.unwrap();
    let v2 = ev2.evaluate_raw(&ModuleSource::from_text("x = 2")).await.unwrap();

    assert_eq!(v1.as_object().unwrap().get("x").unwrap().as_int().unwrap(), 1);
    assert_eq!(v2.as_object().unwrap().get("x").unwrap().as_int().unwrap(), 2);

    // Both evaluators share the same server, but can close independently
    ev1.close().await.unwrap();
    let v3 = ev2.evaluate_raw(&ModuleSource::from_text("x = 3")).await.unwrap();
    assert_eq!(v3.as_object().unwrap().get("x").unwrap().as_int().unwrap(), 3);

    ev2.close().await.unwrap();
    mgr.close().await.unwrap();
}

#[tokio::test]
async fn test_manager_concurrent_evaluations() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let mgr = EvaluatorManager::new().await.unwrap();
    let ev1 = mgr.new_evaluator().await.unwrap();
    let ev2 = mgr.new_evaluator().await.unwrap();

    // Concurrent evaluations from different evaluators
    let s1 = ModuleSource::from_text("n = 100");
    let s2 = ModuleSource::from_text("n = 200");
    let (r1, r2) = tokio::join!(
        ev1.evaluate_raw(&s1),
        ev2.evaluate_raw(&s2),
    );
    assert_eq!(r1.unwrap().as_object().unwrap().get("n").unwrap().as_int().unwrap(), 100);
    assert_eq!(r2.unwrap().as_object().unwrap().get("n").unwrap().as_int().unwrap(), 200);

    ev1.close().await.unwrap();
    ev2.close().await.unwrap();
    mgr.close().await.unwrap();
}

#[tokio::test]
async fn test_manager_close_then_eval() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let mgr = EvaluatorManager::new().await.unwrap();
    mgr.close().await.unwrap();
    let result = mgr.new_evaluator().await;
    assert!(result.is_err(), "creating evaluator after manager close should error");
}

/// Module readers need Pkl 0.33+ for full client-side module reader support.
/// Skipping in 0.32.0 — the server doesn't send InitializeModuleReaderRequest.
#[tokio::test]
#[ignore = "Requires Pkl 0.33+ for client-side module reader protocol"]
async fn test_external_module_reader() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }
    let mut modules = HashMap::new();
    modules.insert("config.pkl".to_string(), r#"name = "from custom module""#.to_string());
    let opts = pkl_core::EvaluatorOptions {
        module_readers: vec![Box::new(TestModuleReader { modules })],
        ..Default::default()
    };
    let e = ServerEvaluator::with_options(opts).await.unwrap();
    let result = e.evaluate_raw(&ModuleSource::from_text(
        r#"import "testmod:config.pkl"
name = config.name
"#,
    )).await;
    // This is expected to fail in Pkl 0.32.x
    assert!(result.is_err(), "module reader test requires Pkl 0.33+");
    e.close().await.unwrap();
}

#[tokio::test]
async fn test_external_resource_reader_text_property() {
    if std::process::Command::new("pkl").arg("--version").output().is_err() {
        eprintln!("Skipping: pkl CLI not found");
        return;
    }

    let mut data = HashMap::new();
    data.insert("hello".to_string(), b"world".to_vec());

    let opts = pkl_core::EvaluatorOptions {
        resource_readers: vec![Box::new(TestResourceReader { data })],
        ..Default::default()
    };

    let e = ServerEvaluator::with_options(opts).await.unwrap();
    let v = e.evaluate_raw(&ModuleSource::from_text(
        r#"
res = read("testres:hello")
text = res.text
uri = res.uri
"#,
    )).await.unwrap();
    let obj = v.as_object().unwrap();
    assert_eq!(obj.get("text").unwrap().as_str().unwrap(), "world");
    assert_eq!(obj.get("uri").unwrap().as_str().unwrap(), "testres:hello");
    e.close().await.unwrap();
}
