pub fn raindrops(n: usize) -> String {
    let f3 = n % 3 == 0;
    let f5 = n % 5 == 0;
    let f7 = n % 7 == 0;

    if !f3 && !f5 && !f7 {
        n.to_string()
    }
    else {
        let mut result = String::new();
        if f3 { result += "Pling"; }
        if f5 { result += "Plang"; }
        if f7 { result += "Plong"; }
        result
    }
}
