pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut fs = vec![];
    let mut known_primes = seed_known_primes();
    while n > 1 {
        let f = find_prime_factor(n, &mut known_primes);
        fs.push(f);
        n /= f;
    }
    fs
}

fn divides(n: u64, k: u64) -> bool {
    k % n == 0
}

fn isqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}

fn seed_known_primes() -> Vec<u64> {
    vec![2, 3]
}

// known_primes may be updated
fn find_prime_factor(n: u64, known_primes: &mut Vec<u64>) -> u64 {
    let max_factor = isqrt(n);
    for p in known_primes.iter() {
        if divides(*p, n) {
            return *p;
        }
        if *p > max_factor {
            return n;
        }
    }
    let mut pp = *known_primes.last().unwrap();
    while pp < max_factor {
        pp = next_prime(pp, known_primes);
        known_primes.push(pp);
        if divides(pp, n) {
            return pp;
        }
    }
    n
}

// last_prime must be an odd prime
fn next_prime(last_prime: u64, known_primes: &[u64]) -> u64 {
    let mut n = last_prime;
    loop {
        n += 2;
        if is_prime(n, &known_primes) {
            return n;
        }
    }
}

// the caller must ensure that there are enough known primes!
fn is_prime(a: u64, known_primes: &[u64]) -> bool {
    if a == 0 || a == 1 {
        return false;
    }
    let max_known_prime = *known_primes.last().expect("no primes known");
    let max_divisor = isqrt(a);
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
