use std::collections::HashMap;

// pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
//     let w1: Vec<HashMap<char, usize>> = words1.clone().into_iter().map(|w| word_count(w)).collect();
//     let w2: Vec<HashMap<char, usize>> = words2.into_iter().map(|w| word_count(w)).collect();
//     let mut re = vec![];

//     for (ind, aa) in w1.iter().enumerate() {
//         if w2.iter().all(|w| compare(aa, w)) {
//             re.push(words1[ind].clone());
//         }
//     }

//     re
// }

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

pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let mut a = vec![0; 26];
    //let mut ww2 = vec![];
    for w in words2 {
        a = merge_array(a, make_array(&w));
        //ww2.push(make_array(&w));
    }

    let mut re = vec![];
    for w in words1 {
        if compare_array(&make_array(&w), &a) {
            re.push(w);
        }

        // if ww2.iter().all(|w2| compare_array(&make_array(&w), w2)) {
        //     re.push(w);
        // }
    }
    re
}

fn make_array(word: &str) -> Vec<i32> {
    let mut a = vec![0; 26];

    word.chars()
        .for_each(|c| a[(c as u32 - 'a' as u32) as usize] += 1);
    a
}

fn merge_array(mut x: Vec<i32>, y: Vec<i32>) -> Vec<i32> {
    for ind in 0..26 {
        if y[ind] > x[ind] {
            x[ind] = y[ind]
        }
    }

    x
}

fn compare_array(a: &[i32], b: &[i32]) -> bool {
    for ind in 0..26 {
        if a[ind] < b[ind] {
            return false;
        }
    }

    true
}

fn main() {}
