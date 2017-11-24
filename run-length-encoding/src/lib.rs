pub fn encode(s: &str) -> String {
    if s.is_empty() {
        return s.to_string();
    }
    let mut result: Vec<u8> = vec![];

    let bs = s.as_bytes();
    let mut last: u8 = bs[0]; // not empty
    let mut run_length: usize = 1;

    for b in bs[1..].iter() {
        if last == *b {
            run_length += 1;
        }
        else {
            emit(&mut result, last, run_length);
            last = *b;
            run_length = 1;
        }
    }
    emit(&mut result, last, run_length);

    String::from_utf8(result).unwrap()
}

fn emit(buffer: &mut Vec<u8>, sym: u8, count: usize) {
    if count > 1 {
        let cs = count.to_string();
        let mut cv = Vec::from(cs.as_bytes());
        buffer.append(&mut cv);
    }
    if count > 0 {
        buffer.push(sym);
    }
}

pub fn decode(s: &str) -> String {
    let mut result = String::new();
    let mut buf = s;
    loop {
        let (count, rest) = leading_digits(buf);
        match count {
            None => break,
            Some(n) => {
                let c: char = rest.chars().next().unwrap();
                result.reserve(n);
                for _ in 0..n {
                    result.push(c);
                }
                buf = &rest[1..];
            }
        }
    }
    result
}

fn leading_digits(s: &str) -> (/*digits:*/ Option<usize>, /*rest:*/ &str) {
    let first = s.chars().next();
    match first {
        None => { return (None, ""); },
        Some(c) => {
            if !c.is_digit(10) {
                return (Some(1), s);
            }
        }
    }
    let first_nondigit = s.find(|c: char| !c.is_digit(10));
    match first_nondigit {
        None    => panic!("length without substance"),
        Some(i) => {
            let (digits, rest) = (&s[..i], &s[i..]);
            (Some(usize::from_str_radix(digits, 10).unwrap()), rest)
        }
    }
}
