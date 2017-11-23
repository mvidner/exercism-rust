// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("The Collatz formula does not work on 0");
    }

    let mut count = 0;
    let mut i = n;
    while i > 1 {
        i = step(i);
        count +=1;
    }
    Ok(count)
}

fn step(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}
