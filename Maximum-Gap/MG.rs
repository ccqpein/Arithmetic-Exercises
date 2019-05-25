pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    if length < 2 as usize {
        return 0;
    }

    let mut new_nums = nums.clone();
    new_nums.sort();
    let mut largest = 0;
    for i in 1..length {
        if new_nums[i] - new_nums[i - 1] > largest {
            largest = new_nums[i] - new_nums[i - 1];
        }
    }

    largest
}

fn main() {
    println!("{:?}", maximum_gap(vec![3, 6, 9, 1])); //=> 3
    println!("{:?}", maximum_gap(vec![10])); //=> 0
}
