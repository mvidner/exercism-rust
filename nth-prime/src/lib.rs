// A slightly sophisticated version: trial division by known primes
// On my machine, the millionth prime takes 5s to find (optimized)
pub fn nth(n: usize) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("The 0th prime does not exist");
    }

    let mut known_primes: Vec<u64> = vec![2, 3];
    let mut candidate = *known_primes.last().unwrap();
    while n > known_primes.len() {
        loop {
            // since we start with an odd one we can skip the even ones
            candidate +=2;
            if is_prime(candidate, &known_primes) {
                known_primes.push(candidate);
                break;
            }
        }
    }
    Ok(known_primes[n - 1])
}

// the caller must ensure that there are enough known primes!
fn is_prime(a: u64, known_primes: &[u64]) -> bool {
    if a == 0 || a == 1 {
        return false;
    }
    let max_known_prime = *known_primes.last().expect("no primes known");
    let max_divisor = (a as f64).sqrt() as u64;
    if  max_divisor > max_known_prime {
        panic!("not enough known primes");
    }
    for divisor in known_primes {
        if a % divisor == 0 {
            return false;
        }
        if *divisor >= max_divisor {
            return true;
        }
    }
    true
}
