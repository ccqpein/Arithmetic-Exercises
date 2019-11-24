fn jf(k: i64) -> i64 {
    if inner(1, k) {
        return 1;
    }

    for i in 1.. {
        if inner(i * (1 + k), k) {
            return i * (1 + k);
        }
    }
    0
}

fn inner(m: i64, k: i64) -> bool {
    let mut flex_m = m;
    let mut sub = 0;

    let mut left = 2 * k;
    while left > k {
        if (flex_m % left) == 0 {
            sub = 0;
        } else if (flex_m % left) <= k {
            return false;
        } else {
            sub = left - (flex_m % left)
        }
        flex_m = m - sub;
        left -= 1;
    }
    true
}

fn main() {
    //dbg!(jf(4));
    //dbg!(jf(14));
    //dbg!(jf(1));
    println!("{}", jf(15));
}
