# pkl_macros

Derive macros for `pkl_core`. Provides `#[derive(PklDecode)]` for automatic deserialization of Pkl values into Rust structs and enums.

Typically used via the re-export in `pkl_core` — you don't need to depend on this crate directly:

```toml
[dependencies]
pkl_core = "0.1"  # re-exports PklDecode derive macro
```

## Attributes

### Struct fields

| Attribute | Description |
|-----------|-------------|
| `#[pkl(rename = "name")]` | Override Pkl property name |
| `#[pkl(default)]` | Use `Default::default()` when missing |
| `#[pkl(skip)]` | Skip this field entirely |
| `#[pkl(flatten)]` | Capture remaining props into `BTreeMap<String, Value>` |

### Enum

```rust
#[derive(PklDecode)]
#[pkl(tag = "type")]
enum Shape {
    Circle { radius: f64 },
    #[pkl(tag = "rect")] Rect { width: f64, height: f64 },
}
```

License: Apache 2.0
