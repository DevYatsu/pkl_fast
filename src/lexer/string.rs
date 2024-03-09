use regex::Regex;

/// Logos does not support look-ahead, thus limiting our possibilities.
/// I came up with an alternative solution, we just extract the content of
/// our strings, store it in a vec, and replace it in the code by the index
/// of the string in the order they appear in the code.
pub fn sanitize_code(code: &str) -> (String, Vec<&str>) {
    let mut string_contents = Vec::with_capacity(10);
    let re = Regex::new(r##"(#+"(?:\\"|[^"])*"#+|"""(?:\\"|[^"])*"""|"(?:\\"|[^"])*")"##).unwrap();

    for (_, caps) in re.captures_iter(code).enumerate() {
        let matched_string = caps.get(0).unwrap().as_str();
        string_contents.push(matched_string)
    }

    // Print the vector containing string contents
    println!("String Contents:");
    let mut modified_code = String::from(code);
    for (index, content) in string_contents.iter().enumerate() {
        modified_code = modified_code.replacen(content, &format!("\"{index}\""), 1);
    }

    (modified_code, string_contents)
}
