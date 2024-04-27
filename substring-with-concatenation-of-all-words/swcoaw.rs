use std::collections::HashSet;

// pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
//     let set = words
//         .into_iter()
//         .enumerate()
//         .collect::<HashSet<(usize, String)>>();

//     (0..s.len())
//         .filter_map(|i| helper(&s, i, set.clone()))
//         .map(|x| x as i32)
//         .collect()
// }

// fn helper(s: &str, start: usize, set: HashSet<(usize, String)>) -> Option<usize> {
//     if set.len() == 0 {
//         return Some(start);
//     }
//     for (ind, ss) in &set {
//         if s[start..].starts_with(ss) {
//             //dbg!(ss);
//             let mut set_c = set.clone();
//             set_c.remove(&(*ind, ss.to_string()));
//             if let Some(_) = helper(s, start + ss.len(), set_c) {
//                 return Some(start);
//             }
//         }
//     }
//     None
// }

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let set = (0..words.len()).into_iter().collect::<HashSet<_>>();

    (0..s.len())
        .filter_map(|i| helper(&s, i, set.clone(), &words))
        .map(|x| x as i32)
        .collect()
}

fn helper(s: &str, start: usize, set: HashSet<usize>, words: &Vec<String>) -> Option<usize> {
    if set.len() == 0 {
        return Some(start);
    }
    for ind in &set {
        let ss = &words[*ind];
        if s[start..].starts_with(ss) {
            //dbg!(ss);
            let mut set_c = set.clone();
            set_c.remove(ind);
            if let Some(_) = helper(s, start + ss.len(), set_c, words) {
                return Some(start);
            }
        }
    }
    None
}

fn main() {
    dbg!(find_substring(
        "barfoothefoobarman".to_string(),
        vec!["foo", "bar"]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    ));

    dbg!(find_substring(
        "wordgoodgoodgoodbestword".to_string(),
        vec!["word", "good", "best", "word"]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    ));

    dbg!(find_substring(
        "barfoofoobarthefoobarman".to_string(),
        vec!["bar", "foo", "the"]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    ));
}
