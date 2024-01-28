// pub fn find_longest_word_bkp(s: String, mut dictionary: Vec<String>) -> String {
//     let chars_set = make_table(&s);

//     dictionary.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());

//     let mut largest_len = 0;
//     let mut flag = false;
//     let mut re = vec![];

//     for x in dictionary {
//         if x.len() < largest_len && flag {
//             break;
//         }
//         if check(&chars_set, make_table(&x)) {
//             flag = true;
//             largest_len = x.len();
//             re.push(x);
//         }
//     }

//     re.sort();

//     re.first().unwrap_or(&String::new()).to_string()
// }

// fn make_table(s: &String) -> Vec<usize> {
//     let mut re = vec![0; 26];
//     for c in s.chars() {
//         re[(c.to_ascii_lowercase() as u8 - 97) as usize] += 1;
//     }
//     re
// }

// fn check(a: &[usize], b: Vec<usize>) -> bool {
//     for (ind, bb) in b.into_iter().enumerate() {
//         if bb > a[ind] {
//             return false;
//         }
//     }
//     true
// }

pub fn find_longest_word(s: String, mut dictionary: Vec<String>) -> String {
    dictionary.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
    let mut largest_len = 0;
    let mut flag = false;
    let mut re = vec![];
    for x in dictionary {
        if x.len() < largest_len && flag {
            break;
        }
        if check(&s, &x) {
            flag = true;
            largest_len = x.len();
            re.push(x.clone());
        }
    }
    re.sort();

    re.first().unwrap_or(&String::new()).to_string()
}

fn check(a: &str, b: &str) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    let mut ai = 0;
    let mut bi = 0;
    let aa: Vec<_> = a.chars().collect();
    let bb: Vec<_> = b.chars().collect();

    while ai < a_len {
        if aa[ai] == bb[bi] {
            bi += 1;
            if bi == b_len {
                return true;
            }
        }
        ai += 1
    }

    false
}

fn main() {
    assert!(check("aewfafwafjlwajflwajflwafj", "ewaf"));
    assert!(!check("aewfafwafjlwajflwajflwafj", "awefawfwaf"));

    assert_eq!(
        "apple".to_string(),
        find_longest_word(
            "abpcplea".into(),
            vec!["ale", "apple", "monkey", "plea"]
                .into_iter()
                .map(|s| s.into())
                .collect()
        )
    );

    assert_eq!(
        "a".to_string(),
        find_longest_word(
            "abpcplea".into(),
            vec!["a", "b", "c"].into_iter().map(|s| s.into()).collect()
        )
    );

    assert_eq!(
        "ewaf".to_string(),
        find_longest_word(
            "aewfafwafjlwajflwajflwafj".into(),
            vec![
                "apple",
                "ewaf",
                "awefawfwaf",
                "awef",
                "awefe",
                "ewafeffewafewf"
            ]
            .into_iter()
            .map(|s| s.into())
            .collect()
        )
    )
}
