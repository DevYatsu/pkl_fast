use crate::{
    parser::{
        expr::{class::ClassInstance, fn_call::FuncCall, member_expr::ExprMember, PklExpr},
        statement::{
            amends::Amends, constant::Constant, extends::Extends, import::Import, module::Module,
            typealias::TypeAlias, PklStatement,
        },
        types::AstPklType,
        value::AstPklValue,
        ExprHash, Identifier, PklResult,
    },
    utils::PathUriName,
    Pkl,
};
use base::{
    bool_api::match_bool_methods_api,
    data_size::{match_data_size_methods_api, match_data_size_props_api},
    duration::{match_duration_methods_api, match_duration_props_api},
    float_api::{match_float_methods_api, match_float_props_api},
    int_api::{match_int_methods_api, match_int_props_api},
    list_api::match_list_props_api,
    string_api::{match_string_methods_api, match_string_props_api},
};
use class::{generate_class_schema, ClassSchema};
use hashbrown::HashMap;
use logos::Span;
use spelling::check_closest_word;
use std::{default, fs};
use types::PklType;
use value::PklValue;

pub mod base;
mod official_pkg;
mod web_import;

mod spelling;

pub mod class;
pub mod types;
pub mod value;

#[derive(Debug, Clone, Default)]
pub enum ModuleData {
    Extended {
        module_name: String,
        variables: Vec<String>,
    },
    Amended {
        module_name: String,
        variables: Vec<String>,
    },
    #[default]
    None,
}

