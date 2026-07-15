# pkl-bindgen

Rust bindings for the [Pkl configuration language](https://pkl-lang.org). Port of Apple's [pkl-go](https://github.com/apple/pkl-go).

> Requires the `pkl` CLI. See [pkl-lang.org](https://pkl-lang.org) to install.

## Choose your path

### CLI — evaluate and generate from the terminal

```bash
cargo install --git https://github.com/DevYatsu/pkl_bindgen.git

pkl-bindgen eval config.pkl
pkl-bindgen expr 'name = "Hello, Pkl!"'
pkl-bindgen generate schema.pkl -o gen.rs
```

### Library — embed in your Rust project

```toml
[dependencies]
pkl_core = "0.2"
pkl_macros = "0.2"
tokio = { version = "1", features = ["rt", "macros"] }

[build-dependencies]
pkl_codegen = "0.2"
```

Define a Pkl schema:

```pkl
# config.pkl
host: String
port: UInt16
timeout: Duration?
```

Two options to use it in Rust:

**With codegen (recommended)** — structs are auto-generated from your `.pkl` file. No manual struct definitions, no drift between schema and code:

```rust
// build.rs
fn main() { pkl_codegen::generate("config.pkl", "src/gen/config.rs").unwrap(); }
```

```rust
// Just include — struct is auto-generated with correct types, optionals, etc.
include!("gen/config.rs");

let cfg: Root = CliEvaluator::new()
    .evaluate(&ModuleSource::from_file("config.pkl")).await?;
println!("{}:{}", cfg.host, cfg.port);
```

**Without codegen** — define structs manually with the derive macro:

```rust
#[derive(PklDecode)]
struct Config {
    host: String,
    port: u16,
    timeout: Option<Duration>,
}

let cfg: Config = CliEvaluator::new()
    .evaluate(&ModuleSource::from_file("config.pkl")).await?;
```

## Evaluators

```rust
// Simple (spawns pkl eval per call)
let e = CliEvaluator::new();

// Persistent server (10x faster repeated eval)
let e = ServerEvaluator::new().await?;

// Shared server process — multiple evaluators, one connection
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
