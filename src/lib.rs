use hashbrown::HashMap;
use lexer::PklToken;
use parser::{parse_pkl, statement::PklStatement};
use table::{ast_to_table, PklTable};

mod lexer;
mod parser;
mod table;
mod types;
mod utils;

pub use parser::PklResult;
pub use table::PklValue;

#[derive(Debug, PartialEq, Clone)]
/// The `Pkl` struct represents the main interface for working with PKL data.
pub struct Pkl {
    table: PklTable,
}

impl Pkl {
    /// Creates a new, empty `Pkl` instance.
    pub fn new() -> Self {
        Self {
            table: PklTable::new(),
        }
    }

    /// Parses a PKL source string and populates the internal context.
    ///
    /// # Arguments
    ///
    /// * `source` - The PKL source string to parse.
    ///
    /// # Returns
    ///
    /// A `PklResult` indicating success or failure.
    pub fn parse(&mut self, source: &str) -> PklResult<()> {
        let parsed = self.generate_ast(source)?;
        self.table.extends(ast_to_table(parsed)?);

        Ok(())
    }

    /// Generates an AST from a PKL source string.
    ///
    /// # Arguments
    ///
    /// * `source` - The PKL source string to parse.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the generated AST or an error message.
    pub fn generate_ast<'a>(&'a self, source: &'a str) -> PklResult<Vec<PklStatement>> {
        use logos::Logos;
        let mut lexer = PklToken::lexer(source);
        parse_pkl(&mut lexer)
    }

    /// Retrieves a value from the context by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `PklValue` associated with the name,
    /// or `None` if the variable is not found.
    pub fn get(&self, name: &str) -> Option<&PklValue> {
        self.table.get(name)
    }

    /// Sets or modifies a value in the context by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to set.
    /// * `value` - The value to set.
    ///
    /// # Returns
    ///
    /// An `Option` containing the previous value associated with the name, if any.
    pub fn set(&mut self, name: &str, value: PklValue) -> Option<PklValue> {
        self.table.insert(name, value)
    }

    /// Removes a value from the context by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to remove.
    ///
    /// # Returns
    ///
    /// An `Option` containing the removed value, if any.
    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<PklValue> {
        self.table.variables.remove(name.as_ref())
    }

    /// Retrieves a boolean value from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to retrieve.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the boolean value or an error message if not found or wrong type.
    pub fn get_bool(&self, name: &str) -> PklResult<bool> {
        match self.table.get(name) {
            Some(PklValue::Bool(b)) => Ok(*b),
            Some(_) => Err((format!("Variable `{}` is not a boolean", name), 0..0)),
            None => Err((format!("Variable `{}` not found", name), 0..0)),
        }
    }

    /// Retrieves an integer value from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to retrieve.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the integer value or an error message if not found or wrong type.
    pub fn get_int(&self, name: &str) -> PklResult<i64> {
        match self.table.get(name) {
            Some(PklValue::Int(i)) => Ok(*i),
            Some(_) => Err((format!("Variable `{}` is not an integer", name), 0..0)),
            None => Err((format!("Variable `{}` not found", name), 0..0)),
        }
    }

    /// Retrieves a floating-point value from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to retrieve.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the floating-point value or an error message if not found or wrong type.
    pub fn get_float(&self, name: &str) -> PklResult<f64> {
        match self.table.get(name) {
            Some(PklValue::Float(f)) => Ok(*f),
            Some(_) => Err((format!("Variable `{}` is not a float", name), 0..0)),
            None => Err((format!("Variable `{}` not found", name), 0..0)),
        }
    }

    /// Retrieves a string value from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to retrieve.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the string value or an error message if not found or wrong type.
    pub fn get_string(&self, name: &str) -> PklResult<String> {
        match self.table.get(name) {
            Some(PklValue::String(s)) => Ok(s.to_owned()),
            Some(_) => Err((format!("Variable `{}` is not a string", name), 0..0)),
            None => Err((format!("Variable `{}` not found", name), 0..0)),
        }
    }

    /// Retrieves an object value from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to retrieve.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the object value or an error message if not found or wrong type.
    pub fn get_object(&self, name: &str) -> PklResult<&HashMap<String, PklValue>> {
        match self.table.get(name) {
            Some(PklValue::Object(o)) => Ok(o),
            Some(_) => Err((format!("Variable `{}` is not an object", name), 0..0)),
            None => Err((format!("Variable `{}` not found", name), 0..0)),
        }
    }
}

impl Default for Pkl {
    fn default() -> Self {
        Self::new()
    }
}

pub mod values {
    pub use crate::table::base::data_size::{Byte, Unit as DataSizeUnit};
    pub use crate::table::base::duration::Unit as DurationUnit;
}
