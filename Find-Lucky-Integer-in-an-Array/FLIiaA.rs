pub fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut cache = [0; 501];
    for n in arr {
        cache[n as usize] += 1;
    }
    for i in (1..=500).rev() {
        if cache[i] == i {
            return i as i32;
        }
    }

    return -1;
}

fn main() {
    assert_eq!(2, find_lucky(vec![2, 2, 3, 4]));
    assert_eq!(3, find_lucky(vec![1, 2, 2, 3, 3, 3]));
    assert_eq!(-1, find_lucky(vec![2, 2, 2, 3, 3]));
    assert_eq!(-1, find_lucky(vec![5]));
    assert_eq!(7, find_lucky(vec![7, 7, 7, 7, 7, 7, 7]));
}
