use std::collections::HashMap;

pub fn my_pow(x: f64, n: i64) -> f64 {
    let mut table = HashMap::new();
    // if n < 0 {
    //     1f64 / helper(&mut table, x, -n)
    // } else {
    //     helper(&mut table, x, n)
    // }
    helper(&mut table, x, n)
}

fn helper(cache: &mut HashMap<i64, f64>, x: f64, n: i64) -> f64 {
    if n == 0 {
        return 1f64;
    }

    if let Some(v) = cache.get(&n) {
        return *v;
    }

    let mut res = if n.abs() >= 100 {
        helper(cache, x, n.abs() / 2)
            * helper(cache, x, n.abs() / 2)
            * helper(cache, x, n.abs() - 2 * (n.abs() / 2))
    } else {
        let mut res = 1f64;
        for _ in 0..n.abs() {
            res *= x
        }

        res
    };

    if n < 0 {
        if res >= 1024f64 {
            res = 0f64
        } else {
            res = 1f64 / res
        }
    }

    cache.insert(n, res);
    res
}

fn main() {
    assert_eq!(my_pow(2f64, 10), 1024f64);
    assert_eq!(my_pow(2f64, -2), 0.25f64);
    //assert_eq!(my_pow(2.1f64, 3), 9.261);
    //dbg!(-2147483648.abs());
    //dbg!(my_pow(2f64, -2147483648));
}
