/// too slow, so it hasn't pass LC tests
pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut point_cache = vec![arr.len() - 1; arr.len()];
    let arr = arr.iter().map(|v| *v as f64).collect();

    for _ in 0..k - 1 {
        println!("{:?}", find_smallest(&mut point_cache, &arr));
    }

    let result = find_smallest(&mut point_cache, &arr);

    vec![result.0 as i32, result.1 as i32]
}

fn find_smallest(p_cache: &mut Vec<usize>, arr: &Vec<f64>) -> (f64, f64) {
    if p_cache.len() == 1 {
        return (arr[0], arr[p_cache[0]]);
    }

    let mut x = 0;
    let mut result = 0;
    loop {
        if x == p_cache.len() {
            break;
        }
        if arr[x] / arr[p_cache[x]] < arr[result] / arr[p_cache[result]] {
            result = x;
        }
        x += 1;
    }
    p_cache[result] -= 1;
    (arr[result], arr[p_cache[result] + 1])
}

fn main() {
    // let mut p = vec![3, 3, 3, 3];
    // for _ in 0..7 {
    //     println!("{:?}", find_smallest(&mut p, &vec![1.0, 2.0, 3.0, 5.0]));
    // }

    // assert_eq!(kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3), vec![2, 5]);
    // assert_eq!(kth_smallest_prime_fraction(vec![1, 7], 1), vec![1, 7]);
    // assert_eq!(
    //     kth_smallest_prime_fraction(vec![1, 13, 17, 59], 6),
    //     vec![13, 17]
    // );

    assert_eq!(
        kth_smallest_prime_fraction(vec![1, 2, 11, 37, 83, 89], 11),
        vec![11, 37]
    );
}
