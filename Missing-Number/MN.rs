pub fn missing_number(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut key = 0;
    for i in nums {
        if key == i {
            key += 1;
        } else {
            return key;
        }
    }
    key
}

fn main() {
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
    assert_eq!(missing_number(vec![0, 1]), 2);
    assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    assert_eq!(missing_number(vec![0]), 1);
    assert_eq!(missing_number(vec![1]), 0);
}
