//! pkl-bindgen — Code generation and evaluation for the [Pkl configuration language](https://pkl-lang.org).
//!
//! # Subcommands
//!
//! * `generate` — Parse a `.pkl` file and generate Rust structs with `PklDecode` derives
//! * `eval` — Evaluate a Pkl module and print the result as JSON/PCF
//! * `expr` — Evaluate inline Pkl text
//!
//! # Examples
//!
//! ```bash
//! # Generate Rust code from a Pkl schema
//! pkl-bindgen generate schema.pkl -o src/gen/schema.rs
//!
//! # Evaluate a config file
//! pkl-bindgen eval config.pkl
//!
//! # Quick inline evaluation
//! pkl-bindgen expr 'name = "Hello, Pkl!"'
//!
//! # Use persistent server evaluator for speed
//! pkl-bindgen eval config.pkl --server
//! ```

use clap::{Parser, Subcommand};
use pkl_core::PklEvaluator;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "pkl-bindgen",
    about = "Code generation and evaluation for Pkl configuration language",
    version,
    long_about = "pkl-bindgen generates type-safe Rust code from Pkl schemas and evaluates Pkl configuration files.\n\nRequires the `pkl` CLI binary (https://pkl-lang.org) to be installed.",
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate Rust code from a Pkl module.
    ///
    /// Parses the .pkl file, extracts classes, type aliases, and
    /// module-level properties, then generates equivalent Rust struct
    /// and enum definitions with PklDecode derives.
    #[command(name = "generate", aliases = &["gen", "g"])]
    Generate {
        /// Path to the .pkl file
        input: PathBuf,

        /// Output file path (defaults to <input>.rs)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Print generated code to stdout instead of writing to file
        #[arg(short, long)]
        stdout: bool,
    },

    /// Evaluate a Pkl module and print the result.
    ///
    /// Reads and evaluates a .pkl file (or remote URI), then outputs
    /// the result in the requested format.
    #[command(name = "eval")]
    Eval {
        /// Path or URI of the Pkl module
        input: String,

        /// Output format: json, pcf, or debug
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Pkl expression to evaluate (e.g. "output.text")
        #[arg(short = 'x', long)]
        expression: Option<String>,

        /// Use persistent server evaluator (faster for multiple runs)
        #[arg(long)]
        server: bool,
    },

    /// Evaluate inline Pkl text.
    ///
    /// Evaluate a Pkl expression passed as a command-line argument
    /// without needing a .pkl file.
    #[command(name = "expr", aliases = &["e"])]
    Expr {
        /// Pkl source text to evaluate
        text: String,

        /// Output format: json, pcf, or debug (default: json)
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Use persistent server evaluator (faster for multiple runs)
        #[arg(long)]
        server: bool,
    },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate { input, output, stdout } => {
            cmd_generate(&input, output.as_ref(), stdout)
        }
        Command::Eval { input, format, expression, server } => {
            let rt = tokio::runtime::Runtime::new().map_err(|e| format!("runtime: {}", e))?;
            rt.block_on(cmd_eval(&input, &format, expression.as_deref(), server))
        }
        Command::Expr { text, format, server } => {
            let rt = tokio::runtime::Runtime::new().map_err(|e| format!("runtime: {}", e))?;
            rt.block_on(cmd_expr(&text, &format, server))
        }
    }
}

fn cmd_generate(input: &PathBuf, output: Option<&PathBuf>, stdout: bool) -> Result<(), String> {
    let out_path = match (output, stdout) {
        (Some(path), _) => path.clone(),
        (None, false) => {
            let mut p = input.clone();
            p.set_extension("rs");
            p
        }
        (None, true) => PathBuf::from(""),
    };

    if stdout {
        let source = std::fs::read_to_string(input)
            .map_err(|e| format!("read {}: {}", input.display(), e))?;
        let module = pkl_codegen::parse_module(&source)?;
        let code = pkl_codegen::generate_code(&module);
        println!("{}", code);
        Ok(())
    } else {
        pkl_codegen::generate(input, &out_path)
    }
}

enum Eval {
    Cli(pkl_core::CliEvaluator),
    Server(pkl_core::ServerEvaluator),
}

impl Eval {
    async fn evaluate_raw(&self, source: &pkl_core::ModuleSource) -> Result<pkl_core::Value, pkl_core::PklError> {
        match self {
            Eval::Cli(e) => e.evaluate_raw(source).await,
            Eval::Server(e) => e.evaluate_raw(source).await,
        }
    }

    async fn evaluate_text(&self, source: &pkl_core::ModuleSource) -> Result<String, pkl_core::PklError> {
        match self {
            Eval::Cli(e) => e.evaluate_text(source).await,
            Eval::Server(e) => e.evaluate_text(source).await,
        }
    }
}

async fn make_eval(server: bool) -> Result<Eval, String> {
    if server {
        Ok(Eval::Server(pkl_core::ServerEvaluator::new().await.map_err(|e| format!("server: {}", e))?))
    } else {
        Ok(Eval::Cli(pkl_core::CliEvaluator::new()))
    }
}

