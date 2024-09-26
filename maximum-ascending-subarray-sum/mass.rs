pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut nums = nums.into_iter();
    let mut init = nums.next().unwrap();
    let mut max = init;
    let mut bucket = vec![];

    for a in nums {
        if a <= init {
            bucket.push(max);
            max = a;
        } else {
            max += a
        }
        init = a;
    }
    bucket.push(max);

    bucket.into_iter().max().unwrap()
}

fn main() {
    assert_eq!(max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    assert_eq!(max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
    assert_eq!(max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]), 33);
}
