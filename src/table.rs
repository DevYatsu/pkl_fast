use crate::{
    parser::{
        AstPklValue, ExprHash, ExprMember, FuncCall, Identifier, PklExpr, PklResult, PklStatement,
    },
    Pkl,
};
use bool_api::match_bool_methods_api;
use data_size::{match_data_size_methods_api, match_data_size_props_api, Byte};
use duration::{match_duration_methods_api, match_duration_props_api, Duration};
use float_api::{match_float_methods_api, match_float_props_api};
use hashbrown::HashMap;
use int_api::{match_int_methods_api, match_int_props_api};
use list_api::match_list_props_api;
use std::{fs, ops::Range};
use string_api::{match_string_methods_api, match_string_props_api};

mod bool_api;
pub mod data_size;
pub mod duration;
mod float_api;
mod int_api;
mod list_api;
mod official_pkg;
mod string_api;

/// Represents a value in the PKL format.
///
/// The `PklValue` enum encapsulates various types of values that can be parsed from a PKL string.
/// These include booleans, floats, integers, strings, multiline strings, objects, and class instances.
///
/// # Variants
///
/// * `Bool` - Represents a boolean value.
/// * `Float` - Represents a floating-point number.
/// * `Int` - Represents an integer, which can be decimal, octal, hex, or binary.
/// * `String` - Represents a single-line string.
/// * `MultiLineString` - Represents a multiline string.
/// * `Object` - Represents a nested object, which is a hashmap of key-value pairs.
/// * `ClassInstance` - Represents an instance of a class, which includes the class name and its properties.
#[derive(Debug, PartialEq, Clone)]
pub enum PklValue {
    Null,

    /// A boolean value.
    Bool(bool),

    /// A floating-point number.
    Float(f64),

    /// An integer value.
    Int(i64),

    /// A single-line string.
    ///
    /// String are String and not &str
    /// because we may need to manipulate and modify them.
    String(String),

    /// A List
    List(Vec<PklValue>),

    /// A nested object represented as a hashmap of key-value pairs.
    Object(HashMap<String, PklValue>),

    /// An instance of a class, including the class name and its properties.
    ClassInstance(String, HashMap<String, PklValue>),

    /// A duration
    Duration(Duration),

    // A datasize
    DataSize(Byte),
}

