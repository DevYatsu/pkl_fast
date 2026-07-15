# pkl-bindgen

Rust bindings for the [Pkl configuration language](https://pkl-lang.org). Port of Apple's [pkl-go](https://github.com/apple/pkl-go).

> Requires the `pkl` CLI. See [pkl-lang.org](https://pkl-lang.org) to install.

## Install

```bash
cargo install --git https://github.com/DevYatsu/pkl_bindgen.git
```

## Quickstart

```toml
[dependencies]
pkl_core = "0.1"
pkl_macros = "0.1"
tokio = { version = "1", features = ["rt", "macros"] }
```

```bash
# CLI eval
pkl-bindgen eval config.pkl

# Inline eval
pkl-bindgen expr 'name = "Hello, Pkl!"'

# Generate Rust code from .pkl
pkl-bindgen generate schema.pkl -o gen.rs
```

```rust
use pkl_core::{PklDecode, PklEvaluator, CliEvaluator, ModuleSource};

#[derive(PklDecode)]
struct Config { host: String, port: u16 }

#[tokio::main]
async fn main() -> Result<(), pkl_core::PklError> {
    let cfg: Config = CliEvaluator::new()
        .evaluate(&ModuleSource::from_file("config.pkl")).await?;
    println!("{}:{}", cfg.host, cfg.port);
    Ok(())
}
```

## Evaluators

```rust
// Simple (spawns pkl eval)
let e = CliEvaluator::new();

// Persistent server (10x faster repeated eval)
let e = ServerEvaluator::new().await?;

// Shared server process
let mgr = EvaluatorManager::new().await?;
let ev1 = mgr.new_evaluator().await?;
let ev2 = mgr.new_evaluator().await?;
```

## Derive macro

```rust
#[derive(PklDecode)]
struct Config {
    name: String,
    #[pkl(rename = "dbHost")]  db_host: String,
    #[pkl(default)]             port: u16,
    #[pkl(skip)]                computed: String,
    #[pkl(flatten)]             extra: BTreeMap<String, Value>,
}

#[derive(PklDecode)]
enum Diet { Seeds, Berries, Insects }

#[derive(PklDecode)]
#[pkl(tag = "type")]
enum Shape {
    Circle { radius: f64 },
    #[pkl(tag = "rect")] Rect { width: f64, height: f64 },
}
```

## Codegen

```rust
// build.rs
fn main() { pkl_codegen::generate("config.pkl", "src/gen/config.rs").unwrap(); }
```

```rust
include!("gen/config.rs");
let cfg: Root = CliEvaluator::new()
    .evaluate(&ModuleSource::from_file("config.pkl")).await?;
```

## External readers

```rust
impl ResourceReader for MyReader {
    fn scheme(&self) -> &str { "secret" }
    fn read(&self, uri: &str) -> Result<Vec<u8>, String> { Ok(b"data".to_vec()) }
}
let e = ServerEvaluator::with_options(EvaluatorOptions {
    resource_readers: vec![Box::new(MyReader)],
    ..Default::default()
}).await?;
// read("secret:key") → "data"
```

Repo: [github.com/DevYatsu/pkl_bindgen](https://github.com/DevYatsu/pkl_bindgen.git)
License: Apache 2.0
