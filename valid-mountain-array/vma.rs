pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let rev_arr: Vec<_> = arr.iter().rev().map(|a| *a).collect();
    let mut a = 0;
    let mut b = arr.len() - 1;
    for xx in arr.windows(2) {
        if xx[1] > xx[0] {
            a += 1;
        } else {
            break;
        }
    }

    for xx in rev_arr.windows(2) {
        if xx[1] > xx[0] {
            b -= 1;
        } else {
            break;
        }
    }
    //println!("{a}, {b}");
    a == b && a != 0 && a != arr.len() - 1
}

fn main() {
    assert!(!valid_mountain_array(vec![2, 1]));
    assert!(!valid_mountain_array(vec![3, 5, 5]));
    assert!(valid_mountain_array(vec![0, 3, 2, 1]));
    assert!(!valid_mountain_array(vec![2, 0, 2]));
}
