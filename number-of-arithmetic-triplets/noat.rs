pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    use std::collections::{HashMap, HashSet};
    let mut values = HashMap::new();

    nums.iter()
        .enumerate()
        .for_each(|(ind, v)| values.entry(v).or_insert(vec![]).push(ind));

    let mut res = 0;
    for (_, n) in nums.iter().enumerate() {
        match (values.get(&(*n + diff)), values.get(&(*n + diff + diff))) {
            (Some(b), Some(c)) => res += (b.len() * c.len()),
            _ => {}
        }
    }

    res as i32
}

fn main() {
    assert_eq!(arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
    assert_eq!(arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
}
