//! `pkl_codegen` — Build-time code generation for Pkl schemas.
//!
//! Parses Pkl source files to extract class and property type information,
//! then generates type-safe Rust structs with `PklDecode` derives.
//!
//! # Usage in build.rs
//!
//! ```rust,no_run
//! pkl_codegen::generate("schemas/config.pkl", "src/gen/config.rs").unwrap();
//! ```

use std::path::Path;

// ── Public API ──

/// Generate Rust code from a Pkl module file.
///
/// Parses the Pkl source, extracts classes, type aliases, and module-level
/// properties, and generates equivalent Rust code with `PklDecode` derives.
pub fn generate(pkl_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Result<(), String> {
    let pkl_path = pkl_path.as_ref();
    let output_path = output_path.as_ref();

    let source = std::fs::read_to_string(pkl_path)
        .map_err(|e| format!("failed to read {}: {}", pkl_path.display(), e))?;

    let module = parse_module(&source)?;

    let code = generate_code(&module);

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create output directory: {}", e))?;
    }
    std::fs::write(output_path, &code)
        .map_err(|e| format!("failed to write output: {}", e))?;

    Ok(())
}

// ── Schema types ──

/// A parsed Pkl module containing classes, type aliases, and properties.
#[derive(Debug)]
pub struct Module {
    /// Class definitions found in the module.
    pub classes: Vec<Class>,
    /// Type alias definitions.
    pub type_aliases: Vec<TypeAlias>,
    /// Module-level property declarations.
    pub properties: Vec<Property>,
}

/// A parsed Pkl class definition.
#[derive(Debug)]
pub struct Class {
    /// The class name.
    pub name: String,
    /// The parent class name, if this class extends another.
    pub superclass: Option<String>,
    /// Properties declared in this class.
    pub properties: Vec<Property>,
    /// Whether this class is declared `open` for extension.
    pub is_open: bool,
    /// Whether this class is declared `abstract`.
    pub is_abstract: bool,
}

/// A parsed Pkl type alias.
#[derive(Debug)]
pub struct TypeAlias {
    /// The alias name.
    pub name: String,
    /// The full type expression (e.g. `"Seeds" | "Berries" | "Insects"`).
    pub type_str: String,
    /// Whether this is a union type (contains `|`).
    pub is_union: bool,
    /// For string literal unions, the individual variant values.
    pub variants: Vec<String>,
}

/// A parsed Pkl property declaration.
#[derive(Debug)]
pub struct Property {
    /// The property name.
    pub name: String,
    /// The type string (e.g. `"String"`, `"List<Int>"`, `"String?"`).
    pub type_str: String,
    /// Whether the property is nullable (`String?`).
    pub optional: bool,
    /// Whether the property has a default value.
    pub has_default: bool,
    /// Modifiers applied to this property (e.g. `hidden`, `local`, `fixed`).
    pub modifiers: Vec<String>,
}

// ── Pkl source parser ──

/// Parse a Pkl source string into its module structure.
///
/// Extracts class definitions, type aliases, and module-level properties.
pub fn parse_module(source: &str) -> Result<Module, String> {
    let mut classes = Vec::new();
    let mut type_aliases = Vec::new();
    let mut properties = Vec::new();

    // Strip comments
    let cleaned = strip_comments(source);

    // First pass: extract class blocks and their line ranges
    let (parsed_classes, class_line_ranges) = parse_class_blocks_with_ranges(&cleaned);

    // Second pass: parse module-level properties, skipping class body lines
    let lines: Vec<&str> = cleaned.lines().collect();
    for (line_idx, line) in lines.iter().enumerate() {
        let line = line.trim();
        if line.is_empty() { continue; }

        // Skip lines inside class bodies
        if class_line_ranges.iter().any(|(start, end)| line_idx >= *start && line_idx < *end) {
            continue;
        }

        // Type alias: `typealias Name = Type`
        if let Some(cap) = try_parse_typealias(line) {
            type_aliases.push(cap);
            continue;
        }

        // Module-level property
        if let Some(prop) = try_parse_property(line) {
            properties.push(prop);
            continue;
        }
    }

    classes.extend(parsed_classes);

    Ok(Module { classes, type_aliases, properties })
}

