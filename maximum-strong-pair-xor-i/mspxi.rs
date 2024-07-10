pub fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut max = 0;
    dbg!(&nums);
    for i in 0..(nums.len() - 1) {
        for j in i + 1..nums.len() {
            if (nums[j] - nums[i]) <= nums[i] {
                let xx = nums[j] ^ nums[i];
                if xx > max {
                    max = xx
                }
            } else {
                break;
            }
        }
    }
    max
}

fn main() {
    assert_eq!(maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
    assert_eq!(maximum_strong_pair_xor(vec![10, 100]), 0);
    assert_eq!(maximum_strong_pair_xor(vec![5, 6, 25, 30]), 7);
    assert_eq!(maximum_strong_pair_xor(vec![1, 5, 4, 1, 7]), 3);
    assert_eq!(maximum_strong_pair_xor(vec![5, 1, 5, 4, 7]), 3);
}
