pub fn maximum_median_sum(mut nums: Vec<i32>) -> i64 {
    nums.sort();

    let times = nums.len() / 3;
    let mut ind = nums.len() - 2;
    let mut sum = 0_i64;
    for _ in 0..times {
        sum += nums[ind] as i64;
        ind -= 2
    }
    sum
}

fn main() {
    dbg!(maximum_median_sum(vec![2, 1, 3, 2, 1, 3]));
    dbg!(maximum_median_sum(vec![1, 1, 10, 10, 10, 10]));
}
