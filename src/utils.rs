/// Parses an identifier from the input stream.
///
/// This macro generates code to parse an identifier token from the given lexer.
/// It can be called with just a lexer for default error messages, or with a lexer
/// and custom error messages.
///
/// # Arguments
///
/// * `$lexer` - An expression that evaluates to a mutable reference to a Lexer.
/// * `$default_unexpected` - (Optional) A custom error message for unexpected tokens.
/// * `$eof_error` - (Optional) A custom error message for unexpected end of file.
///
/// # Returns
///
/// Returns a `Result` containing either:
/// * `Ok((&str, Range<usize>))` - A tuple with the identifier string and its span.
/// * `Err((String, Range<usize>))` - A tuple with an error message and the error span.
///
/// # Examples
///
/// ```
/// // Using default error messages
/// let result = parse_identifier!(lexer);
///
/// // Using custom error messages
/// let result = parse_identifier!(
///     lexer,
///     "Custom unexpected token error",
///     "Custom end of file error"
/// );
/// ```
#[macro_export]
macro_rules! parse_identifier {
    // Pattern 1: Just the lexer
    ($lexer:expr) => {
        parse_identifier!(
            $lexer,
            "unexpected token here, expected an identifier",
            "Expected identifier"
        )
    };
    ($lexer:expr, $default_unexpected:expr) => {
        parse_identifier!($lexer, $default_unexpected, "Expected identifier")
    };
    // Pattern 2: Lexer with custom error messages
    ($lexer:expr, $default_unexpected:expr, $eof_error:expr) => {{
        use crate::lexer::PklToken;
        use crate::parser::Identifier;
        let start = $lexer.span().start;
        while let Some(token) = $lexer.next() {
            match token {
                Ok(PklToken::Identifier(id)) | Ok(PklToken::IllegalIdentifier(id)) => {
                    return Ok(Identifier(id, start..$lexer.span().end))
                }
                Ok(PklToken::NewLine) | Ok(PklToken::Space) => {
                    // Skip spaces and newlines
                }
                Err(e) => {
                    return Err((e.to_string(), $lexer.span()));
                }
                _ => {
                    return Err(($default_unexpected.to_owned(), $lexer.span()));
                }
            }
        }
        Err(($eof_error.to_owned(), $lexer.span()))
    }};
}

/// Parses a string from the input stream.
///
/// This macro generates code to parse a string token from the given lexer.
/// It can be called with just a lexer for default error messages, or with a lexer
/// and custom error messages.
///
/// # Arguments
///
/// * `$lexer` - An expression that evaluates to a mutable reference to a Lexer.
/// * `$default_unexpected` - (Optional) A custom error message for unexpected tokens.
/// * `$eof_error` - (Optional) A custom error message for unexpected end of file.
///
/// # Returns
///
/// Returns a `PklResult` containing either:
/// * `Ok((&str, Range<usize>))` - A tuple with the string content and its span.
/// * `Err((String, Range<usize>))` - A tuple with an error message and the error span.
///
/// # Examples
///
/// ```
/// // Using default error messages
/// let result = parse_string!(lexer);
///
/// // Using custom error messages
/// let result = parse_string!(
///     lexer,
///     "Custom unexpected token error",
///     "Custom end of file error"
/// );
/// ```
#[macro_export]
macro_rules! parse_string {
    // Pattern 1: Just the lexer
    ($lexer:expr) => {
        parse_string!(
            $lexer,
            "unexpected token here, expected a string",
            "Expected string"
        )
    };
    ($lexer:expr, $default_unexpected:expr) => {
        parse_identifier!($lexer, $default_unexpected, "Expected string")
    };
    // Pattern 2: Lexer with custom error messages
    ($lexer:expr, $default_unexpected:expr, $eof_error:expr) => {{
        let start = $lexer.span().start;
        while let Some(token) = $lexer.next() {
            match token {
                Ok(PklToken::String(s)) => return Ok((s, start..$lexer.span().end)),
                Ok(PklToken::NewLine) | Ok(PklToken::Space) => {
                    // Skip spaces and newlines
                }
                Err(e) => {
                    return Err((e.to_string(), $lexer.span()));
                }
                _ => {
                    return Err(($default_unexpected.to_owned(), $lexer.span()));
                }
            }
        }
        Err(($eof_error.to_owned(), $lexer.span()))
    }};
}

