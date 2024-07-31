use crate::{
    errors::PklError,
    parser::{
        expr::{class::ClassInstance, fn_call::FuncCall, member_expr::ExprMember, PklExpr},
        statement::{
            amends::Amends,
            class::ClassDeclaration,
            extends::Extends,
            import::Import,
            module::Module,
            property::{Property, PropertyKind},
            typealias::TypeAlias,
            PklStatement,
        },
        types::AstPklType,
        value::AstPklValue,
        ExprHash, Identifier,
    },
    PklResult,
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
use import::Importer;
use logos::Span;
use types::PklType;
use utils::spelling::check_closest_word;
use value::PklValue;

pub mod base;
mod import;
mod utils;

pub mod class;
pub mod types;
pub mod value;

#[derive(Debug, Clone, PartialEq)]
pub enum PklMember {
    Value {
        value: PklValue,
        is_local: bool,
        is_const: bool,
        is_fixed: bool,
        is_amended: bool,
        is_extended: bool,
    },
    Class {
        value: ClassSchema,
        is_local: bool,
        is_amended: bool,
        is_extended: bool,
    },
    // Function {
    //     value: ClassSchema,
    //     is_local: bool,
    // },
}

impl PklMember {
    pub fn value(value: PklValue) -> Self {
        Self::Value {
            value,
            is_local: false,
            is_const: false,
            is_fixed: false,
            is_amended: false,
            is_extended: false,
        }
    }
    pub fn schema(value: ClassSchema) -> Self {
        Self::Class {
            value,
            is_local: false,
            is_amended: false,
            is_extended: false,
        }
    }
    pub fn set_const(mut self) -> Self {
        match &mut self {
            PklMember::Value { is_const, .. } => *is_const = true,
            PklMember::Class { .. } => (),
        };
        self
    }
    pub fn set_local(mut self) -> Self {
        match &mut self {
            PklMember::Value { is_local, .. } => *is_local = true,
            PklMember::Class { is_local, .. } => *is_local = true,
        };
        self
    }
    pub fn set_fixed(mut self) -> Self {
        match &mut self {
            PklMember::Value { is_fixed, .. } => *is_fixed = true,
            PklMember::Class { .. } => (),
        };
        self
    }
    pub fn set_amended(mut self) -> Self {
        match &mut self {
            PklMember::Value { is_amended, .. } => *is_amended = true,
            PklMember::Class { is_amended, .. } => *is_amended = true,
        };
        self
    }
    pub fn set_extended(mut self) -> Self {
        match &mut self {
            PklMember::Value { is_extended, .. } => *is_extended = true,
            PklMember::Class { is_extended, .. } => *is_extended = true,
        };
        self
    }

    pub fn extract_value(self) -> Option<PklValue> {
        match self {
            PklMember::Value { value, .. } => Some(value),
            PklMember::Class { .. } => None,
        }
    }
    pub fn extract_schema(self) -> Option<ClassSchema> {
        match self {
            PklMember::Value { .. } => None,
            PklMember::Class { value, .. } => Some(value),
        }
    }
    pub fn is_amended(&self) -> bool {
        match self {
            PklMember::Value { is_amended, .. } => *is_amended,
            PklMember::Class { is_amended, .. } => *is_amended,
        }
    }
    pub fn is_extended(&self) -> bool {
        match self {
            PklMember::Value { is_extended, .. } => *is_extended,
            PklMember::Class { is_extended, .. } => *is_extended,
        }
    }
    pub fn is_local(&self) -> bool {
        match self {
            PklMember::Value { is_local, .. } => *is_local,
            PklMember::Class { is_local, .. } => *is_local,
        }
    }
    pub fn is_const(&self) -> bool {
        match self {
            PklMember::Value { is_const, .. } => *is_const,
            PklMember::Class { is_local, .. } => false,
        }
    }
    pub fn is_fixed(&self) -> bool {
        match self {
            PklMember::Value { is_fixed, .. } => *is_fixed,
            PklMember::Class { is_local, .. } => false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PklTable {
    pub importer: Importer,

    pub module_name: Option<String>,
    pub is_open: bool,

    pub members: HashMap<String, PklMember>,
}

impl PartialEq for PklTable {
    fn eq(&self, other: &Self) -> bool {
        if self.members.len() != other.members.len() {
            return false;
        }

        self.members
            .iter()
            .all(|(key, value)| other.members.get(key).map_or(false, |v| *value == *v))
    }
}

impl PklTable {
    pub fn is_empty(&self) -> bool {
        self.members.is_empty() & self.module_name.is_none()
    }

    /// Inserts a member with the given name and value into the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the member to insert.
    /// * `value` - The value of the member to insert.
    ///
    /// # Returns
    ///
    /// An `Option` containing the previous value associated with the name, if any.
    pub fn insert(&mut self, name: impl Into<String>, value: PklMember) -> Option<PklMember> {
        self.members.insert(name.into(), value)
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
    /// table1.insert("var1", PklValue::Int(1).into());
    ///
    /// let mut table2 = PklTable::new();
    /// table2.insert("var2", PklValue::Int(2));
    ///
    /// table1.extend(table2);
    ///
    /// assert_eq!(table1.get("var1"), Some(&PklValue::Int(1).into()));
    /// assert_eq!(table1.get("var2"), Some(&PklValue::Int(2)));
    /// ```
    pub fn extend(&mut self, other_table: PklTable) {
        self.members.extend(other_table.members);
    }

    /// Retrieves the value of a member with the given name from the context.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the member to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `PklValue` associated with the name,
    /// or `None` if the variable is not found.
    pub fn get(&self, name: impl AsRef<str>) -> Option<&PklMember> {
        self.members.get(name.as_ref())
    }

    pub fn get_schema(&self, name: impl AsRef<str>) -> Option<ClassSchema> {
        self.get(name)
            .map(|member| member.extract_schema())
            .flatten()
    }
    pub fn get_value(&self, name: impl AsRef<str>) -> Option<PklValue> {
        self.get(name)
            .map(|member| member.extract_value())
            .flatten()
    }

    pub fn import(
        &mut self,
        module_uri: &str,
        local_name: Option<&str>,
        span: Span,
    ) -> PklResult<()> {
        let imported_table = self
            .importer
            .import(module_uri, span.to_owned())
            .map_err(|e| e.with_file_name(module_uri.to_owned()))?;

        fn transform_map(original: HashMap<String, PklMember>) -> HashMap<String, PklValue> {
            original
                .into_iter()
                .filter_map(|(key, member)| member.extract_value().map(|v| (key, v)))
                .collect()
        }

        let value = transform_map(imported_table.members);
        let member = PklMember::value(value.into()).set_const().set_local();

        if let Some(local) = local_name {
            self.insert(local, member);
            return Ok(());
        }

        let name = Importer::construct_name_from_uri(module_uri, span);
        self.insert(name, member);

        Ok(())
    }

    pub fn amends(&mut self, module_uri: &str, span: Span) -> PklResult<()> {
        let amended_table = self
            .importer
            .amends(module_uri, span.to_owned())
            .map_err(|e| e.with_file_name(module_uri.to_owned()))?;

        // let module_name = match amended_table.module_name.to_owned() {
        //     Some(name) => name,
        //     None => Importer::construct_name_from_uri(module_uri, span.to_owned()),
        // };

        self.extend(amended_table);

        Ok(())
    }

    /// Interpret an pkl extends clause,
    /// reads the given module uri and
    /// extends the current file if the
    /// other module is an open module.
    pub fn extends(&mut self, module_uri: &str, span: Span) -> PklResult<()> {
        let extended_table = self
            .importer
            .extends(module_uri, span.to_owned())
            .map_err(|e| e.with_file_name(module_uri.to_owned()))?;

        if !extended_table.is_open {
            return Err((
                format!("Cannot extend module '{module_uri}': module is not declared as open"),
                span,
            )
                .into());
        }

        // let module_name = match extended_table.module_name.to_owned() {
        //     Some(name) => name,
        //     None => Importer::construct_name_from_uri(module_uri, span),
        // };

        self.extend(extended_table);

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
                .map(|v| v.extract_value())
                .flatten()
                .ok_or_else(|| (format!("unknown property `{}`", id), range).into()),
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
                                )
                                    .into())
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
                                )
                                    .into())
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
                        )
                            .into()),
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
                                    )
                                        .into())
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
                                    )
                                        .into())
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
                            )
                                .into()),
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
                        )
                            .into()),
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
                    )
                        .into()),
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
            )
                .into()),
        };

        let new_hash: Result<HashMap<_, _>, PklError> =
            b.0.into_iter()
                .map(|(name, expr)| {
                    let evaluated_expr = self.evaluate(expr)?;
                    Ok((name.into(), evaluated_expr))
                })
                .collect();

        let schema = match self.get_schema(a.0) {
            Some(schema) => schema,
            None => return Err((format!("Unknown class '{}'", a.0), a.1).into()),
        };

        let found_schema = new_hash?;

        for k in schema.keys() {
            if !found_schema.contains_key(k) {
                return Err((format!("Missing key '{k}' in instance of {}", a.0), b.1).into());
            }
        }
        for k in found_schema.keys() {
            if !schema.contains_key(k) {
                return Err((format!("Unknown key '{k}' in instance of {}", a.0), b.1).into());
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
                )
                    .into());
            }
        }

        Ok(PklValue::ClassInstance(a.0.into(), found_schema))
    }

    fn evaluate_amending_object(&self, a: &str, b: ExprHash, span: Span) -> PklResult<PklValue> {
        let other_object = match self.get_value(a) {
            Some(PklValue::Object(hash)) => hash,
            _ => return Err((format!("Unknown object `{}`", a), span).into()),
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
                    return Err(("A file cannot have 2 module clauses".to_owned(), span).into());
                }
                if amends_found || import_found || in_body {
                    return Err((
                        "Module clause must be at the start of the file".to_owned(),
                        span,
                    )
                        .into());
                }

                table.module_name = Some(full_name.0.to_owned());
                table.is_open = is_open;
                module_clause_found = true;
            }
            PklStatement::AmendsClause(Amends { name, span }) => {
                if extends_found {
                    return Err((
                        "Cannot have both an amends clause and an extends clause".to_owned(),
                        span,
                    )
                        .into());
                }
                if amends_found {
                    return Err(("A file cannot have 2 amends clauses".to_owned(), span).into());
                }
                if import_found || in_body {
                    return Err((
                        "Amends clause must be before import clauses and file body".to_owned(),
                        span,
                    )
                        .into());
                }

                table.amends(name, span)?;
                amends_found = true;
            }
            PklStatement::ExtendsClause(Extends { name, span }) => {
                if amends_found {
                    return Err((
                        "Cannot have both an amends clause and an extends clause".to_owned(),
                        span,
                    )
                        .into());
                }
                if import_found || in_body {
                    return Err((
                        "Extends clause must be before import clauses and file body".to_owned(),
                        span,
                    )
                        .into());
                }

                table.extends(name, span)?;
                extends_found = true;
            }
            PklStatement::Import(Import {
                name,
                local_name,
                span,
            }) => {
                // need to handle globbed import as well

                if in_body {
                    return Err((
                        "Keyword `import` is not allowed here, it should be before file body. (If you must use this name as identifier, enclose it in backticks.)".to_owned(),
                        span,
                    )
                        .into());
                }

                table.import(name, local_name, span)?;
                import_found = true;
            }
            PklStatement::TypeAlias(TypeAlias { .. }) => {
                // need to interpret typealiases
                // store somewhere in the PklTable
                // the types
                // todo!
            }

            PklStatement::Property(Property {
                name,
                _type,
                value,
                span,
            }) => {
                in_body = true;

                let evaluated_value = table.evaluate_in_variable(value, _type)?;

                // checks if type corresponds to value
                if let Some(_type) = _type {
                    let span = _type.span();
                    let true_type: PklType = _type.into();
                    if !evaluated_value.is_instance_of(&true_type) {
                        return Err((
                            format!(
                                "Expected value of type `{}`, but got value '{}'",
                                true_type, name.0
                            ),
                            span,
                        )
                            .into());
                    }
                }

                if let Some(previous_prop) = table.insert(name.0, PklMember::value(evaluated_value))
                {
                    if !previous_prop.is_amended() || !previous_prop.is_extended() {
                        return Err(
                            (format!("Duplicate definition of member '{}'", name.0), span).into(),
                        );
                    }
                }
            }
            PklStatement::Class(declaration) => {
                in_body = true;
                handle_class(&mut table, declaration)?;
            }

            // there three prefixes below can be before a Class,
            // a TypeAlias, a Property or a function
            // in any order
            PklStatement::Local(stmt, span) => {
                in_body = true;

                match *stmt {
                    PklStatement::Property(_) => todo!(),
                    PklStatement::Class(_) => todo!(),
                    PklStatement::TypeAlias(_) => todo!(),
                    PklStatement::Const(_, _) => todo!(),
                    PklStatement::Local(_, span) => todo!(),

                    PklStatement::Fixed(_, span) => {
                        return Err((
                            format!("Modifier `fixed` is redundant here; just use `local`."),
                            span,
                        )
                            .into())
                    }
                    PklStatement::ModuleClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::AmendsClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::ExtendsClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::Import(imp) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                }
            }

            PklStatement::Const(stmt, span) => {
                in_body = true;

                match *stmt {
                    PklStatement::Property(_) => todo!(),
                    PklStatement::Const(_, _) => todo!(),
                    PklStatement::Fixed(_, span) => todo!(),
                    PklStatement::Local(_, span) => todo!(),

                    PklStatement::Class(stmt) => {
                        return Err((stmt.modifier_not_applicable_err("const"), stmt.span).into())
                    }
                    PklStatement::TypeAlias(stmt) => {
                        return Err((stmt.modifier_not_applicable_err("const"), stmt.span).into())
                    }
                    PklStatement::ModuleClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::AmendsClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::ExtendsClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::Import(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                }
            }
            PklStatement::Fixed(stmt, span) => {
                in_body = true;

                match *stmt {
                    PklStatement::Property(_) => todo!(),

                    PklStatement::Const(_, _) => todo!(),

                    PklStatement::Class(stmt) => {
                        return Err((stmt.modifier_not_applicable_err("fixed"), stmt.span).into())
                    }
                    PklStatement::TypeAlias(stmt) => {
                        return Err((stmt.modifier_not_applicable_err("fixed"), stmt.span).into())
                    }
                    PklStatement::Fixed(_, span) => todo!(),
                    PklStatement::Local(_, span) => {
                        return Err((
                            format!("Modifier `fixed` is redundant here; just use `local`."),
                            span,
                        )
                            .into())
                    }

                    PklStatement::ModuleClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::AmendsClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::ExtendsClause(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                    PklStatement::Import(stmt) => {
                        return Err((stmt.not_allowed_here_err(), stmt.span).into())
                    }
                }
            }
        }
    }

    Ok(table)
}

