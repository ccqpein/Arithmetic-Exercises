use std::collections::{HashMap, HashSet};

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let s = nums.into_iter().fold(HashMap::new(), |mut s, num| {
        *s.entry(num).or_insert(0) += 1;

        s
    });

    let mut result = 0;
    for (_, num) in s {
        if num == 1 {
            return -1;
        }
        result += helper(num);
    }
    return result;
}

fn helper(num: i32) -> i32 {
    match num % 3 {
        0 => num / 3,
        2 | 1 => num / 3 + 1,
        _ => unreachable!(),
    }
}

fn main() {
    assert_eq!(4, min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]));
    assert_eq!(-1, min_operations(vec![2, 1, 2, 2, 3, 3]));
}
