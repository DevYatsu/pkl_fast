# pkl_codegen

Build-time code generation for Pkl configuration files. Parses `.pkl` source files and generates type-safe Rust structs with `PklDecode` derives.

## Usage in build.rs

```toml
[build-dependencies]
pkl_codegen = "0.1"
```

```rust
// build.rs
fn main() {
    pkl_codegen::generate("schemas/config.pkl", "src/gen/config.rs").unwrap();
}
```

```rust
// src/main.rs
include!("gen/config.rs");

let cfg: Root = pkl_core::CliEvaluator::new()
    .evaluate(&pkl_core::ModuleSource::from_file("config.pkl")).await?;
```

## What it generates

| Pkl construct | Rust output |
|---------------|-------------|
| `class Name { ... }` | `struct Name` with `#[derive(PklDecode)]` |
| `typealias X = Y` | `type X = Y` |
| `typealias X = "a" \| "b"` | `enum X { A, B }` with `PklDecode` impl |
| Module properties | `struct Root` |
| `String?` | `Option<String>` |
| `List<T>` | `Vec<T>` |
| `Map<K,V>` | `BTreeMap<K,V>` |

## API

```rust
// Parse Pkl source and generate code in one step
pkl_codegen::generate("input.pkl", "output.rs")?;

// Or split for custom processing
let module = pkl_codegen::parse_module(source)?;
let code = pkl_codegen::generate_code(&module);
```

License: Apache 2.0
