use std::collections::{HashMap, HashSet};

struct WordFilter {
    pre_table: HashMap<String, HashSet<usize>>,
    pro_table: HashMap<String, HashSet<usize>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut table: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut post_table: HashMap<String, HashSet<usize>> = HashMap::new();
        for (ind, s) in words.iter().enumerate() {
            for index in 1..=s.len() {
                let en = table
                    .entry(s[0..index].to_string())
                    .or_insert(HashSet::new());
                en.insert(ind);

                let en = post_table
                    .entry(s[s.len() - index..s.len()].to_string())
                    .or_insert(HashSet::new());
                en.insert(ind);
            }
        }

        Self {
            pre_table: table,
            pro_table: post_table,
        }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        match (self.pre_table.get(&prefix), self.pro_table.get(&suffix)) {
            (Some(s1), Some(s2)) => {
                let mut ss: Vec<&usize> = s1.intersection(s2).collect();
                ss.sort();
                ss.into_iter().last().map_or(-1, |a| *a as i32)
            }
            _ => return -1,
        }
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

fn main() {
    let a = WordFilter::new(vec![
        "cabaabaaaa".to_string(),
        "ccbcababac".to_string(),
        "bacaabccba".to_string(),
        "bcbbcbacaa".to_string(),
        "abcaccbcaa".to_string(),
        "accabaccaa".to_string(),
        "cabcbbbcca".to_string(),
        "ababccabcb".to_string(),
        "caccbbcbab".to_string(),
        "bccbacbcba".to_string(),
    ]);

    dbg!(&"abc".to_string()[0..0]);

    assert_eq!(5, a.f("a".to_string(), "aa".to_string()));
    dbg!(a.pre_table.get("bccbacbcba"));
    dbg!(a.pro_table.get("a"));
    assert_eq!(9, a.f("bccbacbcba".to_string(), "a".to_string()));
}
