pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut neg = 0;

    for (ind, i) in nums.iter().enumerate() {
        if *i > 0 {
            return neg.max((nums.len() - ind) as i32);
        } else if *i < 0 {
            neg += 1;
        }
    }

    return neg;
}

fn main() {
    assert_eq!(3, maximum_count(vec![-2, -1, -1, 1, 2, 3]));
    assert_eq!(3, maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
    assert_eq!(4, maximum_count(vec![5, 20, 66, 1314]));
}
