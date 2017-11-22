type Triangle = (u32, u32, u32);

pub fn find() -> Option<u32> {
    find_n(1000).map(|(a, b, c)| a * b * c)
}

fn find_n(n: u32) -> Option<Triangle> {
    let ts = triangles_with_circumference(n);
    let mut found = ts.into_iter().filter(|t| is_pythagorean_triplet(*t));
    found.next()
}

fn is_pythagorean_triplet(t: Triangle) -> bool {
    let (a, b, c) = t;
    a * a + b * b == c *c
}

fn triangles_with_circumference(n: u32) -> Vec<Triangle> {
    let mut ts = vec![];
    // a < b < c
    // c: at most n, at least n/3 (otherwise the shorter ones cannot make up to n)
    for c in n / 3 .. n + 1 {
        // b: 1 < b < c
        for b in 2 .. c {
            let a: i32 = n as i32 - c as i32 - b as i32;
            if a > 0 && a < b as i32 {
                ts.push((a as u32, b, c));
            }
        }
    }
    ts
}
