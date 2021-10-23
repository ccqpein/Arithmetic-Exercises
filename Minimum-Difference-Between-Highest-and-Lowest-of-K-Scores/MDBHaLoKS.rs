pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    //dbg!(&nums);
    let mut result = i32::MAX;
    for i in 0..(nums.len() - k as usize + 1) {
        let a = nums[i + k as usize - 1] - nums[i];
        if a <= result {
            result = a
        }
    }
    if result == i32::MAX {
        0
    } else {
        result
    }
}

fn main() {
    for (case, k, result) in vec![(vec![9, 4, 1, 7], 2, 2)] {
        assert_eq!(minimum_difference(case, k), result);
    }
}
