pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    helper(&arr) as i32
}

fn helper(arr: &[i32]) -> usize {
    if arr.len() == 1 {
        return 0;
    }

    if arr[0] > arr[1] {
        return 0;
    }

    if arr.len() == 2 {
        return 1;
    }

    let mid = arr.len() / 2;
    let ai = helper(&arr[..mid]);
    let bi = helper(&arr[mid..]) + mid;

    if arr[ai] > arr[bi] {
        return ai;
    } else {
        return bi;
    }
}

fn main() {
    assert_eq!(peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    assert_eq!(peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
}
