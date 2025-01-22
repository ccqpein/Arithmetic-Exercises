pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    //let mut set = HashSet::new();
    let aa = nums.iter().fold(0, |a, b| a ^ b);
    let set = nums.into_iter().collect::<HashSet<i32>>();

    set.into_iter().fold(0, |a, b| a ^ b) ^ aa
}

fn main() {
    assert_eq!(duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
    assert_eq!(duplicate_numbers_xor(vec![1, 2, 3]), 0);
    assert_eq!(duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
}
