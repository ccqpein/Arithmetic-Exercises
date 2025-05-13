pub fn length_after_transformations(s: String, t: i32) -> i32 {
    let mut cache = vec![0_usize; 26];

    s.bytes()
        .into_iter()
        .for_each(|b| cache[(b - b'a') as usize] += 1);

    let mut last_c = 0;
    let mut cache_c = 0;
    for _ in 0..t {
        last_c = cache[0];
        for ind in 1..26 {
            cache_c = cache[ind];
            cache[ind] = last_c;
            last_c = cache_c;
        }
        cache[0] = last_c;
        cache[1] += last_c;
        //dbg!(&cache);

        cache.iter_mut().for_each(|n| *n = *n % (1_000_000_000 + 7));
    }

    (cache.into_iter().sum::<usize>() % (1_000_000_000 + 7)) as i32
}

fn main() {
    dbg!(length_after_transformations("abcyy".to_string(), 2));
    dbg!(length_after_transformations("azbk".to_string(), 1));
    dbg!(length_after_transformations(
        "jqktcurgdvlibczdsvnsg".to_string(),
        7517
    ));
}
