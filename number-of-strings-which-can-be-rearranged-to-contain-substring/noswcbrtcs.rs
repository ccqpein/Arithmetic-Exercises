const MOD: i64 = 1_000_000_000 + 7;

pub fn string_count(n: i32) -> i32 {
    let n = n as i64;
    let mut result = pow_a(26, n);

    result = result
        - ((pow_a(25, n - 1) * (n + 75)) % MOD - (pow_a(24, n - 1) * (n * 2 + 72)) % MOD
            + (pow_a(23, n - 1) * (n + 23)) % MOD
            + MOD)
            % MOD
        + MOD;

    (result % MOD) as i32
}

fn pow_a(mut a: i64, mut n: i64) -> i64 {
    let mut x: i64 = 1;
    while n > 0 {
        if n & 1 == 1 {
            x = (x * a % MOD) % MOD;
        }
        a = (a * a % MOD) % MOD;
        n >>= 1;
    }
    x
}

fn main() {
    assert_eq!(string_count(4), 12);
    assert_eq!(string_count(10), 83943898);
}
