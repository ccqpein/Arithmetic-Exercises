pub fn is_power_of_four(n: i32) -> bool {
    n > 0 && (n & (n - 1) == 0) && ((n & 0x55555555) == n)
}

fn main() {
    assert!(is_power_of_four(16));
    assert!(!is_power_of_four(5));
    assert!(is_power_of_four(1));
}