async fn cmd_eval(input: &str, format: &str, expression: Option<&str>, server: bool) -> Result<(), String> {
    let evaluator = make_eval(server).await?;

    let source = if input.starts_with("http://") || input.starts_with("https://") || input.starts_with("pkl:") {
        pkl_core::ModuleSource::from_uri(input.to_string())
    } else {
        pkl_core::ModuleSource::from_file(input)
    };

    if expression.is_some() {
        let text = evaluator.evaluate_text(&source).await.map_err(|e| format!("eval: {}", e))?;
        println!("{}", text);
    } else {
        let value = evaluator.evaluate_raw(&source).await.map_err(|e| format!("eval: {}", e))?;
        print_value(&value, format);
    }

    Ok(())
}

async fn cmd_expr(text: &str, format: &str, server: bool) -> Result<(), String> {
    let evaluator = make_eval(server).await?;
    let source = pkl_core::ModuleSource::from_text(text);
    let value = evaluator.evaluate_raw(&source).await.map_err(|e| format!("eval: {}", e))?;
    print_value(&value, format);
    Ok(())
}

fn print_value(value: &pkl_core::Value, format: &str) {
    match format {
        "json" => print_value_json(value, 0),
        "pcf" | "pkl" => print_value_pcf(value, 0),
        "debug" => println!("{:#?}", value),
        _ => print_value_json(value, 0),
    }
    println!();
}

fn print_value_json(value: &pkl_core::Value, indent: usize) {
    use pkl_core::Value;
    let prefix = " ".repeat(indent);
    match value {
        Value::Null => print!("null"),
        Value::Boolean(b) => print!("{}", b),
        Value::Int(i) => print!("{}", i),
        Value::Float(f) => print!("{}", f),
        Value::String(s) => print!("\"{}\"", s),
        Value::Object(map) => {
            print!("{{");
            for (i, (k, v)) in map.iter().enumerate() {
                if i > 0 { print!(","); }
                println!();
                print!("{}  \"{}\": ", prefix, k);
                print_value_json(v, indent + 2);
            }
            if !map.is_empty() { println!(); print!("{}", prefix); }
            print!("}}");
        }
        Value::List(items) | Value::Listing(items) | Value::Set(items) => {
            print!("[");
            for (i, item) in items.iter().enumerate() {
                if i > 0 { print!(", "); }
                print_value_json(item, indent + 2);
            }
            print!("]");
        }
        Value::Map(pairs) => {
            print!("{{");
            for (i, (k, v)) in pairs.iter().enumerate() {
                if i > 0 { print!(","); }
                println!();
                print!("{}  ", prefix);
                print_value_json(k, indent + 2);
                print!(": ");
                print_value_json(v, indent + 2);
            }
            if !pairs.is_empty() { println!(); print!("{}", prefix); }
            print!("}}");
        }
        Value::Mapping(pairs) => {
            print!("{{");
            for (i, (k, v)) in pairs.iter().enumerate() {
                if i > 0 { print!(","); }
                println!();
                print!("{}  ", prefix);
                print_value_json(k, indent + 2);
                print!(": ");
                print_value_json(v, indent + 2);
            }
            if !pairs.is_empty() { println!(); print!("{}", prefix); }
            print!("}}");
        }
        Value::Duration(d) => print!("\"{}\"", d),
        Value::DataSize(d) => print!("\"{}\"", d),
        Value::Pair(p) => {
            print!("[");
            print_value_json(&p.0, indent + 2);
            print!(", ");
            print_value_json(&p.1, indent + 2);
            print!("]");
        }
        Value::IntSeq { start, end, step } => print!("IntSeq({}, {}, {})", start, end, step),
        Value::Regex(s) => print!("\"{}\"", s),
        Value::Bytes(b) => print!("Bytes({} bytes)", b.len()),
    }
}

fn print_value_pcf(value: &pkl_core::Value, indent: usize) {
    use pkl_core::Value;
    let prefix = " ".repeat(indent);
    match value {
        Value::Null => print!("null"),
        Value::Boolean(b) => print!("{}", b),
        Value::Int(i) => print!("{}", i),
        Value::Float(f) => print!("{}", f),
        Value::String(s) => print!("\"{}\"", s),
        Value::Object(map) => {
            for (k, v) in map.iter() {
                print!("{}{} = ", prefix, k);
                if matches!(v, Value::Object(_) | Value::List(_) | Value::Listing(_)) {
                    println!();
                    print!("{}{{", prefix);
                    println!();
                    print_value_pcf(v, indent + 2);
                    println!();
                    print!("{}}}", prefix);
                } else {
                    print_value_pcf(v, indent);
                    println!();
                }
            }
        }
        Value::List(items) | Value::Listing(items) => {
            for item in items {
                print!("{}- ", prefix);
                print_value_pcf(item, indent + 2);
                println!();
            }
        }
        Value::Duration(d) => print!("{}.{}", d.value, d.unit),
        Value::DataSize(d) => print!("{}.{}", d.value, d.unit),
        other => print!("{:?}", other),
    }
}