// Helper macro to count arguments
#[macro_export]
macro_rules! count_args {
    ($($arg_index:tt),*) => {
        <[()]>::len(&[$(count_args!(@single $arg_index)),*])
    };
    (@single $arg_index:tt) => { () };
}

#[macro_export]
macro_rules! generate_method {
    ($name:expr,$args:expr; $($arg_index:tt : $arg_type:ident),+; $action:expr; $range:expr) => {{
        use crate::count_args;

        let name: &str = $name;
        let number_of_args: usize = count_args!($($arg_index),+);
        let args: &Vec<PklValue<'_>> = $args;

        if args.len() != number_of_args {
            return Err((
                format!(
                    "Method '{}' expects exactly {} argument(s)",
                    name, number_of_args
                ),
                $range,
            ));
        }

        $(
            if stringify!($arg_type) == "Number" {
                if args[$arg_index].get_type() != "Float" && args[$arg_index].get_type() != "Int" {
                    return Err((
                        format!(
                            "{} method expects argument at index {} to be of type Number, but found {}",
                            name, $arg_index, args[$arg_index].get_type()
                        ),
                        $range,
                    ));
                }
            } else if args[$arg_index].get_type() != stringify!($arg_type) {
                return Err((
                    format!(
                        "{} method expects argument at index {} to be of type {}, but found {}",
                        name, $arg_index, stringify!($arg_type), args[$arg_index].get_type()
                    ),
                    $range,
                ));
            }
        )+

        let args_tuple = (
            $(
                match &args[$arg_index] {
                    PklValue::$arg_type(value) => value.to_owned(),
                    _ => return Err((
                        format!(
                            "{} method expects argument at index {} to be of type {}, but found {}",
                            name, $arg_index, stringify!($arg_type), args[$arg_index].get_type()
                        ),
                        $range,
                    )),
                }
            ),+
        );

        $action(args_tuple)
    }};
    ($name:expr,$args:expr; $action:expr; $range:expr) => {{
        let name: &str = $name;
        let number_of_args: usize = 0;
        let args: &Vec<PklValue<'_>> = $args;

        if args.len() != number_of_args {
            return Err((
                format!(
                    "Method '{}' expects 0 argument",
                    name
                ),
                $range,
            ));
        }

        $action
    }};

    ($name:expr, $args:expr; Numbers: $args_number:expr; $action:expr; $range:expr) => {{
        // Case only useful when the method takes several Number arguments

        let name: &str = $name;
        let number_of_args: usize = $args_number;
        let args: &Vec<PklValue<'_>> = $args;
        if args.len() != number_of_args {
            return Err((
                format!(
                    "Method '{}' expects exactly {} argument(s)",
                    name, number_of_args
                ),
                $range,
            ));
        }



        let mut args_tuple: [f64; $args_number] = [0.0; $args_number];

        for arg_number in 0..=number_of_args {
            if args[arg_number].get_type() != "Float" && args[arg_number].get_type() != "Int" {
                return Err((
                    format!(
                        "{} method expects argument at index {} to be of type Number, but found {}",
                        name, arg_number, args[arg_number].get_type()
                    ),
                    $range,
                ));
            }

            args_tuple[arg_number] = match &args[arg_number] {
                PklValue::Float(value) => *value,
                PklValue::Int(value) => *value as f64,
                _ => return Err((
                    format!(
                        "{} method expects argument at index {} to be of type Number, but found {}",
                        name, arg_number, args[arg_number].get_type()
                    ),
                    $range,
                )),
            };
        }
        $action(args_tuple)
    }};

}
