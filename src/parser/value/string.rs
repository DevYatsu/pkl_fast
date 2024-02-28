#[derive(Debug, PartialEq, Clone)]
pub enum StringFragment<'source> {
    Escaped(char),
    Interpolated(&'source str),
    RawText(&'source str),
}

impl<'source> StringFragment<'source> {
    pub fn from_raw_string(raw_string: &'source str) -> Result<Vec<Self>, LexingError> {
        let mut fragments = Vec::new();
        let mut start_index = 0;
        let mut end_index = 0;

        while end_index < raw_string.len() {
            match raw_string.chars().nth(end_index) {
                Some('\\') => {
                    // Push raw text before the escaped character
                    if end_index > start_index {
                        fragments
                            .push(StringFragment::RawText(&raw_string[start_index..end_index]));
                    }
                    // Advance to the escaped character
                    start_index = end_index;

                    // Check if there's a character after the backslash
                    match raw_string.chars().nth(end_index + 1) {
                        Some('(') => {
                            // Push raw text before the interpolated string
                            if end_index > start_index {
                                fragments.push(StringFragment::RawText(
                                    &raw_string[start_index..end_index],
                                ));
                            }
                            // Advance to the opening parenthesis
                            start_index = end_index;
                            // Find the closing parenthesis
                            let mut closing_index = end_index + 1;
                            while let Some(c) = raw_string.chars().nth(closing_index) {
                                if c == ')' {
                                    // Push the interpolated string
                                    fragments.push(StringFragment::Interpolated(
                                        &raw_string[start_index + 1..closing_index],
                                    ));
                                    // Advance past the closing parenthesis
                                    end_index = closing_index + 1;
                                    // Update the start index for the next iteration
                                    start_index = end_index;
                                    break;
                                }
                                closing_index += 1;
                            }
                        }
                        Some(escaped_char) => {
                            // Push the escaped character
                            fragments.push(StringFragment::Escaped(escaped_char));
                            // Advance past the escaped character
                            end_index += 2;
                            // Update the start index for the next iteration
                            start_index = end_index;
                        }
                        // if no character
                        None => return Err(LexingError::UnterminatedString),
                    }
                }
                Some(_) => {
                    // Advance to the next character
                    end_index += 1;
                }
                None => {
                    // Break if reached the end of the string
                    break;
                }
            }
        }

        // Push the remaining raw text
        if end_index > start_index {
            fragments.push(StringFragment::RawText(&raw_string[start_index..]));
        }

        Ok(fragments)
    }
}