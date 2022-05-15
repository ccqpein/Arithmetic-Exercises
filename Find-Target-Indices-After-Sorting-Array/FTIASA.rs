pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut start = 0;
    let mut count = 0;
    for n in nums {
        if n < target {
            start += 1;
        }
        if n == target {
            count += 1;
        }
    }
    if count != 0 {
        (start..start + count).collect()
    } else {
        vec![]
    }
}

fn main() {
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
}
