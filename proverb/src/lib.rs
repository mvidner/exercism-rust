pub fn build_proverb(list: Vec<&str>) -> String {
    let mut lines: Vec<String> = vec![];
    let mut i = list.iter().rev().peekable();
    loop {
        match i.next() {
            Some(a) => {
                match i.peek() {
                    Some(b) => { lines.push(want_x_lose_y(b, a)); },
                    None    => { lines.reverse();
                                 lines.push(want(a)); },
                }
            }
            None => break,
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
