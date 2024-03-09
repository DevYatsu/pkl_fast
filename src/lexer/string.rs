use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum StringKind<'source> {
    Classic(&'source str),
    MultiLine(&'source str),
    Delimited {
        start_delimiter: &'source str,
        end_delimiter: &'source str,
        value: &'source str,
    },
}

impl<'source> From<&'source str> for StringKind<'source> {
    fn from(value: &'source str) -> Self {
        match value {
            value if value.starts_with("\"") => StringKind::Classic(&value[1..value.len() - 1]),
            value if value.starts_with("\"\"\"") => StringKind::Classic(&value[3..value.len() - 3]),
            value => {
                let start_delimiter = &value[..value.chars().take_while(|&c| c == '#').count()];
                let end_delimiter = &value[..value.chars().rev().take_while(|&c| c == '#').count()];
                let trimed_value = value
                    .trim_start_matches(start_delimiter)
                    .trim_end_matches(end_delimiter);

                StringKind::Delimited {
                    start_delimiter,
                    end_delimiter,
                    value: &trimed_value[1..trimed_value.len() - 1],
                }
            }
        }
    }
}

/// Logos does not support look-ahead, thus limiting our possibilities.
/// I came up with an alternative solution, we just extract the content of
/// our strings, store it in a vec, and replace it in the code by the index
/// of the string in the order they appear in the code.
pub fn sanitize_code(code: &str) -> (String, Vec<StringKind<'_>>) {
    let mut string_contents = Vec::with_capacity(10);
    let re = Regex::new(r##"(#+"(?:\\"|[^"])*"#+|"""(?:\\"|[^"])*"""|"(?:\\"|[^"])*")"##).unwrap();

    let mut modified_code = String::from(code);

    for (index, caps) in re.captures_iter(code).enumerate() {
        let matched_string = caps.get(0).unwrap().as_str();

        modified_code = modified_code.replacen(matched_string, &format!("\"{index}\""), 1);
        string_contents.push(StringKind::from(matched_string))
    }

    (modified_code, string_contents)
}