impl ModuleData {
    pub fn get_variables(&mut self) -> Option<&Vec<String>> {
        match self {
            ModuleData::Extended { variables, .. } => Some(variables),
            ModuleData::Amended { variables, .. } => Some(variables),
            ModuleData::None => None,
        }
    }
    pub fn get_variables_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            ModuleData::Extended { variables, .. } => Some(variables),
            ModuleData::Amended { variables, .. } => Some(variables),
            ModuleData::None => None,
        }
    }
    pub fn name(&self) -> Option<&str> {
        match self {
            ModuleData::Extended { module_name, .. } => Some(module_name),
            ModuleData::Amended { module_name, .. } => Some(module_name),
            ModuleData::None => None,
        }
    }

    pub fn get_amended_variables(&self) -> Option<&Vec<String>> {
        match self {
            ModuleData::Extended { .. } => None,
            ModuleData::Amended { variables, .. } => Some(variables),
            ModuleData::None => None,
        }
    }
    pub fn get_amended_variables_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            ModuleData::Extended { .. } => None,
            ModuleData::Amended { variables, .. } => Some(variables),
            ModuleData::None => None,
        }
    }
    pub fn get_extended_variables(&self) -> Option<&Vec<String>> {
        match self {
            ModuleData::Extended { variables, .. } => Some(variables),
            ModuleData::Amended { .. } => None,
            ModuleData::None => None,
        }
    }
    pub fn get_extended_variables_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            ModuleData::Extended { variables, .. } => Some(variables),
            ModuleData::Amended { .. } => None,
            ModuleData::None => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PklTable {
    pub variables: HashMap<String, PklValue>,
    pub schemas: HashMap<String, ClassSchema>,

    pub module_data: ModuleData,
    pub module_name: Option<String>,
    pub is_open: bool,
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
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty() & self.schemas.is_empty() & self.module_name.is_none()
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

    pub fn add_schema(
        &mut self,
        name: impl Into<String>,
        schema: ClassSchema,
    ) -> Option<ClassSchema> {
        self.schemas.insert(name.into(), schema)
    }
    pub fn get_schema(&self, name: impl AsRef<str>) -> Option<&ClassSchema> {
        self.schemas.get(name.as_ref())
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
    /// table1.extend(table2);
    ///
    /// assert_eq!(table1.get("var1"), Some(&PklValue::Int(1)));
    /// assert_eq!(table1.get("var2"), Some(&PklValue::Int(2)));
    /// ```
    pub fn extend(&mut self, other_table: PklTable) {
        self.variables.extend(other_table.variables);
        self.schemas.extend(other_table.schemas);
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
        module_uri: &str,
        local_name: Option<&str>,
        span: Span,
    ) -> PklResult<()> {
        match module_uri {
            uri if uri.starts_with("package://") => return web_import::import_pkg(self, uri, span),
            uri if uri.starts_with("pkl:") => return official_pkg::import_pkg(self, uri, span),
            uri if uri.starts_with("https://") => return web_import::import_https(self, uri, span),
            file_path => {
                let file_content = fs::read_to_string(file_path)
                    .map_err(|e| (format!("Error reading {file_path}: {}", e), span))?;

                let mut pkl = Pkl::new();
                pkl.parse(&file_content)?;
                let hash = pkl.table.variables;

                if let Some(name) = local_name {
                    self.insert(name, hash.into());
                    return Ok(());
                }

                let name = file_path.get_uri_name();
                self.insert(name, hash.into());
            }
        };

        Ok(())
    }

    pub fn amends(&mut self, module_uri: &str, span: Span) -> PklResult<()> {
        match module_uri {
            uri if uri.starts_with("package://") => return web_import::amends_pkg(self, uri, span),
            uri if uri.starts_with("pkl:") => return official_pkg::amends_pkg(self, uri, span),
            uri if uri.starts_with("https://") => return web_import::amends_http(self, uri, span),
            file_path => {
                let file_content = fs::read_to_string(file_path)
                    .map_err(|e| (format!("Error reading {file_path}: {}", e), span))?;

                let mut pkl = Pkl::new();
                pkl.parse(&file_content)?;

                let amended = pkl
                    .table
                    .variables
                    .keys()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>();

                self.module_data = ModuleData::Amended {
                    module_name: file_path.get_uri_name(),
                    variables: amended,
                };
                self.extend(pkl.table);
            }
        };

        Ok(())
    }

    /// Interpret an pkl extends clause,
    /// reads the given module uri and
    /// extends the current file if the
    /// other module is an open module.
    pub fn extends(&mut self, module_uri: &str, span: Span) -> PklResult<()> {
        match module_uri {
            uri if uri.starts_with("package://") => {
                return web_import::extends_pkg(self, uri, span)
            }
            uri if uri.starts_with("pkl:") => return official_pkg::extends_pkg(self, uri, span),
            uri if uri.starts_with("https://") => return web_import::extends_http(self, uri, span),
            file_path => {
                let file_content = fs::read_to_string(file_path)
                    .map_err(|e| (format!("Error reading {file_path}: {}", e), span.to_owned()))?;

                let mut pkl = Pkl::new();
                pkl.parse(&file_content)?;

                if !pkl.table.is_open {
                    return Err((
                        format!(
                            "Cannot extend module '{file_path}': module is not declared as open"
                        ),
                        span,
                    ));
                }

                let extended = pkl
                    .table
                    .variables
                    .keys()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>();

                self.module_data = ModuleData::Extended {
                    module_name: file_path.get_uri_name(),
                    variables: extended,
                };
                self.extend(pkl.table);
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
            PklExpr::FuncCall(FuncCall(Identifier(name, _), args, _span)) => {
                // all function calls
                match name {
                    "List" => self.evaluate_list(args),
                    _ => todo!(),
                }
            }
        }
    }

    /// Evaluates an expression in the context of a variable declaration.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression to evaluate.
    /// * `opt_type` - If written, the user-defined type of the expression to evaluate.
    ///
    /// # Returns
    ///
    /// A `PklResult` containing the evaluated value or an error message with the range.
    pub fn evaluate_in_variable(
        &self,
        expr: PklExpr,
        opt_type: Option<AstPklType>,
    ) -> PklResult<PklValue> {
        match expr {
            PklExpr::Value(v) => match v {
                AstPklValue::ClassInstance(ClassInstance(a, b, span)) => match (a, opt_type) {
                    (Some(id), Some(_type)) => match _type {
                        AstPklType::Basic(ref value, _) if value == &id.0 => self
                            .evaluate_class_instance(Some(Identifier(value, b.1.to_owned())), b)
                            .map(PklValue::into),
                        AstPklType::Basic(value, type_span) => Err((
                            format!("Type '{value}' and '{}' do not match.", id.0),
                            type_span,
                        )),
                        AstPklType::StringLiteral(_, _) => todo!(),
                        AstPklType::Union(_, _) => todo!(),
                        AstPklType::Nullable(_) => todo!(),
                        AstPklType::WithAttributes {
                            name, attributes, ..
                        } => todo!(),
                        AstPklType::WithRequirement {
                            base_type,
                            requirements,
                            ..
                        } => todo!(),
                    },
                    (Some(id), None) => self
                        .evaluate_class_instance(Some(id), b)
                        .map(PklValue::into),
                    (None, Some(_type)) => match _type {
                        AstPklType::Basic(ref value, _) => self
                            .evaluate_class_instance(Some(Identifier(value, b.1.to_owned())), b)
                            .map(PklValue::into),
                        AstPklType::StringLiteral(_, _) => todo!(),
                        AstPklType::Union(_, _) => todo!(),
                        AstPklType::Nullable(_) => todo!(),
                        AstPklType::WithAttributes {
                            name, attributes, ..
                        } => todo!(),
                        AstPklType::WithRequirement {
                            base_type,
                            requirements,
                            ..
                        } => todo!(),
                    },
                    (None, None) => Err((
                        "Unknown class instance, add the name of the class!".to_owned(),
                        span,
                    )),
                },
                _ => self.evaluate_value(v),
            },
            _ => self.evaluate(expr),
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
            AstPklValue::ClassInstance(ClassInstance(a, b, _)) => {
                self.evaluate_class_instance(a, b)?
            }
            AstPklValue::AmendedObject(a, b, _) => self.evaluate_amended_object(*a, b)?,
            AstPklValue::AmendingObject(a, b, span) => self.evaluate_amending_object(a, b, span)?,
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

    /// Function should only be called when not in a variable declaration
    fn evaluate_class_instance(
        &self,
        a: Option<Identifier<'_>>,
        b: ExprHash,
    ) -> PklResult<PklValue> {
        let a = match a {
            Some(a) => a,
            None => return Err((
                "Class Instance name is expected to be provided when not in a constant declaration"
                    .to_owned(),
                b.1,
            )),
        };

        let new_hash: Result<HashMap<_, _>, _> =
            b.0.into_iter()
                .map(|(name, expr)| {
                    let evaluated_expr = self.evaluate(expr)?;
                    Ok((name.into(), evaluated_expr))
                })
                .collect();

        let schema = match self.get_schema(a.0) {
            Some(schema) => schema,
            None => return Err((format!("Unknown class '{}'", a.0), a.1)),
        };

        let found_schema = new_hash?;

        for k in schema.keys() {
            if !found_schema.contains_key(k) {
                return Err((format!("Missing key '{k}' in instance of {}", a.0), b.1));
            }
        }
        for k in found_schema.keys() {
            if !schema.contains_key(k) {
                return Err((format!("Unknown key '{k}' in instance of {}", a.0), b.1));
            }
        }

        // Todo: Check if the types of the values are correct in the found_schema
        for (k, v) in &found_schema {
            let _type = schema.get(k).unwrap();
            if !v.is_instance_of(_type) {
                return Err((
                    format!(
                        "Invalid type for key '{k}', not an instance of '{:?}'",
                        _type
                    ),
                    b.1,
                ));
            }
        }

        Ok(PklValue::ClassInstance(a.0.into(), found_schema))
    }

    fn evaluate_amending_object(&self, a: &str, b: ExprHash, span: Span) -> PklResult<PklValue> {
        let other_object = match self.get(a) {
            Some(PklValue::Object(hash)) => hash,
            _ => return Err((format!("Unknown object `{}`", a), span)),
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
    let mut table = PklTable::default();

    // encountered a body statement
    // == no more import stmt
    let mut in_body = false;
    let mut module_clause_found = false;
    let mut amends_found = false;
    let mut extends_found = false;
    let mut import_found = false;

    for statement in ast {
        match statement {
            PklStatement::ModuleClause(Module {
                full_name,
                span,
                is_open,
            }) => {
                if module_clause_found {
                    return Err(("A file cannot have 2 module clauses".to_owned(), span));
                }
                if amends_found || import_found || in_body {
                    return Err((
                        "Module clause must be at the start of the file".to_owned(),
                        span,
                    ));
                }

                table.module_name = Some(full_name.to_owned());
                table.is_open = is_open;
                module_clause_found = true;
            }
            PklStatement::AmendsClause(Amends { name, span }) => {
                if extends_found {
                    return Err((
                        "Cannot have both an amends clause and an extends clause".to_owned(),
                        span,
                    ));
                }
                if amends_found {
                    return Err(("A file cannot have 2 amends clauses".to_owned(), span));
                }
                if import_found || in_body {
                    return Err((
                        "Amends clause must be before import clauses and file body".to_owned(),
                        span,
                    ));
                }

                table.amends(name, span)?;
                amends_found = true;
            }
            PklStatement::ExtendsClause(Extends { name, span }) => {
                if amends_found {
                    return Err((
                        "Cannot have both an amends clause and an extends clause".to_owned(),
                        span,
                    ));
                }
                if import_found || in_body {
                    return Err((
                        "Extends clause must be before import clauses and file body".to_owned(),
                        span,
                    ));
                }

                table.extends(name, span)?;
                extends_found = true;
            }
            PklStatement::Constant(Constant {
                name, value, _type, ..
            }) => {
                in_body = true;
                handle_constant(&mut table, name, value, _type)?;
            }
            PklStatement::Class(declaration) => {
                in_body = true;

                let (name, schema) = generate_class_schema(declaration);
                table.add_schema(name, schema);
            }
            PklStatement::TypeAlias(TypeAlias { .. }) => {
                // need to interpret typealiases
                // store somewhere in the PklTable
                // the types
                // todo!
            }
            PklStatement::Import(Import {
                name,
                local_name,
                span,
            }) => {
                if in_body {
                    return Err((
                        "Import statements must be before document body".to_owned(),
                        span,
                    ));
                }

                table.import(name, local_name, span)?;
                import_found = true;
            }
        }
    }

    Ok(table)
}

fn handle_constant(
    table: &mut PklTable,
    name: Identifier<'_>,
    value: PklExpr<'_>,
    _type: Option<AstPklType>,
) -> PklResult<()> {
    let evaluated_value = table.evaluate_in_variable(value, _type.clone())?;

    // checks if adding variables to amending module
    // that is not in amended module
    if let Some(amended_vars) = table.module_data.get_amended_variables() {
        if !amended_vars.contains(&name.0.to_owned()) {
            return Err((
                format!(
                    "Cannot add variable '{}' to amended module '{}'",
                    name.0,
                    table.module_data.name().unwrap(/* safe: if amended_variables.is_some() then name is too */)
                ),
                name.1,
            ));
        }
    }

    // checks if type corresponds to value
    if let Some(_type) = _type {
        let span = _type.span();
        let true_type: PklType = _type.into();
        if !evaluated_value.is_instance_of(&true_type) {
            return Err((
                format!(
                    "Type '{}' does not correspond to the value of '{}'",
                    true_type, name.0
                ),
                span,
            ));
        }
    }

    // checks for spelling errors
    if let Some(vars) = table.module_data.get_variables() {
        let vars = vars
            .iter()
            .filter(|x| x != &name.0)
            .map(String::as_str)
            .collect::<Vec<&str>>();

        if !vars.is_empty() {
            match check_closest_word(name.0, vars.as_slice(), 1) {
                Some(closest) => {
                    return Err((
                        format!(
                            "Did you mean to write '{}' instead of '{}'?",
                            closest, name.0
                        ),
                        name.1,
                    ))
                }
                None => (),
            };
        }
    }

    if let Some(_) = table.insert(name.0, evaluated_value) {
        // variables can be either amended or extended
        match table.module_data.get_variables_mut() {
            Some(vars) => {
                if let Some(pos) = vars.iter().position(|x| x == &name.0) {
                    vars.remove(pos);
                } else {
                    return Err((format!("Cannot reassign variable '{}'", name.0), name.1));
                }
            }
            None => return Err((format!("Cannot reassign variable '{}'", name.0), name.1)),
        }
    }

    Ok(())
}
