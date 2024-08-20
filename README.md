# pkl_fast

Fastest pkl-parsing crate out there (and surely the only one)!

I am currently working on a big rework, as the current lexer (logos) does not cover all the features I need, I am replacing it with the pest crate! Should have payed more attention, sry!

## Features

- Parse Pkl string into a structured representation (hashmap) in rust
- Parse Pkl string into an AST
- Support for strings, integers (decimal, octal, hex, binary), floats, boolean, objects (amends syntax as well), class instances
- Boolean API supported
- String API (mostly) supported
- Int/Float/Duration/DataSize properties and methods supported

## Currently Not Supported

- Multiline String containing <<">> not preceded by a backlash, String interpolation and Strings with custom delimiters
- Lists methods API, only properties are supported
- Listings, Mappings, Maps
- functions -> thus also functions and methods taking functions as parameters
- Packages (official or not) imports not supported
- Globbed imports + dynamic imports + amends expresions
- type annotations
- Classes declarations
- If expressions

## Installation

When in your rust project, simply run: `cargo add new-pkl` (for the moment use new-pkl crate, new stable release coming to pkl_fast really soon)

## Usage

Here's an example of how to parse a PKL string and retrieve values from the context:

```rust
use new_pkl::{Pkl, PklResult, PklValue};

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
