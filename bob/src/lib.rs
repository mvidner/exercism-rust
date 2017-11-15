pub fn reply(message: &str) -> &str {
    let trimmed = message.trim(); // wihtout trailing (and leading) whitespace

    // When we have a yelling question, it is considered yelling. Check it 1st.
    if is_yelling(trimmed) {
        "Whoa, chill out!"
    }
    else if is_question(trimmed) {
        "Sure."
    }
    else if trimmed.is_empty() {
        "Fine. Be that way!"
    }
    else {
        "Whatever."
    }
}

// It ends with a question mark '?', after stripping whitespace
fn is_question(s: &str) -> bool {
    s.chars().last() == Some('?')
}

// Yelling is independent of an exclamation mark.
// It means that there are some letters and all are in uppercase.
fn is_yelling(s: &str) -> bool {
    // A peekable iterator 
    let mut letters = s.chars().filter(|c| c.is_alphabetic()).peekable();
    let not_empty = letters.peek().is_some();
    not_empty && letters.all(|c| c.is_uppercase())
}
