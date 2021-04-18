pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    let mut record: HashMap<i32, HashSet<i32>> = HashMap::new();
    record.insert(-1, HashSet::new());
    nums.sort();
    for n in nums {
        let mut new_s = HashSet::new();
        new_s.insert(n);
        record.insert(
            n,
            new_s
                .union(
                    record
                        .iter()
                        .filter(|(k, _)| n % **k == 0)
                        .map(|(_, v)| (v.len(), v))
                        .max_by_key(|v| v.0)
                        .unwrap()
                        .1,
                )
                .cloned()
                .collect(),
        );
    }
    let mut result = record
        .iter()
        .max_by_key(|(k, v)| v.len())
        .unwrap()
        .1
        .iter()
        .cloned()
        .collect::<Vec<i32>>();

    result.sort();
    result
}

fn main() {
    dbg!(largest_divisible_subset(vec![1, 2, 3]));
    dbg!(largest_divisible_subset(vec![1, 2, 4, 8]));
}
