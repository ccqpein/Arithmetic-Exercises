pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    helper(&nums, 4, 0) as i32
}

fn helper(nums: &[i32], left: usize, already: i32) -> usize {
    if nums.len() < left {
        return 0;
    }

    if left == 1 {
        return nums.iter().filter(|b| **b == already).count();
    }

    let mut re = 0;
    for i in 0..nums.len() {
        re += helper(&nums[i + 1..], left - 1, already + nums[i])
    }
    re
}

fn main() {
    assert_eq!(count_quadruplets(vec![1, 2, 3, 6]), 1);
    assert_eq!(count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    assert_eq!(count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
}
