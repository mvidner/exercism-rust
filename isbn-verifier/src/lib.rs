/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let rv: Result<Vec<u8>, String> = parse_isbn(isbn);
    let r:  Result<(),      String> = rv.and_then(check_valid_isbn_num);

    match r {
        Ok(()) => true,
        Err(s) => {
            println!("{}: {}", isbn, s);
            false
        }
    }
}

fn parse_isbn(isbn: &str) -> Result<Vec<u8>, String> {
    let rvec: Vec<Result<u8, String>> = isbn
        .chars()
        .filter(|&c| c != '-')
        .map(|c| parse_isbn_digit(c))
        .enumerate()
        .map(|(i, r)| match r {
            Ok(10) => {
                if i != 9 {
                    Err(format!("X found at position {}", i))
                } else {
                    Ok(10)
                }
            },
            Ok(n)  => Ok(n),
            Err(e) => Err(e),
        })
        .collect::<Vec<_>>();
    // Collection<Result> -> Result<Collection>
    let result = rvec.iter().cloned().collect();
    result
}

fn parse_isbn_digit(c: char) -> Result<u8, String> {
    if c == 'X' {
        Ok(10)
    } else if c >= '0' && c <= '9' {
        Ok((c as u8) - ('0' as u8))
    } else {
        Err(format!("'{}' is not an ISBN digit", c))
    }
}

fn check_valid_isbn_num(x: Vec<u8>) -> Result<(), String> {
    if x.len() != 10 {
        return Err("Length (without dashes) is not 10".to_string());
    }
    let sum =
        x[0] as u32 * 10 +
        x[1] as u32 * 9 +
        x[2] as u32 * 8 +
        x[3] as u32 * 7 +
        x[4] as u32 * 6 +
        x[5] as u32 * 5 +
        x[6] as u32 * 4 +
        x[7] as u32 * 3 +
        x[8] as u32 * 2 +
        x[9] as u32;
    if sum % 11 == 0 {
        Ok(())
    } else {
        Err("Checksum does not match".to_string())
    }
}