fn parse_class_blocks_with_ranges(source: &str) -> (Vec<Class>, Vec<(usize, usize)>) {
    let cleaned = strip_comments(source);
    let lines: Vec<&str> = cleaned.lines().collect();
    let mut classes = Vec::new();
    let mut ranges = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Look for class declaration
        if let Some(class_info) = try_extract_class_header(line) {
            let (name, superclass, is_abstract, is_open) = class_info;

            // Find opening brace
            let mut body_start = i;
            let mut found_open = false;
            for (offset, line) in lines[i..].iter().enumerate() {
                if line.trim().contains('{') || line.trim().ends_with('{') {
                    body_start = i + offset;
                    found_open = true;
                    break;
                }
            }
            if !found_open { i += 1; continue; }

            // Find matching closing brace
            let mut depth = 0;
            let mut body_end = body_start + 1;
            let mut found_close = false;
            for (offset, line) in lines[body_start..].iter().enumerate() {
                for ch in line.trim().chars() {
                    match ch {
                        '{' => depth += 1,
                        '}' => { depth -= 1; if depth == 0 { body_end = body_start + offset + 1; found_close = true; break; } }
                        _ => {}
                    }
                }
                if found_close { break; }
            }
            if !found_close { i += 1; continue; }

            // Parse properties from class body
            let mut properties = Vec::new();
            for line in &lines[(body_start + 1)..(body_end - 1)] {
                if let Some(prop) = try_parse_property(line.trim()) {
                    properties.push(prop);
                }
            }

            ranges.push((body_start, body_end));
            classes.push(Class { name, superclass, properties, is_open, is_abstract });
            i = body_end;
        } else {
            i += 1;
        }
    }

    (classes, ranges)
}

fn try_extract_class_header(line: &str) -> Option<(String, Option<String>, bool, bool)> {
    let line = line.trim();
    if !line.starts_with("class ") && !line.starts_with("abstract class ") && !line.starts_with("open class ") {
        return None;
    }

    let mut rest = line;
    let mut is_abstract = false;
    let mut is_open = false;

    if rest.starts_with("abstract ") { is_abstract = true; rest = rest.strip_prefix("abstract ").unwrap(); }
    if rest.starts_with("open ") { is_open = true; rest = rest.strip_prefix("open ").unwrap(); }
    if !rest.starts_with("class ") { return None; }
    rest = rest.strip_prefix("class ").unwrap();

    // Split on '{', 'extends', or whitespace
    let name_end = rest.find(['{', ' ', '\t']).unwrap_or(rest.len());
    let name = rest[..name_end].to_string();
    if name.is_empty() { return None; }

    let after_name = rest[name_end..].trim();
    let superclass = after_name
        .strip_prefix("extends ")
        .map(|e| {
            let extends_end = e.find(['{', ' ']).unwrap_or(e.len());
            e[..extends_end].trim().to_string()
        });

    Some((name, superclass, is_abstract, is_open))
}

