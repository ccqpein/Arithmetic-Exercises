pub fn nth_ugly_number(n: i32) -> i32 {
    use std::collections::VecDeque;
    let mut cache: VecDeque<i64> = vec![1, 2, 3, 4, 5].into();

    if n <= 5 {
        return cache[(n - 1) as usize] as i32;
    }

    for _ in 6..=n {
        let the_last = cache.back().unwrap();
        let vv = cache
            .iter()
            .map(|v| {
                [2, 3, 5].into_iter().filter_map(move |x| {
                    if x * v > *the_last {
                        Some(x * *v)
                    } else {
                        None
                    }
                })
                //.collect::<Vec<_>>()
            })
            .flatten()
            .min()
            .unwrap();

        loop {
            if *cache.get(0).unwrap() * 5 <= vv {
                cache.pop_front();
            } else {
                break;
            }
        }

        cache.push_back(vv);
    }

    cache.pop_back().unwrap() as i32
}

fn main() {
    assert_eq!(12, nth_ugly_number(10));
    assert_eq!(1, nth_ugly_number(1));
    assert_eq!(2123366400, nth_ugly_number(1690));
}
