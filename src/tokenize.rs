pub fn tokenize(line: &str) -> (&str, Vec<&str>) {
    let trimmed_line: &str = line.trim();
    let tokens: Vec<&str> = trimmed_line.split(' ').collect();

    return (trimmed_line, tokens);
}