fn strip_comments(source: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
            // Line comment: skip to end of line
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
            // Block comment: skip to */
            i += 2;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            continue;
        }
        if i + 2 < len && chars[i] == '/' && chars[i + 1] == '/' && chars[i + 2] == '/' {
            // Doc comment: skip to end of line
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

fn try_parse_typealias(line: &str) -> Option<TypeAlias> {
    let line = line.trim();
    if !line.starts_with("typealias ") {
        return None;
    }
    let rest = line.strip_prefix("typealias ")?;

    // Split on '=' to get name and type
    let eq_pos = rest.find('=')?;
    let name = rest[..eq_pos].trim().to_string();
    let type_str = rest[eq_pos + 1..].trim().to_string();

    // Check if it's a union of string literals: "a" | "b" | "c"
    let variants: Vec<String> = type_str
        .split('|')
        .map(|s| s.trim().trim_matches('"'))
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let is_union = type_str.contains('|');

    Some(TypeAlias { name, type_str, is_union, variants })
}

fn try_parse_property(line: &str) -> Option<Property> {
    let line = line.trim();

    // Skip non-property lines
    if line.starts_with("class ")
        || line.starts_with("typealias ")
        || line.starts_with("import ")
        || line.starts_with("amends ")
        || line.starts_with("extends ")
        || line.starts_with("module ")
        || line.starts_with("output")
        || line.starts_with('{')
        || line.starts_with('}')
        || line.starts_with("function ")
        || line.starts_with("local ")
    {
        return None;
    }

    // Extract modifiers (hidden, local, fixed, const, abstract, open)
    let mut modifiers = Vec::new();
    let mut rest = line;
    loop {
        let stripped = rest.trim_start();
        let word = stripped.split_whitespace().next()?;
        if matches!(word, "hidden" | "local" | "fixed" | "const" | "abstract" | "open") {
            modifiers.push(word.to_string());
            rest = stripped.strip_prefix(word).unwrap_or(stripped);
        } else {
            break;
        }
    }

    // Parse `name: Type` or `name: Type = default`
    let rest = rest.trim();
    if !rest.contains(':') {
        return None;
    }

    let colon_pos = rest.find(':')?;
    let name = rest[..colon_pos].trim().to_string();

    let after_colon = rest[colon_pos + 1..].trim();

    // Check for default value (= ...)
    let (type_part, has_default) = if let Some(eq_pos) = after_colon.find("= ") {
        (after_colon[..eq_pos].trim(), true)
    } else {
        (after_colon, false)
    };

    // Parse optional (`?`)
    let (base_type, optional) = if let Some(base) = type_part.strip_suffix('?') {
        (base.trim().to_string(), true)
    } else {
        (type_part.to_string(), false)
    };

    Some(Property {
        name,
        type_str: base_type,
        optional,
        has_default,
        modifiers,
    })
}
// ── Code generation ──

/// Generate Rust source code from a parsed Pkl module.
///
/// Produces struct definitions with `PklDecode` derives for classes,
/// enum definitions for string literal union type aliases, and a `Root`
/// struct for module-level properties.
pub fn generate_code(module: &Module) -> String {
    let mut code = String::new();
    code.push_str("// Auto-generated by pkl_codegen. Do not edit.\n\n");
    code.push_str("#![allow(dead_code, non_snake_case)]\n\n");
    code.push_str("use pkl_core::{PklDecode, Duration, DataSize, Value};\n");
    code.push_str("use std::collections::BTreeMap;\n\n");

    // Generate type aliases
    for ta in &module.type_aliases {
        if ta.is_union && ta.variants.len() > 1 && ta.variants.iter().all(|v| v.starts_with(|c: char| c.is_alphanumeric())) {
            // Generate as enum
            code.push_str(&generate_enum_from_union(ta));
        } else {
            // Generate as type alias
            let mapped = map_pkl_type_str(&ta.type_str);
            code.push_str(&format!("pub type {} = {};\n\n", ta.name, mapped));
        }
    }

    // Generate classes
    for class in &module.classes {
        code.push_str(&generate_struct(class, module));
        code.push('\n');
    }

    // Generate root struct for module-level properties
    if !module.properties.is_empty() {
        code.push_str(&generate_root_struct(module));
        code.push('\n');
    }

    code
}

fn generate_enum_from_union(ta: &TypeAlias) -> String {
    let variants: Vec<String> = ta.variants.iter()
        .filter(|v| !v.is_empty())
        .map(|v| {
            let variant_name = to_rust_ident(v);
            format!("    {}", variant_name)
        })
        .collect();

    let match_arms: Vec<String> = ta.variants.iter()
        .filter(|v| !v.is_empty())
        .map(|v| {
            let variant_name = to_rust_ident(v);
            format!("            \"{}\" => Ok(Self::{}),", v, variant_name)
        })
        .collect();

    format!(
        r#"#[derive(PklDecode, Debug, Clone, PartialEq)]
pub enum {} {{
{}
}}

impl PklDecode for {} {{
    fn decode(value: Value) -> std::result::Result<Self, pkl_core::PklError> {{
        match value {{
            Value::String(s) => match s.as_str() {{
{}
                _ => Err(pkl_core::PklError::DecodeError(format!("unknown variant: {{}}", s))),
            }},
            other => Err(pkl_core::PklError::TypeMismatch {{
                expected: "string",
                actual: format!("{{:?}}", other),
            }}),
        }}
    }}
}}
"#,
        ta.name,
        variants.join(",\n"),
        ta.name,
        match_arms.join("\n"),
    )
}

fn generate_struct(class: &Class, _module: &Module) -> String {
    let derives = ["PklDecode", "Debug", "Clone"];

    let mut code = String::new();

    // Doc comment if class has documentation (we can't detect this in simple parser)
    code.push_str(&format!("#[derive({})]\n", derives.join(", ")));
    code.push_str(&format!("pub struct {} {{\n", class.name));

    for prop in &class.properties {
        let rust_type = map_pkl_type_str(&prop.type_str);
        let optional = if prop.optional { "Option<" } else { "" };
        let close = if prop.optional { ">" } else { "" };

        // Convert Pkl camelCase to Rust snake_case
        let rust_name = to_snake_case(&prop.name);
        code.push_str(&format!("    pub {}: {}{}{},\n", rust_name, optional, rust_type, close));
    }

    code.push_str("}\n");
    code
}

fn generate_root_struct(module: &Module) -> String {
    let mut code = "#[derive(PklDecode, Debug, Clone)]\npub struct Root {\n".to_string();

    for prop in &module.properties {
        let rust_type = map_pkl_type_str(&prop.type_str);
        let optional = if prop.optional { "Option<" } else { "" };
        let close = if prop.optional { ">" } else { "" };
        let rust_name = to_snake_case(&prop.name);
        code.push_str(&format!("    pub {}: {}{}{},\n", rust_name, optional, rust_type, close));
    }

    code.push_str("}\n");
    code
}

// ── Type mapping ──

fn map_pkl_type_str(pkl_type: &str) -> String {
    let t = pkl_type.trim();
    match t {
        "String" => "String".to_string(),
        "Int" | "UInt" | "Int8" | "Int16" | "Int32" | "UInt8" | "UInt16" | "UInt32" => "i64".to_string(),
        "Float" => "f64".to_string(),
        "Boolean" => "bool".to_string(),
        "Duration" => "Duration".to_string(),
        "DataSize" => "DataSize".to_string(),
        "Null" | "Nothing" | "Unit" => "()".to_string(),
        "Any" | "unknown" | "Dynamic" => "Value".to_string(),
        _ if t.starts_with("List<") || t.starts_with("Listing<") => {
            let inner = extract_generic_param(t).unwrap_or("Value");
            format!("Vec<{}>", map_pkl_type_str(inner))
        }
        _ if t.starts_with("Set<") => {
            let inner = extract_generic_param(t).unwrap_or("Value");
            format!("Vec<{}>", map_pkl_type_str(inner))
        }
        _ if t.starts_with("Map<") || t.starts_with("Mapping<") => {
            let params = extract_generic_params(t);
            if params.len() == 2 {
                format!("BTreeMap<{}, {}>", map_pkl_type_str(&params[0]), map_pkl_type_str(&params[1]))
            } else {
                "BTreeMap<Value, Value>".to_string()
            }
        }
        _ if t.starts_with("Pair<") => {
            let params = extract_generic_params(t);
            if params.len() == 2 {
                format!("({}, {})", map_pkl_type_str(&params[0]), map_pkl_type_str(&params[1]))
            } else {
                "(Value, Value)".to_string()
            }
        }
        "Regex" => "String".to_string(),
        "Bytes" => "Vec<u8>".to_string(),
        "Number" => "f64".to_string(), // Number can be Int or Float, use f64
        // Union types: "A | B | C"
        _ if t.contains(" | ") || t.contains('|') => {
            let parts: Vec<&str> = t.split('|').map(|s| s.trim()).collect();
            if parts.iter().all(|p| p.starts_with('"') && p.ends_with('"')) {
                // String literal union — generate enum name
                // This will be handled by the type alias → enum generation
                let enum_name = parts.iter()
                    .map(|p| p.trim_matches('"'))
                    .map(to_rust_ident)
                    .collect::<Vec<_>>()
                    .join("Or");
                format!("String /* union: {} */", enum_name)
            } else {
                "Value".to_string()
            }
        }
        // String literal type: "value"
        _ if t.starts_with('"') && t.ends_with('"') => {
            "String".to_string()
        }
        // Class reference or other named type
        _ => t.to_string(),
    }
}

fn extract_generic_param(t: &str) -> Option<&str> {
    let start = t.find('<')?;
    let end = t.rfind('>')?;
    let inner = &t[start + 1..end];
    // Handle simple single param (no commas at top level)
    if inner.contains(',') {
        None
    } else {
        Some(inner.trim())
    }
}

fn extract_generic_params(t: &str) -> Vec<String> {
    let start = t.find('<');
    let end = t.rfind('>');
    match (start, end) {
        (Some(s), Some(e)) => {
            let inner = &t[s + 1..e];
            let mut params = Vec::new();
            let mut depth = 0;
            let mut current = String::new();
            for ch in inner.chars() {
                match ch {
                    '<' => { depth += 1; current.push(ch); }
                    '>' => { depth -= 1; current.push(ch); }
                    ',' if depth == 0 => {
                        params.push(current.trim().to_string());
                        current = String::new();
                    }
                    _ => { current.push(ch); }
                }
            }
            if !current.is_empty() {
                params.push(current.trim().to_string());
            }
            params
        }
        _ => Vec::new(),
    }
}

// ── Name conversion helpers ──

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.char_indices() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

fn to_rust_ident(s: &str) -> String {
    if s.is_empty() {
        return "Empty".to_string();
    }
    let mut result = String::new();
    let mut upper_next = true;
    for ch in s.chars() {
        if ch == '_' || ch == '-' || ch == ' ' {
            upper_next = true;
        } else if upper_next {
            result.push(ch.to_ascii_uppercase());
            upper_next = false;
        } else {
            result.push(ch.to_ascii_lowercase());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_properties() {
        let src = r#"
name: String
count: Int
flag: Boolean
"#;
        let module = parse_module(src).unwrap();
        assert_eq!(module.properties.len(), 3);
        assert_eq!(module.properties[0].name, "name");
        assert_eq!(module.properties[0].type_str, "String");
        assert_eq!(module.properties[1].name, "count");
        assert_eq!(module.properties[1].type_str, "Int");
        assert_eq!(module.properties[2].name, "flag");
        assert_eq!(module.properties[2].type_str, "Boolean");
    }

    #[test]
    fn test_parse_optional_property() {
        let src = r#"name: String?"#;
        let module = parse_module(src).unwrap();
        assert_eq!(module.properties[0].name, "name");
        assert_eq!(module.properties[0].type_str, "String");
        assert!(module.properties[0].optional);
    }

    #[test]
    fn test_parse_property_with_default() {
        let src = r#"timeout: Duration = 30.s"#;
        let module = parse_module(src).unwrap();
        assert_eq!(module.properties[0].name, "timeout");
        assert!(module.properties[0].has_default);
    }

    #[test]
    fn test_parse_class() {
        let src = r#"
class Bird {
  name: String
  lifespan: Int
}
"#;
        let module = parse_module(src).unwrap();
        assert_eq!(module.classes.len(), 1);
        assert_eq!(module.classes[0].name, "Bird");
        assert_eq!(module.classes[0].properties.len(), 2);
        assert_eq!(module.classes[0].properties[0].name, "name");
        assert_eq!(module.classes[0].properties[1].name, "lifespan");
    }

    #[test]
    fn test_parse_class_with_extends() {
        let src = r#"
class Animal {
  name: String
}
class Bird extends Animal {
  wingspan: Float
}
"#;
        let module = parse_module(src).unwrap();
        assert_eq!(module.classes.len(), 2);
        assert_eq!(module.classes[0].name, "Animal");
        assert_eq!(module.classes[1].name, "Bird");
        assert_eq!(module.classes[1].superclass.as_deref(), Some("Animal"));
    }

    #[test]
    fn test_parse_generic_types() {
        let src = r#"
tags: List<String>
metadata: Map<String, String>
"#;
        let module = parse_module(src).unwrap();
        assert_eq!(module.properties.len(), 2);
    }

    #[test]
    fn test_parse_typealias() {
        let src = "typealias Email = String";
        let module = parse_module(src).unwrap();
        assert_eq!(module.type_aliases.len(), 1);
        assert_eq!(module.type_aliases[0].name, "Email");
    }

    #[test]
    fn test_parse_union_typealias() {
        let src = r#"typealias Diet = "Seeds" | "Berries" | "Insects""#;
        let module = parse_module(src).unwrap();
        assert_eq!(module.type_aliases.len(), 1);
        let ta = &module.type_aliases[0];
        assert!(ta.is_union);
        assert_eq!(ta.variants, vec!["Seeds", "Berries", "Insects"]);
    }

    #[test]
    fn test_type_mapping_list() {
        assert_eq!(map_pkl_type_str("List<String>"), "Vec<String>");
        assert_eq!(map_pkl_type_str("Listing<Int>"), "Vec<i64>");
    }

    #[test]
    fn test_type_mapping_map() {
        assert_eq!(map_pkl_type_str("Map<String, Int>"), "BTreeMap<String, i64>");
    }

    #[test]
    fn test_snake_case_conversion() {
        assert_eq!(to_snake_case("camelCase"), "camel_case");
        assert_eq!(to_snake_case("name"), "name");
        assert_eq!(to_snake_case("myFieldName"), "my_field_name");
    }

    #[test]
    fn test_struct_generation() {
        let src = r#"
class Person {
  firstName: String
  lastName: String
  age: Int
}
"#;
        let module = parse_module(src).unwrap();
        let code = generate_code(&module);
        assert!(code.contains("struct Person"));
        assert!(code.contains("first_name"));
        assert!(code.contains("last_name"));
        assert!(code.contains("age"));
        assert!(code.contains("PklDecode"));
    }

    #[test]
    fn test_enum_generation_from_union() {
        let src = r#"typealias Diet = "Seeds" | "Berries" | "Insects""#;
        let module = parse_module(src).unwrap();
        let code = generate_code(&module);
        assert!(code.contains("enum Diet"));
        assert!(code.contains("Seeds"));
        assert!(code.contains("Berries"));
        assert!(code.contains("Insects"));
        assert!(code.contains("impl PklDecode for Diet"));
    }

    #[test]
    fn test_full_pkl_file() {
        let src = r#"
/// Application configuration
name: String
version: Int
debug: Boolean
tags: List<String>
timeout: Duration?
storage: DataSize

class Database {
  host: String
  port: UInt16
  credentials: Credentials
}

class Credentials {
  username: String
  password: String?
}

typealias Port = UInt16
typealias Env = "development" | "staging" | "production"
"#;
        let module = parse_module(src).unwrap();
        let code = generate_code(&module);

        // Has module-level properties
        assert!(code.contains("struct Root"));
        assert!(code.contains("name"));
        assert!(code.contains("debug"));
        assert!(code.contains("tags"));
        assert!(code.contains("Option<Duration>"));

        // Has classes
        assert!(code.contains("struct Database"));
        assert!(code.contains("struct Credentials"));

        // Has enum from union
        assert!(code.contains("enum Env"));
        assert!(code.contains("development"));

        // Has type alias
        assert!(code.contains("type Port = i64"));

        println!("Generated code:\n{}", code);
    }

    #[test]
    fn test_generated_code_compiles() {
        // This test verifies that our generated code is valid Rust
        let src = r#"
name: String
count: Int
flag: Boolean
tags: List<String>
timeout: Duration?

class Config {
  host: String
  port: Int
  debug: Boolean
}

typealias Env = "dev" | "prod"
"#;
        let module = parse_module(src).unwrap();
        let code = generate_code(&module);

        // Write to a temp file and try to parse it with syn
        use std::io::Write;
        let tmp = std::env::temp_dir().join("pkl_codegen_test.rs");
        let mut f = std::fs::File::create(&tmp).unwrap();
        f.write_all(code.as_bytes()).unwrap();

        // Check that syn can parse the generated code
        let parsed = syn::parse_file(&code);
        assert!(parsed.is_ok(), "Generated code should be valid Rust. Errors: {:?}", parsed.err());

        println!("Generated code:\n{}", code);
    }
}
