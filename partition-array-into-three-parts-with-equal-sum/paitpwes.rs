pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    let mut cache = 0;
    for (ind, v) in arr.iter().enumerate() {
        cache += v;
        if helper(cache, &arr[ind + 1..]) {
            return true;
        }
    }
    false
}

fn helper(target: i32, l: &[i32]) -> bool {
    let mut cache = 0;
    for (ind, d) in l.iter().enumerate() {
        cache += *d;
        if cache == target {
            if ind + 1 < l.len()
                && l.get(ind + 1..)
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|v| v)
                    .sum::<i32>()
                    == target
            {
                return true;
            }
        }
    }
    false
}

fn main() {
    assert!(can_three_parts_equal_sum(vec![
        0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1
    ]));
    assert!(!can_three_parts_equal_sum(vec![
        0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1
    ]));
    assert!(can_three_parts_equal_sum(vec![
        3, 3, 6, 5, -2, 2, 5, 1, -9, 4
    ]));

    assert!(!can_three_parts_equal_sum(vec![1, -1, 1, -1]));
}
