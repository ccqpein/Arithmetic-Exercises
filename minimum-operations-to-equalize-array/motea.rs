pub fn min_operations(nums: Vec<i32>) -> i32 {
    let x = nums[0];
    if nums.into_iter().all(|v| v == x) {
        0
    } else {
        1
    }
}

fn main() {}
