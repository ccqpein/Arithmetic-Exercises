/*fn replace_words<'a>(words: &'a Vec<&'a str>, sentence: &'a str) -> Vec<String> {
    let temp = sentence.split(' ').collect();
    let mut result: Vec<String>;
    for w in temp {}
}*/

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
    //println!("{:?}", replace_words(&words, &sentence));
    println!("{:?}", match_head_of_str("aa", "aaab"));
}
