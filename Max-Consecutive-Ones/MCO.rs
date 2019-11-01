pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    nums.split(|x| *x == 0)
        .map(|x| x.len() as i32)
        .max()
        .unwrap()
}

fn main() {
    println!("{:?}", find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]));
}
