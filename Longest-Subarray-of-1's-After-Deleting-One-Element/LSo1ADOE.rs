pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let whole_len = nums.len() as i32;
    let nums = nums.split(|num| *num == 0);
    let mut result = 0;
    let mut cache = 0;
    for dd in nums {
        //dbg!(dd);
        if dd.is_empty() {
            cache = 0;
        } else {
            let ddl = dd.len() as i32;
            if ddl + cache >= result {
                result = ddl + cache;
            }
            cache = ddl;
        }
    }

    if whole_len == result {
        return result - 1;
    }

    result
}

fn main() {
    assert_eq!(longest_subarray(vec![1, 1, 0, 1]), 3);
    assert_eq!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
    assert_eq!(longest_subarray(vec![1, 1, 1]), 2);
    assert_eq!(longest_subarray(vec![1, 1, 0, 0, 1, 1, 1, 0, 1]), 4);
    assert_eq!(longest_subarray(vec![0, 0, 0]), 0);

    assert_eq!(
        longest_subarray(vec![
            0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1
        ]),
        19
    )
}
