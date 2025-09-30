use std::collections::VecDeque;

pub fn triangular_sum(nums: Vec<i32>) -> i32 {
    help(&mut nums.into_iter().collect::<VecDeque<_>>())
}

fn help(nums: &mut VecDeque<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    for i in 0..nums.len() - 1 {
        nums[i] = (nums[i] + nums[i + 1]) % 10;
    }

    nums.pop_back();

    help(nums)
}

fn main() {
    dbg!(triangular_sum(vec![1, 2, 3, 4, 5]));
}