impl PklValue {
    pub fn get_type(&self) -> &str {
        match self {
            PklValue::Null => "Null",
            PklValue::Bool(_) => "Bool",
            PklValue::Float(_) => "Float",
            PklValue::Int(_) => "Int",
            PklValue::String(_) => "String",
            PklValue::List(_) => "List",
            PklValue::Object(_) => "Object",
            PklValue::ClassInstance(_, _) => "ClassInstance",
            PklValue::Duration(_) => "Duration",
            PklValue::DataSize(_) => "DataSize",
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, PklValue::String(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, PklValue::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, PklValue::Float(_) | PklValue::Int(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, PklValue::Null)
    }

    pub fn is_list(&self) -> bool {
        matches!(self, PklValue::List(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, PklValue::Object(_))
    }

    pub fn is_datasize(&self) -> bool {
        matches!(self, PklValue::DataSize(_))
    }

    pub fn is_duration(&self) -> bool {
        matches!(self, PklValue::Duration(_))
    }

    pub fn as_string(&self) -> Option<&String> {
        if let PklValue::String(ref s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let PklValue::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            PklValue::Float(f) => Some(*f),
            PklValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            PklValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            PklValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<PklValue>> {
        if let PklValue::List(ref l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, PklValue>> {
        if let PklValue::Object(ref o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn as_datasize(&self) -> Option<&Byte> {
        if let PklValue::DataSize(ref d) = self {
            Some(d)
        } else {
            None
        }
    }

    pub fn as_duration(&self) -> Option<&Duration> {
        if let PklValue::Duration(ref d) = self {
            Some(d)
        } else {
            None
        }
    }
}

impl From<bool> for PklValue {
    fn from(value: bool) -> Self {
        PklValue::Bool(value)
    }
}

impl From<f64> for PklValue {
    fn from(value: f64) -> Self {
        PklValue::Float(value)
    }
}

impl From<i64> for PklValue {
    fn from(value: i64) -> Self {
        PklValue::Int(value)
    }
}

impl From<String> for PklValue {
    fn from(value: String) -> Self {
        PklValue::String(value)
    }
}

impl From<Vec<PklValue>> for PklValue {
    fn from(value: Vec<PklValue>) -> Self {
        PklValue::List(value)
    }
}

impl From<HashMap<String, PklValue>> for PklValue {
    fn from(value: HashMap<String, PklValue>) -> Self {
        PklValue::Object(value)
    }
}

impl From<(String, HashMap<String, PklValue>)> for PklValue {
    fn from(value: (String, HashMap<String, PklValue>)) -> Self {
        PklValue::ClassInstance(value.0, value.1)
    }
}

impl From<Duration> for PklValue {
    fn from(value: Duration) -> Self {
        PklValue::Duration(value)
    }
}

impl From<Byte> for PklValue {
    fn from(value: Byte) -> Self {
        PklValue::DataSize(value)
    }
}

impl From<()> for PklValue {
    fn from(_: ()) -> Self {
        PklValue::Null
    }
}

#[derive(Debug, Clone)]
pub struct PklTable {
    pub variables: HashMap<String, PklValue>,
}

impl PartialEq for PklTable {
    fn eq(&self, other: &Self) -> bool {
        if self.variables.len() != other.variables.len() {
            return false;
        }

        self.variables
            .iter()
            .all(|(key, value)| other.variables.get(key).map_or(false, |v| *value == *v))
    }
}

impl PklTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Inserts a variable with the given name and value into the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to insert.
    /// * `value` - The value of the variable to insert.
    ///
    /// # Returns
    ///
    /// An `Option` containing the previous value associated with the name, if any.
    pub fn insert(&mut self, name: impl Into<String>, value: PklValue) -> Option<PklValue> {
        self.variables.insert(name.into(), value)
    }

    /// Merges another `PklTable` into this table.
    ///
    /// This method takes another `PklTable` and inserts all of its variables into the current table.
    /// If a variable with the same name already exists in the current table, it will be overwritten
    /// with the value from the other table.
    ///
    /// # Arguments
    ///
    /// * `other_table` - The `PklTable` to merge into the current table.
    ///
    /// # Example
    ///
    /// ```
    /// let mut table1 = PklTable::new();
    /// table1.insert("var1", PklValue::Int(1));
    ///
    /// let mut table2 = PklTable::new();
    /// table2.insert("var2", PklValue::Int(2));
    ///
    /// table1.extends(table2);
    ///
    /// assert_eq!(table1.get("var1"), Some(&PklValue::Int(1)));
    /// assert_eq!(table1.get("var2"), Some(&PklValue::Int(2)));
    /// ```
    pub fn extends(&mut self, other_table: PklTable) {
        for (name, value) in other_table.variables {
            self.insert(name, value);
        }
    }

    /// Retrieves the value of a variable with the given name from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `PklValue` associated with the name,
    /// or `None` if the variable is not found.
    pub fn get(&self, name: impl Into<String>) -> Option<&PklValue> {
        self.variables.get(&name.into())
    }

    pub fn import(
        &mut self,
        name: &str,
        local_name: Option<&str>,
        rng: Range<usize>,
    ) -> PklResult<()> {
        match name {
            name if name.starts_with("package://") => {
                return Err(("Package imports not yet supported!".to_owned(), rng))
            }
            name if name.starts_with("pkl:") => return official_pkg::import_pkg(self, name, rng),
            name if name.starts_with("https://") => {
                return Err(("Web imports not yet supported!".to_owned(), rng))
            }
            file_name => {
                let file_content = fs::read_to_string(file_name)
                    .map_err(|e| (format!("Error reading {file_name}: {}", e), rng))?;

                let mut pkl = Pkl::new();
                pkl.parse(&file_content)?;
                let hash = pkl.table.variables;

                if let Some(local) = local_name {
                    self.insert(local, hash.into());
                    return Ok(());
                }

                self.variables.extend(hash);
            }
        };

        Ok(())
    }

    /// Evaluates an expression in the current context.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression to evaluate.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the evaluated value or an error message with the range.
    pub fn evaluate(&self, expr: PklExpr) -> PklResult<PklValue> {
        match expr {
            PklExpr::Identifier(Identifier(id, range)) => self
                .get(id)
                .cloned()
                .ok_or_else(|| (format!("unknown variable `{}`", id), range)),
            PklExpr::Value(value) => self.evaluate_value(value),
            PklExpr::MemberExpression(base_expr, indexor, range) => {
                let base = self.evaluate(*base_expr)?;

                match indexor {
                    ExprMember::Identifier(Identifier(property, _)) => match base {
                        PklValue::Int(int) => match_int_props_api(int, property, range),
                        PklValue::Float(float) => match_float_props_api(float, property, range),
                        PklValue::Object(hashmap) => {
                            if let Some(data) = hashmap.get(property) {
                                Ok(data.to_owned())
                            } else {
                                Err((
                                    format!("Object does not possess a '{property}' field"),
                                    range,
                                ))
                            }
                        }
                        PklValue::String(s) => match_string_props_api(&s, property, range),
                        PklValue::ClassInstance(_class_name, hashmap) => {
                            if let Some(data) = hashmap.get(property) {
                                Ok(data.to_owned())
                            } else {
                                Err((
                                    format!("Object does not possess a '{property}' field"),
                                    range,
                                ))
                            }
                        }
                        PklValue::DataSize(byte) => {
                            match_data_size_props_api(byte, property, range)
                        }
                        PklValue::Duration(duration) => {
                            match_duration_props_api(duration, property, range)
                        }
                        PklValue::List(list) => match_list_props_api(list, property, range),

                        _ => Err((
                            format!("Indexing of value '{:?}' not yet supported", base),
                            range,
                        )),
                    },
                    ExprMember::FuncCall(FuncCall(Identifier(fn_name, _), values, _)) => {
                        // here are method calls
                        let args = self.evaluate_fn_args(values)?;

                        match base {
                            PklValue::Bool(bool) => {
                                match_bool_methods_api(bool, fn_name, args, range)
                            }
                            PklValue::Int(int) => match_int_methods_api(int, fn_name, args, range),
                            PklValue::Float(float) => {
                                match_float_methods_api(float, fn_name, args, range)
                            }
                            PklValue::Object(hashmap) => {
                                // need to allow functions as fields of objects
                                if let Some(data) = hashmap.get(fn_name) {
                                    Ok(data.to_owned())
                                } else {
                                    Err((
                                        format!("Object does not possess a '{fn_name}' field"),
                                        range,
                                    ))
                                }
                            }
                            PklValue::String(s) => {
                                // we should directly use s not &s
                                match_string_methods_api(&s, fn_name, args, range)
                            }
                            PklValue::ClassInstance(_class_name, hashmap) => {
                                if let Some(data) = hashmap.get(fn_name) {
                                    Ok(data.to_owned())
                                } else {
                                    Err((
                                        format!("Object does not possess a '{fn_name}' field"),
                                        range,
                                    ))
                                }
                            }
                            PklValue::DataSize(byte) => {
                                match_data_size_methods_api(byte, fn_name, args, range)
                            }
                            PklValue::Duration(duration) => {
                                match_duration_methods_api(duration, fn_name, args, range)
                            }
                            PklValue::List(list) => match_list_props_api(list, fn_name, range),

                            _ => Err((
                                format!("Indexing of value '{:?}' not yet supported", base),
                                range,
                            )),
                        }
                    }
                }
            }
            PklExpr::FuncCall(FuncCall(Identifier(name, _), args, _rng)) => {
                // all function calls
                match name {
                    "List" => self.evaluate_list(args),
                    _ => todo!(),
                }
            }
        }
    }

    /// Evaluates an AST PKL value in the current context.
    ///
    /// # Arguments
    ///
    /// * `value` - The AST PKL value to evaluate.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the evaluated value or an error message with the range.
    fn evaluate_value(&self, value: AstPklValue) -> PklResult<PklValue> {
        let result = match value {
            AstPklValue::Bool(b, _) => PklValue::Bool(b),
            AstPklValue::Float(f, _) => PklValue::Float(f),
            AstPklValue::Int(i, _) => PklValue::Int(i),
            AstPklValue::Null(_) => PklValue::Null,
            AstPklValue::String(s, _) | AstPklValue::MultiLineString(s, _) => {
                PklValue::String(s.to_owned())
            }
            AstPklValue::List(values, _) => self.evaluate_list(values)?,
            AstPklValue::Object(o) => self.evaluate_object(o)?,
            AstPklValue::ClassInstance(a, b, _) => self.evaluate_class_instance(a, b)?,
            AstPklValue::AmendedObject(a, b, _) => self.evaluate_amended_object(*a, b)?,
            AstPklValue::AmendingObject(a, b, rng) => self.evaluate_amending_object(a, b, rng)?,
        };

        Ok(result)
    }

    fn evaluate_object(&self, o: ExprHash) -> PklResult<PklValue> {
        let new_hash: Result<HashMap<_, _>, _> =
            o.0.into_iter()
                .map(|(name, expr)| {
                    let evaluated_expr = self.evaluate(expr)?;
                    Ok((name.into(), evaluated_expr))
                })
                .collect();

        new_hash.map(PklValue::Object)
    }

    fn evaluate_fn_args(&self, values: Vec<PklExpr>) -> PklResult<Vec<PklValue>> {
        let new_hash: Result<Vec<_>, _> = values
            .into_iter()
            .map(|expr| {
                let evaluated_expr = self.evaluate(expr)?;
                Ok(evaluated_expr)
            })
            .collect();

        new_hash
    }

    fn evaluate_list(&self, values: Vec<PklExpr>) -> PklResult<PklValue> {
        let new_hash = self.evaluate_fn_args(values);

        new_hash.map(PklValue::List)
    }

    fn evaluate_class_instance(&self, a: &str, b: ExprHash) -> PklResult<PklValue> {
        let new_hash: Result<HashMap<_, _>, _> =
            b.0.into_iter()
                .map(|(name, expr)| {
                    let evaluated_expr = self.evaluate(expr)?;
                    Ok((name.into(), evaluated_expr))
                })
                .collect();

        new_hash.map(|h| PklValue::ClassInstance(a.into(), h))
    }

    fn evaluate_amending_object(
        &self,
        a: &str,
        b: ExprHash,
        rng: Range<usize>,
    ) -> PklResult<PklValue> {
        let other_object = match self.get(a) {
            Some(PklValue::Object(hash)) => hash,
            _ => return Err((format!("Unknown object `{}`", a), rng)),
        };

        let mut new_hash = other_object.clone();
        for (name, expr) in b.0 {
            new_hash.insert(name.into(), self.evaluate(expr)?);
        }

        Ok(PklValue::Object(new_hash))
    }

    fn evaluate_amended_object(&self, a: AstPklValue, b: ExprHash) -> PklResult<PklValue> {
        let first_object = match self.evaluate_value(a)? {
            PklValue::Object(o) => o,
            _ => unreachable!("should not be reached due to the parser work"),
        };

        let mut new_hash = first_object;
        for (name, expr) in b.0 {
            new_hash.insert(name.into(), self.evaluate(expr)?);
        }

        Ok(PklValue::Object(new_hash))
    }
}

pub fn ast_to_table(ast: Vec<PklStatement>) -> PklResult<PklTable> {
    let mut table = PklTable::new();

    let mut in_body = false;

    for statement in ast {
        match statement {
            PklStatement::Constant(name, expr, _) => {
                in_body = true;
                table.insert(name, table.evaluate(expr)?);
            }
            PklStatement::Import(value, local_name, rng) => {
                if in_body {
                    return Err((
                        "Import statements must be before document body".to_owned(),
                        rng,
                    ));
                }

                table.import(value, local_name, rng)?;
            }
        }
    }

    Ok(table)
}
