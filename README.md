# pkl-parser

This crate exports a parser for Apple's Pkl language.

## Features

All the Pkl syntax is supported.

## Installation

When in your rust project, simply run: `cargo add pkl-parser`

## Usage

Here's an example of how to parse a PKL string and retrieve values from the context:

```rust
use pkl_parser::{Pkl, PklResult, PklValue};

fn main() -> PklResult<()> {
    let source = r#"
    bool_var = true
    int_var = 42
    float_var = 3.14
    $string_var = "hello"
    object_var {
        key1 = "value1"
        key2 = 2
    }
    "#;

    let mut pkl = Pkl::new();
    pkl.parse(source)?;

    println!("{:?}", pkl.get("int_var")); // Ok(PklValue::Int(42))

    // Get values
    println!("{:?}", pkl.get_bool("bool_var")); // Ok(true)
    println!("{:?}", pkl.get_int("int_var")); // Ok(42)
    println!("{:?}", pkl.get_float("float_var")); // Ok(3.14)
    println!("{:?}", pkl.get_string("$string_var")); // Ok("hello")
    println!("{:?}", pkl.get_object("object_var")); // Ok(HashMap with key1 and key2)

    // Modify values
    pkl.set("int_var", PklValue::Int(100));

    // Remove values
    pkl.remove("float_var");
    println!("{:?}", pkl.get_float("float_var")); // Err("Variable `float_var` not found")

    // Or just generate an ast
    let mut pkl = Pkl::new();
    // the ast contains the start and end indexes of each value and statement
    let ast = pkl.generate_ast(source)?;

    Ok(())
}
```

### LICENSE

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
