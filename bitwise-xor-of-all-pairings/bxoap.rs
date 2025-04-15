pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let len1 = nums1.len();
    let len2 = nums2.len();
    nums1
        .into_iter()
        .map(|v| xor_times(v, len2))
        .reduce(|acc, e| acc ^ e)
        .unwrap()
        ^ nums2
            .into_iter()
            .map(|v| xor_times(v, len1))
            .reduce(|acc, e| acc ^ e)
            .unwrap()
}

fn xor_times(v: i32, time: usize) -> i32 {
    if time % 2 == 0 {
        0
    } else {
        v
    }
}

fn main() {
    dbg!(xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]));
}
