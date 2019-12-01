pub fn tribonacci(n: i32) -> i32 {
    use std::collections::HashMap;
    let mut cache: HashMap<i32, i32> = [(0, 0), (1, 1), (2, 1)].iter().cloned().collect();

    for i in 3..=n {
        cache.insert(
            i,
            cache.get(&(i - 3)).unwrap()
                + cache.get(&(i - 2)).unwrap()
                + cache.get(&(i - 1)).unwrap(),
        );
    }

    *cache.get(&n).unwrap()
}

fn main() {}
