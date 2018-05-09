fn sum_vec(nums: &Vec<i32>) -> i32 {
    let mut result = 0;
    for i in nums {
        result += *i;
    }
    result
}

fn check_sub_array_sum(nums: Vec<i32>, k: i32) -> bool {
    let length = nums.len();
    if length <= 1 {
        return false;
    }

    if sum_vec(&nums) == 0 {
        return true;
    }

    if k == 0 {
        return false;
    }

    if length >= 2 && k > 0 {
        return true;
    }
}

fn main() {}
