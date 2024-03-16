use crate::parser::errors::ParsingError;
use crate::parser::statement::comment::{doc_comment, line_comment, multiline_comment};
use crate::parser::statement::Statement;
use crate::parser::statement::{info_statement, module_statement, open_module_statement};

use winnow::ascii::alphanumeric1;
use winnow::combinator::{alt, fail, opt};
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::{dispatch, PResult, Parser};

use self::statement::import::import_statement;
use self::statement::{amends_statement, extends_statement, var_statement, ClassType};

pub mod errors;
mod expression;
mod macros;
mod operator;
pub mod statement;
mod types;

mod utils;
pub mod value;

pub type ParsingResult<T> = PResult<T>;

pub fn parse<'source>(
    file_name: &'source str,
    source: &'source str,
) -> Result<Vec<statement::Statement<'source>>, ParsingError> {
    let mut parser = PklParser::new(file_name, source);

    match parser.parse() {
        Ok(_) => (),
        Err(e) => {
            if let Some(e) = e.clone().into_inner() {
                println!("{:?}", &parser.source_input()[parser.input().len()..]);
                println!("{e}");

                let err_as_str = e.to_string();
                let error_type = err_as_str.trim_start_matches("expected ");
                match error_type {
                    "identifier" | "module name" | "identifier not keyword" => return Err(ParsingError::invalid_id(&parser)),
                    "variable" => return Err(ParsingError::unexpected(&parser, "a valid variable declaration")),
                    "type" => return Err(ParsingError::unexpected(&parser, "a valid type")),
                    "space" => return Err(ParsingError::unexpected(&parser, "a space")),
                    "in" => return Err(ParsingError::unexpected(&parser, "'in'")),
                    "exponential integer" => (),
                    "string literal" => return Err(ParsingError::expected_simple_string(&parser)),
                    "unicode" => return Err(ParsingError::invalid_unicode(&parser)),
                    "opening bracket" | "open bracket" => return Err(ParsingError::unexpected(&parser, "'{'")),
                    "opening brace" | "open brace" => return Err(ParsingError::unexpected(&parser, "'['")),
                    "opening parenthesis" | "open parenthesis" => return Err(ParsingError::unexpected(&parser, "'('")),
                    "closing bracket" | "close bracket" => return Err(ParsingError::unexpected(&parser, "'}'")),
                    "closing brace" | "close brace" => return Err(ParsingError::unexpected(&parser, "']'")),
                    "closing parenthesis" | "close parenthesis" => return Err(ParsingError::unexpected(&parser, "')'")),
                    "expr" => return Err(ParsingError::expected_expression(&parser)),
                    _ => return Err(ParsingError::eof(&parser, error_type)),
                }
            }

            // return eof error
            return Err(ParsingError::eof(&parser, e.to_string().as_str()));
        }
    }

    Ok(parser.statements())
}

#[derive(Debug, Clone)]
/// PklParser is the main parser struct, possessing the `parse` method to parse the tokens in the lexer.
pub struct PklParser<'source> {
    file_path: &'source str,
    statements: Vec<Statement<'source>>,
    input: &'source str,
    source_input: &'source str,
}

impl<'source> PklParser<'source> {
    /// The function to initialize an instance of PklParser.
    pub fn new(name: &'source str, source: &'source str) -> Self {
        Self {
            file_path: name,
            statements: vec![],
            input: source,
            source_input: source,
        }
    }

