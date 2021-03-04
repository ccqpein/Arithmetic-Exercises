use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut table = HashMap::new();
    let bound = nums.len() / 3;
    let mut result = Vec::with_capacity(2);
    for n in nums {
        let en = table.entry(n).or_insert(0);
        *en += 1;
        if *en > bound && !result.contains(&n) {
            result.push(n);
        }
        if result.len() == 2 {
            return result;
        }
    }
    result
}

fn main() {
    assert_eq!(majority_element(vec![3, 2, 3]), vec![3]);
    assert_eq!(majority_element(vec![1, 2]), vec![1, 2]);
    assert_eq!(majority_element(vec![1]), vec![1]);
}
