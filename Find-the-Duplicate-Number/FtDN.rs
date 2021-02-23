pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut cache = vec![false; nums.len()];
    for n in nums {
        if !cache[n as usize] {
            cache[n as usize] = true
        } else {
            return n;
        }
    }
    0
}

fn main() {
    assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    assert_eq!(find_duplicate(vec![1, 1]), 1);
    assert_eq!(find_duplicate(vec![1, 1, 2]), 1);
}
