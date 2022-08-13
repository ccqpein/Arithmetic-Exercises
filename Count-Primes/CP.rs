use std::collections::HashSet;

pub fn count_primes(n: i32) -> i32 {
    let mut count = 0;
    let mut set = HashSet::new();
    for i in 2..n {
        if !set.contains(&i) {
            count += 1;
            (2..=(n / i)).into_iter().for_each(|a| {
                set.insert(a * i);
            })
        }
    }
    count
}

pub fn count_primes2(n: i32) -> i32 {
    match n {
        0 => return 0,
        1 => return 0,
        _ => (),
    }

    let mut bucket = vec![1; n as usize];
    bucket[0] = 0;
    bucket[1] = 0;

    for i in 2..n as usize {
        if bucket[i] == 1 {
            let mut j = i * i;
            while j < n as usize {
                bucket[j] = 0;
                j += i;
            }
        }
    }
    bucket.into_iter().sum()
}

fn main() {
    dbg!(count_primes(10));
    dbg!(count_primes(1));
    dbg!(count_primes(10000));
    //dbg!(count_primes(5000000));
    dbg!(count_primes2(5000000));
    dbg!(count_primes2(0));
}