fn handle_property(
    table: &mut PklTable,
    Property {
        name,
        _type,
        value,
        span,
    }: Property,
) -> PklResult<()> {
    let evaluated_value = table.evaluate_in_variable(value, _type.clone())?;

    // checks for spelling errors
    if let Some(vars) = table.module_data.get_variables() {
        let vars = vars
            .iter()
            .filter_map(|x| {
                if x.1.as_str() != name.0 {
                    Some(x.1.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>();

        if !vars.is_empty() && name.0.len() > 2 {
            match check_closest_word(name.0, vars.as_slice(), 1) {
                Some(closest) => {
                    return Err((
                        format!(
                            "Did you mean to write '{}' instead of '{}'?",
                            closest, name.0
                        ),
                        name.1,
                    )
                        .into())
                }
                None => (),
            };
        }
    }

    // checks if user creates variables
    // not present in amended module
    if let Some(amended_vars) = table.module_data.get_amended_variables() {
        if !matches!(kind, PropertyKind::Local)
            && !amended_vars.iter().any(|x| x.1.as_str() == name.0)
        {
            return Err((
                format!(
                    "Cannot add variable '{}' to amended module '{}'",
                    name.0,
                    table.module_data.name().unwrap(/* safe: if amended_variables.is_some() then name is too */)
                ),
                name.1).into());
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
            )
                .into());
        }
    }

    // assign variable
    // if reassigned then checks
    // if var is amended/extended then allows
    // assignment in new module
    // otherwise throws an Error
    if let Some((prop_kind, prop_name)) = table.insert(name.0, (kind, evaluated_value)) {
        if matches!(prop_kind, PropertyKind::Fixed) {
            return Err((
                format!("cannot assign to fixed property '{}'", name.0),
                span,
            )
                .into());
        }
        if matches!(prop_kind, PropertyKind::Const) {
            return Err((
                format!("cannot assign to const property '{}'", name.0),
                span,
            )
                .into());
        }
        if matches!(prop_kind, PropertyKind::ConstLocal) {
            return Err((
                format!("cannot assign to const local property '{}'", name.0),
                span,
            )
                .into());
        }
        match table.module_data.get_variables_mut() {
            Some(vars) => {
                if let Some(pos) = vars.iter().position(|x| x.1.as_str() == name.0) {
                    vars.remove(pos);
                } else {
                    return Err((format!("cannot add new property '{}'", name.0), span).into());
                }
            }
            None => return Err((format!("cannot add new property '{}'", name.0), span).into()),
        }
    }

    Ok(())
}

fn handle_class(table: &mut PklTable, declaration: ClassDeclaration) -> PklResult<()> {
    let (name, schema) = generate_class_schema(declaration);

    // checks for spelling errors
    if let Some(vars) = table.module_data.get_schemas() {
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
                    )
                        .into())
                }
                None => (),
            };
        }
    }

    // checks if adding variables to amending module
    // that is not in amended module
    if let Some(amended_schemas) = table.module_data.get_amended_schemas() {
        if !amended_schemas.contains(&name.0.to_owned()) {
            return Err((
                format!(
                    "Cannot define class '{}', class is not defined inside of amended module '{}'",
                    name.0,
                    table.module_data.name().unwrap(/* safe: if amended_variables.is_some() then name is too */)
                ),
                name.1).into());
        }
    }

    // assign schema
    // if reassigned then checks
    // if schema is amended/extended then allows
    // assignment in new module
    // otherwise throws an Error
    if let Some(_) = table.add_schema(name.0, schema) {
        // variables can be either amended or extended
        match table.module_data.get_schemas_mut() {
            Some(vars) => {
                if let Some(pos) = vars.iter().position(|x| x == &name.0) {
                    vars.remove(pos);
                } else {
                    return Err((format!("Cannot reassign variable '{}'", name.0), name.1).into());
                }
            }
            None => return Err((format!("Cannot reassign variable '{}'", name.0), name.1).into()),
        }
    }

    Ok(())
}
