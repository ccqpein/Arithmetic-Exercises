pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    inner_helper(&arr)
}

fn inner_helper(arr: &[i32]) -> bool {
    if arr.len() < 3 {
        return false;
    }

    if (arr[0] % 2 != 0) && (arr[1] % 2 != 0) && (arr[2] % 2 != 0) {
        return true;
    }

    inner_helper(&arr[1..])
}

fn main() {
    assert!(!three_consecutive_odds(vec![2, 6, 4, 1]));
    assert_eq!(
        three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
        true
    );
}
