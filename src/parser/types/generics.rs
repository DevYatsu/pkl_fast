pub fn extract_generics<'a>(raw_string: &'a str) -> (&'a str, std::str::Split<'a, char>) {
    let generic_start_index = raw_string.find('<').unwrap();
    let base_type = &raw_string[0..generic_start_index];

    let generics = raw_string[generic_start_index + 1..raw_string.len() - 1].split(',');

    (base_type, generics)
}
