pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter().position(|&x| x == target).map(|v| v as i32).unwrap_or(-1)
}
