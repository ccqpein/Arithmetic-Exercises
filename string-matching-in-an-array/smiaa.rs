fn sub_string(s: &str, sub: &str) -> bool {
    //dbg!(s);
    //dbg!(sub);
    for i in sub.len()..=s.len() {
        //dbg!(&s[(i - sub.len())..i]);
        //dbg!(sub);
        if &s[(i - sub.len())..i] == sub {
            return true;
        }
    }
    return false;
}

pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    //dbg!(&words);
    let mut result = vec![];
    for i in 0..words.len() - 1 {
        for j in i + 1..words.len() {
            if sub_string(&words[j], &words[i]) {
                result.push(words[i].to_string());
                break;
            }
        }
    }
    result
}

fn main() {
    //dbg!(sub_string("mass", "as"));
    dbg!(string_matching(
        vec!["mass", "as", "hero", "superhero"]
            .into_iter()
            .map(|ss| ss.to_string())
            .collect(),
    ));

    dbg!(string_matching(
        vec!["leetcode", "et", "code"]
            .into_iter()
            .map(|ss| ss.to_string())
            .collect(),
    ));

    dbg!(string_matching(
        vec!["blue", "green", "bu"]
            .into_iter()
            .map(|ss| ss.to_string())
            .collect(),
    ));

    dbg!(string_matching(
        vec!["leetcoder", "leetcode", "od", "hamlet", "am"]
            .into_iter()
            .map(|ss| ss.to_string())
            .collect(),
    ));
}
