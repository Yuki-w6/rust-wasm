use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        sc: [(u32, u64); n]
    }

    let mut counts: BTreeMap<u64, u64> = BTreeMap::new();
    
    for (s, c) in sc {
        *counts.entry(s).or_insert(0) += c;
    }

    let mut keys_to_check: Vec<u64> = counts.keys().cloned().collect();
    while let Some(key) = keys_to_check.pop() {
        let count = counts[&key];
        if count >= 2 {
            let next_key = key * 2;
            *counts.entry(next_key).or_insert(0) += count / 2;
            if !keys_to_check.contains(&next_key) {
                keys_to_check.push(next_key);
            }
        }
        counts.insert(key, count % 2);
    }

    let result: u64 = counts.values().sum();
    println!("{}", result);
}
