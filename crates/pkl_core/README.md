# pkl_core

Core runtime library for embedding the [Pkl configuration language](https://pkl-lang.org) into Rust applications.

## Features

- **`PklDecode` trait** — deserialize Pkl values into Rust types
- **`PklEvaluator` trait** — abstract over evaluation backends
- **`CliEvaluator`** — spawns `pkl eval` as a subprocess
- **`ServerEvaluator`** — persistent `pkl server` connection (10x faster)
- **`EvaluatorManager`** — share one server process across evaluators
- **External readers** — register custom `ResourceReader`/`ModuleReader` for `read("scheme:...")`
- **Project support** — load `PklProject` files and resolve dependencies
- **Full type coverage** — `Value`, `Duration`, `DataSize`, all Pkl collection types

## Usage

```toml
[dependencies]
pkl_core = "0.1"
pkl_macros = "0.1"
tokio = { version = "1", features = ["rt", "macros"] }
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

| Backend | When to use |
|---------|-------------|
| `CliEvaluator` | Simple, one-off evaluations |
| `ServerEvaluator` | Repeated evaluations, 10x faster |
| `EvaluatorManager` | Shared process, multiple evaluators |

## Derive macro

The `#[derive(PklDecode)]` macro is re-exported from `pkl_macros` and supports:

- `#[pkl(rename = "...")]`, `#[pkl(default)]`, `#[pkl(skip)]`, `#[pkl(flatten)]`
- String literal unions, untagged unions, tagged unions (`#[pkl(tag = "field")]`)

## Requirements

Requires the `pkl` CLI binary. See [pkl-lang.org](https://pkl-lang.org) to install.

License: Apache 2.0
