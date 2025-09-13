pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let init = nums[0] - nums.get(1..).unwrap().into_iter().map(|n| *n).sum::<i32>();
    if init % 2 == 0 {
        return (nums.len() - 1) as i32;
    } else {
        return 0;
    }
}

fn main() {
    dbg!(count_partitions(vec![10, 10, 3, 7, 6]));
    dbg!(count_partitions(vec![1, 2, 2]));
    dbg!(count_partitions(vec![2, 4, 6, 8]));
    dbg!(count_partitions(vec![1, 5, 5]));
}
