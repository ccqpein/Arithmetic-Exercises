pub fn num_matching_subseq_old(s: String, words: Vec<String>) -> i32 {
    use std::collections::HashMap;

    let mut index: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        let mut temp = vec![i + 1];
        if let Some(val) = index.get_mut(&c) {
            temp = val.clone();
            temp.append(&mut vec![i + 1]);
        }
        index.insert(c, temp);
    }

    fn make_illegal_(l: Vec<Vec<usize>>) -> bool {
        let mut init_set = vec![0 as usize];
        for ll in l {
            let temp = ll
                .iter()
                .filter(|x| init_set.iter().any(|y| y < x))
                .map(|x| *x)
                .collect::<Vec<usize>>();
            if temp.is_empty() {
                return false;
            } else {
                init_set = temp;
            }
        }
        return true;
    }

    println!("{:?}", index);

    words
        .iter()
        .filter(|word| {
            let mut temp = vec![];
            for c in word.chars() {
                if let Some(val) = index.get(&c) {
                    temp.push(val.clone());
                } else {
                    return false;
                }
            }
            make_illegal_(temp)
        })
        .map(|x| println!("{}", x))
        .count() as i32
}

pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    //use std::str::Chars;

    //let mut words_ = words.iter().map(|w| w.chars()).collect::<Vec<Chars>>();
    let mut words_ = words;
    let mut result = 0;
    for c in s.chars() {
        //println!("{:?}", words_);
        let mut new_words = vec![];
        //println!("{:?}", new_words);
        for word in words_ {
            if word == c.to_string() {
                result += 1;
            } else if word.chars().nth(0).unwrap() == c {
                let mut a = word.chars();
                a.next();
                println!("{:?}", a);
                new_words.push(a.as_str().to_string());
            } else {
                new_words.push(word);
            }
        }
        //println!("{:?}", new_words);
        words_ = new_words;
    }
    result
}

//this is best solution
pub fn num_matching_subseq2(s: String, words: Vec<String>) -> i32 {
    let words_ = words;
    let mut result = 0;

    for word in words_ {
        let mut wc = word.chars();
        let mut this = wc.next().unwrap();
        for c in s.chars() {
            if c == this {
                this = if let Some(c) = wc.next() {
                    c
                } else {
                    result += 1;
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    println!(
        "{}",
        num_matching_subseq2(
            "abcde".to_string(),
            vec![
                "a".to_string(),
                "bb".to_string(),
                "acd".to_string(),
                "ace".to_string()
            ]
        )
    );

    println!(
        "{}",
        num_matching_subseq2(
            "dsahjpjauf".to_string(),
            vec![
                "ahjpjau".to_string(),
                "ja".to_string(),
                "ahbwzgqnuk".to_string(),
                "tnmlanowax".to_string()
            ]
        )
    );
}
