use std::collections::HashMap;

pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let w1: Vec<HashMap<char, usize>> = words1.clone().into_iter().map(|w| word_count(w)).collect();
    let w2: Vec<HashMap<char, usize>> = words2.into_iter().map(|w| word_count(w)).collect();
    let mut re = vec![];

    for (ind, aa) in w1.iter().enumerate() {
        if w2.iter().all(|w| compare(aa, w)) {
            re.push(words1[ind].clone());
        }
    }

    re
}

fn compare(a: &HashMap<char, usize>, b: &HashMap<char, usize>) -> bool {
    for (c, n) in b {
        match a.get(c) {
            Some(nn) => {
                if nn < n {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

fn word_count(s: String) -> HashMap<char, usize> {
    let mut a = HashMap::new();
    s.chars()
        .into_iter()
        .for_each(|c| *a.entry(c).or_insert(0) += 1);
    a
}

fn main() {}
