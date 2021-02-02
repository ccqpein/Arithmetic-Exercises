use std::collections::HashMap;

// pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
//     let mut table: HashMap<usize, Vec<Vec<String>>> = HashMap::new();
//     helper(&s, 0, &word_dict, &mut table)
//         .unwrap_or(vec![])
//         .iter()
//         .map(|v| {
//             //println!("{:?}", v);
//             v.iter().rev().cloned().collect::<Vec<String>>().join(" ")
//             //v.join(" ")
//         })
//         .collect()
// }

// fn concat(s1: &str, s2: &str) -> String {
//     let mut a = s1.clone().to_string();
//     a.push_str(s2);
//     a
// }

// fn helper(
//     s: &str,
//     this: usize,
//     dict: &[String],
//     table: &mut HashMap<usize, Vec<Vec<String>>>,
// ) -> Option<Vec<Vec<String>>> {
//     if let Some(_) = table.get(&this) {
//         return table.get(&this).cloned();
//     }

//     if s == "" {
//         table.insert(this, vec![]);
//         return Some(vec![]);
//     }

//     for d in dict {
//         if s.starts_with(d) {
//             if let Some(data) = helper(s.split_at(d.len()).1, d.len() + this, dict, table) {
//                 let a = table.entry(this).or_insert(vec![]);
//                 if data.len() == 0 {
//                     a.push(vec![d.clone()])
//                 } else {
//                     data.iter().for_each(|dd| {
//                         a.push({
//                             let mut cache = dd.clone();
//                             cache.push(d.clone());
//                             cache
//                         })
//                     })
//                 };
//             }
//         }
//     }

//     return table.get(&this).cloned();
// }

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut table: HashMap<String, Vec<String>> = HashMap::new();
    helper(&s, &word_dict, &mut table)
}

fn helper(s: &str, dict: &[String], table: &mut HashMap<String, Vec<String>>) -> Vec<String> {
    if let Some(a) = table.get(s) {
        return a.clone();
    }

    if s == "" {
        return vec![];
    }

    let mut cache = vec![];
    for w in dict {
        if !s.starts_with(w) {
            continue;
        }
        if s.len() == w.len() {
            cache.push(w.clone());
        } else {
            let a = helper(s.split_at(w.len()).1, dict, table);
            for i in a {
                let mut te = String::from(w);
                te.push_str(" ");
                te.push_str(&i);
                cache.push(te)
            }
        }
    }
    table.insert(s.into(), cache);
    table.get(s).cloned().unwrap_or(vec![])
}

fn main() {
    dbg!(word_break(
        String::from("catsanddog"),
        vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string()
        ]
    ));

    dbg!(word_break(
        String::from("pineapplepenapple"),
        ["apple", "pen", "applepen", "pine", "pineapple"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    ));

    dbg!(word_break(
        String::from("catsandog"),
        ["cats", "dog", "sand", "and", "cat"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    ));

    dbg!(word_break(
        String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        ["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]

            .iter()
            .map(|s| s.to_string())
            .collect()
    ));
}
