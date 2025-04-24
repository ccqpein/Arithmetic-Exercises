pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
    let mut hook = nums[0];
    let mut num = 1;
    for a in 1..nums.len() {
        if nums[a] < hook {
        } else {
            num += 1;
            hook = nums[a];
        }
    }
    num
}

fn main() {
    dbg!(maximum_possible_size(vec![4, 2, 5, 3, 5]));
    dbg!(maximum_possible_size(vec![1, 2, 3]));
}
