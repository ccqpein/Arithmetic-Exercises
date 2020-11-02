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

pub fn find_lucky2(arr: Vec<i32>) -> i32 {
    let mut cache = [0; 501];
    arr.iter().fold(&mut cache, |cache, &n| {
        cache[n as usize] += 1;
        cache
    });
    (1..=500)
        .rev()
        .find(|&x| cache[x as usize] == x)
        .unwrap_or(-1)
}

fn main() {
    assert_eq!(2, find_lucky2(vec![2, 2, 3, 4]));
    assert_eq!(3, find_lucky2(vec![1, 2, 2, 3, 3, 3]));
    assert_eq!(-1, find_lucky2(vec![2, 2, 2, 3, 3]));
    assert_eq!(-1, find_lucky2(vec![5]));
    assert_eq!(7, find_lucky2(vec![7, 7, 7, 7, 7, 7, 7]));
}
