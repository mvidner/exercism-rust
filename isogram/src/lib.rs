use std::collections::HashSet;

pub fn check(s: &str) -> bool {
    let letters_i = s.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_uppercase().collect::<String>());
    let mut once: HashSet<String> = HashSet::new();
    for l in letters_i {
        if once.contains(&l) {
            return false
        }
        once.insert(l);
    }
    true
}
