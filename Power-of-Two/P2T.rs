pub fn is_power_of_two(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    let mut a: i32 = 0b1;
    for _ in 0..31 {
        if a & n == n {
            return true;
        }
        a <<= 1;
    }
    return false;
}

pub fn is_power_of_two2(n: i32) -> bool {
    n != 0 && (n as i64).count_ones() == 1
}

fn main() {
    assert!(is_power_of_two(16));
    assert!(is_power_of_two(1));
    assert!(!is_power_of_two(3));
    assert!(is_power_of_two(4));
    assert!(!is_power_of_two(5));
    assert!(!is_power_of_two(0));
    assert!(!is_power_of_two(-2147483648));

    assert!(is_power_of_two2(16));
    assert!(is_power_of_two2(1));
    assert!(!is_power_of_two2(3));
    assert!(is_power_of_two2(4));
    assert!(!is_power_of_two2(5));
    assert!(!is_power_of_two2(0));
    assert!(!is_power_of_two2(-2147483648))
}
