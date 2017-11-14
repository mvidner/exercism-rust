// A simple and stupid way with trial division by all numbers.
// On my machine, the millionth prime takes 33s to find
// (optimized and unoptimized alike)
pub fn nth(n: usize) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("The 0th prime does not exist");
    }
    let mut count = 1;
    let mut candidate = 2;
    while count < n {
        loop {
            candidate += 1;
            if is_prime(candidate) {
                break;
            }
        }
        count += 1;
    }
    Ok(candidate)
}

fn is_prime(a: u64) -> bool {
    if a == 0 || a == 1 {
        return false;
    }
    let max_divisor = (a as f64).sqrt() as u64;
    let mut divisor = 2;
    while divisor <= max_divisor {
        if a % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}
