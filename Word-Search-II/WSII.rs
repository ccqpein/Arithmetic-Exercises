use std::collections::{HashMap, HashSet};

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut table: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    for r in 0..board.len() {
        for c in 0..board.get(0).unwrap().len() {
            let e = table.entry(board[r][c]).or_insert(HashSet::new());
            e.insert((r, c));
        }
    }

    //let mut all_string: HashMap<(String, (usize, usize)), bool> = HashMap::new();
    fn rec_helper(
        current: Option<(usize, usize)>,
        rest_words: &[char],
        table: &HashMap<char, HashSet<(usize, usize)>>,
        //all_record: &mut HashMap<(String, (usize, usize)), bool>,
        path_record: HashSet<(usize, usize)>,
    ) -> bool {
        // if let Some(v) = all_record.get(&(String::from_iter(rest_words), current.unwrap_or((0, 0))))
        // {
        //     return *v;
        // }

        if rest_words.len() == 0 {
            return true;
        }

        let next_around: HashSet<(usize, usize)> = match current {
            Some(c) => {
                if let Some(next_c) = table.get(&rest_words[0]) {
                    let aaa = around(c);
                    let aaa = aaa
                        .difference(&path_record)
                        .cloned()
                        .collect::<HashSet<_>>();
                    aaa.intersection(next_c).cloned().collect()
                } else {
                    return false;
                }
            }
            None => {
                if let Some(next_c) = table.get(&rest_words[0]) {
                    next_c.iter().cloned().collect()
                } else {
                    return false;
                }
            }
        };

        for cc in next_around {
            let mut new_path_record = path_record.clone();
            new_path_record.insert(cc);
            if rec_helper(
                Some(cc),
                &rest_words[1..],
                table,
                //all_record,
                new_path_record,
            ) {
                // if let Some(ccc) = current {
                //     all_record
                //         .entry((String::from_iter(&*rest_words), ccc))
                //         .or_insert(true);
                // }

                return true;
            } else {
                // if let Some(ccc) = current {
                //     all_record
                //         .entry((String::from_iter(&*rest_words), ccc))
                //         .or_insert(false);
                // }
            }
        }
        return false;
    }

    words
        .iter()
        .filter(|w| {
            rec_helper(
                None,
                &w.chars().collect::<Vec<_>>(),
                &table,
                //&mut all_string,
                HashSet::new(),
            )
        })
        .map(|s| s.clone())
        .collect()
}

fn around(current: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    if current.0 == 0 {
        result.insert((current.0 + 1, current.1));
    } else {
        result.insert((current.0 + 1, current.1));
        result.insert((current.0 - 1, current.1));
    }

    if current.1 == 0 {
        result.insert((current.0, current.1 + 1));
    } else {
        result.insert((current.0, current.1 + 1));
        result.insert((current.0, current.1 - 1));
    }

    result
}

fn main() {
    dbg!(find_words(
        vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v']
        ],
        [
            "oath".to_string(),
            "pea".to_string(),
            "eat".to_string(),
            "rain".to_string()
        ]
        .to_vec()
    ));

    dbg!(find_words(
        vec![vec!['a', 'a'],],
        ["aaa".to_string()].to_vec()
    ));

    dbg!(find_words(
        vec![vec!['a', 'b'], vec!['c', 'd']],
        vec![
            "ab".to_string(),
            "cb".to_string(),
            "bd".to_string(),
            "ac".to_string(),
            "ca".to_string(),
            "da".to_string(),
            "bc".to_string(),
            "db".to_string(),
            "abb".to_string(),
            "acb".to_string()
        ]
    ));

    dbg!(find_words(
        vec![vec!['a', 'b'], vec!['a', 'a']],
        vec![
            "aba".to_string(),
            "baa".to_string(),
            "bab".to_string(),
            "aaab".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
            "aaba".to_string()
        ]
    ));
}
