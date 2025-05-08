pub fn sanitize_filename(title: &str) -> String {
    title.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect()
}