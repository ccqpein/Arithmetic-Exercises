/// https://florian.github.io/xor-trick/

fn in_place_swap(x: &mut i32, y: &mut i32) {
    *x ^= *y;
    *y ^= *x;
    *x ^= *y;
}

fn find_missing_number(l: &[i32], n: i32) -> i32 {
    let mut result = 0;
    for nn in 1..=n {
        result ^= nn;
    }

    for v in l {
        result ^= v;
    }
    result
}

fn find_duplicate_number(l: &[i32], n: i32) -> i32 {
    let mut result = 0;
    for nn in 1..=n {
        result ^= nn;
    }

    for v in l {
        result ^= v;
    }
    result
}

fn main() {
    let mut x = 12;
    let mut y = 456;

    in_place_swap(&mut x, &mut y);
    assert_eq!((x, y), (456, 12));

    ///////
    let l = vec![1, 2, 3, 4, 6, 7, 8, 9, 10];
    assert_eq!(find_missing_number(&l, 10), 5);

    //////
    let l = vec![1, 2, 3, 4, 5, 6, 6, 7, 8, 9, 10];
    assert_eq!(find_duplicate_number(&l, 10), 6);
}
