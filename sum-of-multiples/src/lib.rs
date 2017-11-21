#![feature(iter_arith)]
use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();

    for f in factors {
        let mut i = *f;
        while i < limit {
            multiples.insert(i);
            i += *f;
        }
    }
    multiples.iter().sum()
}
