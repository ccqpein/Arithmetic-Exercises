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

fn main() {
    assert!(is_power_of_two(16));
    assert!(is_power_of_two(1));
    assert!(!is_power_of_two(3));
    assert!(is_power_of_two(4));
    assert!(!is_power_of_two(5));
    assert!(!is_power_of_two(0));
    assert!(!is_power_of_two(-2147483648))
}
