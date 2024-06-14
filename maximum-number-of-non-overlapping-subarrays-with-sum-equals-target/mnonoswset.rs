use std::collections::HashSet;

pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut memo = vec![0; n + 1];
    for i in 0..n {
        memo[i + 1] = memo[i] + nums[i];
    }
    dbg!(&memo);

    let mut count = 0;
    let mut set = HashSet::new();
    for i in (0..=n).rev() {
        let v = memo[i];
        let need = v + target;
        dbg!(&need);
        if set.contains(&need) {
            count += 1;
            set = HashSet::new();
        }
        set.insert(v);
    }
    count
}

fn main() {
    assert_eq!(2, max_non_overlapping(vec![1, 1, 1, 1, 1], 2));
    assert_eq!(2, max_non_overlapping(vec![-1, 3, 5, 1, 4, 2, -9], 6));
}
