fn replace_words<'a>(words: &'a Vec<&'a str>, sentence: &'a str) -> Vec<String> {
    let temp: Vec<&str> = sentence.split(' ').collect();
    let mut result: Vec<String> = vec![];
    'outer: for w_in_sentence in temp {
        for word in words {
            if match_head_of_str(word, w_in_sentence) {
                result.push(word.to_string());
                continue 'outer;
            }
        }
        result.push(w_in_sentence.to_string());
    }
    result
}

fn match_head_of_str(a: &str, b: &str) -> bool {
    let len1 = a.chars().count();
    let len2 = b.chars().count();

    if len1 >= len2 {
        return a.starts_with(b);
    } else {
        return b.starts_with(a);
    }
}

fn main() {
    let words = vec!["cat", "bat", "rat"];
    let sentence = "the cattle was rattled by the battery";
    //println!("{:?}", match_head_of_str("aa", "aaab"));
    println!("{:?}", replace_words(&words, &sentence));
}
