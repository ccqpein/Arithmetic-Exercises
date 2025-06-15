pub fn greatest_letter(s: String) -> String {
    use std::collections::HashMap;
    let mut ffl = vec![false; 26];
    let mut ffu = vec![false; 26];

    let lcs = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .enumerate()
        .map(|(x, c)| (c, x))
        .collect::<HashMap<_, _>>();
    let ucs = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(x, c)| (c, x))
        .collect::<HashMap<_, _>>();

    let ind_to_char = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .collect::<HashMap<_, _>>();

    let mut count = HashMap::new();

    for c in s.chars() {
        match lcs.get(&c) {
            Some(n) => ffl[*n] = true,
            None => (),
        }

        match ucs.get(&c) {
            Some(n) => ffu[*n] = true,
            None => (),
        }

        *count.entry(c.to_ascii_lowercase()).or_insert(0) += 1
    }

    for n in (0..26).rev() {
        if ffl[n] && ffu[n] {
            return ind_to_char.get(&n).unwrap().to_string();
        }
    }

    String::new()
}

fn main() {
    // dbg!('a' as u32);
    // dbg!('a' as usize);

    dbg!(greatest_letter("lEeTcOdE".to_string()));
    dbg!(greatest_letter("arRAzFif".to_string()));
    dbg!(greatest_letter("AbCdEfGhIjK".to_string()));
}
