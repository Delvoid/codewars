use std::collections::BTreeSet;

pub fn longest(a1: &str, a2: &str) -> String {
    let set1: BTreeSet<char> = a1.chars().collect();
    let set2: BTreeSet<char> = a2.chars().collect();
    set1.union(&set2).collect::<String>()
}

// BTreeSet from the standard lib is used to store the distinct letters in each string.
// BTreeSet is a set data structure that automatically sorts its elements.

pub fn longest_2(a1: &str, a2: &str) -> String {
    let mut res: Vec<_> = a1.chars().collect();
    res.extend(a2.chars());
    res.sort();
    res.dedup();
    res.into_iter().collect()
}

pub fn longest_3(a1: &str, a2: &str) -> String {
    a1.chars()
        .chain(a2.chars())
        .collect::<BTreeSet<char>>()
        .iter()
        .collect()
}
