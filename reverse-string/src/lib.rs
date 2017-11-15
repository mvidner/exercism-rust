pub fn reverse(s: &str) -> String {
    let mut result = String::new();
    result.reserve(s.len());

    let mut cs: Vec<char> = s.chars().collect();
    cs.reverse();
    for c in cs {
        result.push(c);
    }
    result
}
