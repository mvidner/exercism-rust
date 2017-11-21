pub fn build_proverb(list: Vec<&str>) -> String {
    let mut lines: Vec<String> = vec![]; // result
    let mut i = list.iter().peekable();
    let mut op_first: Option<&str> = None; // the originally missing thing
    loop {
        match i.next() {
            None => break,
            Some(a) => {
                let first: &str = match op_first {
                    Some(f) => f,
                    None    => { op_first = Some(a); a },
                };

                let line = match i.peek() {
                    Some(b) => want_x_lose_y(a, b),
                    None    => want(first),
                };
                lines.push(line);
            }
        }
    }

    lines.join("\n")
}

fn want_x_lose_y(x: &str, y: &str) -> String {
    format!("For want of a {} the {} was lost.", x, y)
}

fn want(x: &str) -> String {
    format!("And all for the want of a {}.", x)
}
