pub fn verse(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
              Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_string(),
        _ => {
            format!("{0} of beer on the wall, {0} of beer.\n\
                      Take one down and pass it around, {1} of beer on the wall.\n",
                    n_bottles(n),
                    n_bottles(n - 1))
        }
    }
}

pub fn sing(start: i32, end: i32) -> String {
    if start < end {
        return "".to_string();
    }

    // a procedural solution
    let mut result = String::new();
    let mut i = start;
    while i > end {
        result.push_str(&verse(i));
        result.push('\n');
        i -= 1;
    }
    result.push_str(&verse(i));
    result
}

fn n_bottles(n: i32) -> String {
    match n {
        // 0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        x => format!("{} bottles", x)
    }
}
