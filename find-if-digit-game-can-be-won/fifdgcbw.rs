pub fn can_alice_win(nums: Vec<i32>) -> bool {
    nums.iter().filter(|n| **n < 10).sum::<i32>() != nums.iter().filter(|n| **n >= 10).sum::<i32>()
}

fn main() {
    assert!(!can_alice_win(vec![1, 2, 3, 4, 10]));
}
