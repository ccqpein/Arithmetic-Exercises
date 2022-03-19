use std::collections::HashMap;

pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let w1 = word1.chars().fold(HashMap::<char, i32>::new(), |mut m, c| {
        *m.entry(c).or_default() += 1;
        m
    });
    let w2 = word2.chars().fold(HashMap::<char, i32>::new(), |mut m, c| {
        *m.entry(c).or_default() += 1;
        m
    });

    w1.iter()
        .map(|(k, v)| {
            if (w2.get(k).unwrap_or(&0) - v).abs() > 3 {
                false
            } else {
                true
            }
        })
        .all(|b| b)
        && w2
            .iter()
            .map(|(k, v)| {
                if (w1.get(k).unwrap_or(&0) - v).abs() > 3 {
                    false
                } else {
                    true
                }
            })
            .all(|b| b)
}

fn main() {
    dbg!(check_almost_equivalent(
        String::from("zzzyyy"),
        String::from("iiiiii")
    ));
}