    pub fn source_input(&self) -> &'source str {
        self.source_input
    }
    pub fn input(&self) -> &'source str {
        self.input
    }
    pub fn statements_ref(&self) -> &Vec<Statement<'source>> {
        &self.statements
    }
    pub fn statements(self) -> Vec<Statement<'source>> {
        self.statements
    }

    /// This function parses the tokens in the lexer.
    ///
    /// To access the parsed statements, use the `statements` field.
    pub fn parse(&mut self) -> ParsingResult<()> {
        loop {
            opt(take_while(0.., |c: char| c.is_newline() || c.is_space()))
                .parse_next(&mut self.input)?;

            if self.input.len() == 0 {
                break;
            }

            let opt_variable_statement = opt(var_statement).parse_next(&mut self.input)?;

            if let Some(s) = opt_variable_statement {
                // line ending is in the parser directly as there is no need for one when the var is an object
                self.statements.push(s);
                continue;
            }

            let statement = dispatch!(alt((alphanumeric1, "@", "///","//", "/*"));
                "amends" => amends_statement,
                "extends" => extends_statement,
                "import" => import_statement,
                "module" => module_statement,
                "@" => info_statement,// need to support values not only string literals
                "open" => open_module_statement, // need to add open class
                "///" => doc_comment,
                "//" => line_comment,
                "/*" => multiline_comment,
                _ => fail,
            )
            .parse_next(&mut self.input)?;

            self.statements.push(statement);
        }

        Ok(())
    }

    fn _parse_basic_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        todo!()
        // self.parse_class_declaration(ClassType::None)
    }
    fn _parse_open_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        todo!()
        // self.parse_class_declaration(ClassType::Open)
    }
    fn _parse_abstract_class_declaration(&mut self) -> ParsingResult<Statement<'source>> {
        todo!()
        // self.parse_class_declaration(ClassType::Abstract)
    }

    fn _parse_class_declaration(&mut self, _type: ClassType) -> ParsingResult<Statement<'source>> {
        todo!()
        // let name = parse_identifier(self)?;
        // let token = retrieve_next_token(self)?;

        // let extends = match token {
        //     PklToken::Extends => {
        //         let value = parse_identifier(self)?;

        //         match retrieve_opt_next_token(self)? {
        //             Some(PklToken::OpenBracket) => (),
        //             Some(PklToken::NewLine)
        //             | Some(PklToken::LineComment)
        //             | Some(PklToken::DocComment) => {
        //                 self.new_line_parsed = true;
        //                 return Ok(Statement::ClassDeclaration {
        //                     name,
        //                     extends: Some(value),
        //                     _type,
        //                     fields: None,
        //                 });
        //             }
        //             _ => {
        //                 return Err(ParsingError::unexpected(
        //                     self,
        //                     "'{' or line ending".to_owned(),
        //                 ))
        //             }
        //         };

        //         Some(value)
        //     }
        //     PklToken::OpenBracket => None,
        //     PklToken::NewLine | PklToken::LineComment | PklToken::DocComment => {
        //         self.new_line_parsed = true;
        //         return Ok(Statement::ClassDeclaration {
        //             name,
        //             extends: None,
        //             _type,
        //             fields: None,
        //         });
        //     }
        //     _ => return Err(ParsingError::unexpected(self, "'{'".to_owned())),
        // };

        // let fields = hashmap_while_not_token2(
        //     self,
        //     PklToken::NewLine,
        //     PklToken::CloseBracket,
        //     &parse_class_field,
        // )?
        // .into();

        // Ok(Statement::ClassDeclaration {
        //     name,
        //     extends,
        //     _type,
        //     fields,
        // })
    }

    // this function is defined here as it uses self.new_line_parsed
    fn _parse_var_statement(
        &mut self,
        name: &'source str,
        is_local: bool,
    ) -> ParsingResult<Statement<'source>> {
        // let token = retrieve_next_token(self)?;

        todo!()

        // let optional_type = match token {
        //     PklToken::OpenBracket => {
        //         let (object, next_token) = parse_object(self, None)?;

        //         match next_token {
        //             Some(PklToken::NewLine)
        //             | Some(PklToken::BlockComment)
        //             | Some(PklToken::DocComment) => {
        //                 self.new_line_parsed = true;
        //             }
        //             None => (),
        //             _ => return Err(ParsingError::unexpected(self, "'='".to_owned())),
        //         };

        //         return Ok(Statement::VariableDeclaration {
        //             name,
        //             value: Expression::Value(object),
        //             optional_type: None,
        //             is_local,
        //         });
        //     }
        //     PklToken::Colon => {
        //         let (variable_type, next_token) = parse_opt_newlines(self, &parse_type)?;

        //         match next_token {
        //             Some(PklToken::EqualSign) => {}
        //             Some(PklToken::NewLine) => {
        //                 self.new_line_parsed = true;

        //                 return Ok(Statement::VariableDeclaration {
        //                     name,
        //                     value: Expression::Value(variable_type.default_value(self)?),
        //                     optional_type: Some(variable_type),
        //                     is_local,
        //                 });
        //             }
        //             None => {
        //                 return Ok(Statement::VariableDeclaration {
        //                     name,
        //                     value: Expression::Value(variable_type.default_value(self)?),
        //                     optional_type: Some(variable_type),
        //                     is_local,
        //                 })
        //             }
        //             _ => return Err(ParsingError::unexpected(self, "'='".to_owned())),
        //         };

        //         Some(variable_type)
        //     }
        //     PklToken::EqualSign => None,
        //     _ => Err(ParsingError::unexpected(self, "'=', ':' or '{'".to_owned()))?,
        // };

        // let (value, next_token) = parse_opt_newlines(self, &parse_expr)?;

        // match next_token {
        //     Some(PklToken::NewLine) | Some(PklToken::DocComment) | Some(PklToken::LineComment) => {
        //         self.new_line_parsed = true;
        //     }
        //     None => (),
        //     _ => return Err(ParsingError::unexpected(self, "line end".to_owned())),
        // }

        // Ok(Statement::VariableDeclaration {
        //     name,
        //     value,
        //     optional_type,
        //     is_local,
        // })
    }
    fn _parse_typealias(&mut self) -> ParsingResult<Statement<'source>> {
        // let token = retrieve_next_token(self)?;
        todo!()
        // let (alias, generics_params) = match token {
        //     PklToken::Identifier(v) => (v, None),
        //     PklToken::GenericTypeAnnotationStart(name) => {
        //         let types = list_while_not_token2(
        //             self,
        //             PklToken::Comma,
        //             PklToken::RightAngleBracket(">"),
        //             &parse_type,
        //         )?;

        //         (name, Some(types))
        //     }
        //     _ => return Err(ParsingError::expected_identifier(self)),
        // };

        // expect_token(self, PklToken::EqualSign)?;

        // let (equivalent_type, next_token) = parse_opt_newlines(self, &parse_type)?;

        // match next_token {
        //     Some(PklToken::NewLine) | Some(PklToken::LineComment) | Some(PklToken::DocComment) => {
        //         self.new_line_parsed = true;
        //     }
        //     _ => return Err(ParsingError::unexpected(self, "line ending".to_owned())),
        // };

        // Ok(Statement::TypeAlias {
        //     alias,
        //     equivalent_type,
        //     generics_params,
        // })
    }
}
