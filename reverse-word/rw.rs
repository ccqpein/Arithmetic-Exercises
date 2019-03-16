fn reverse_word(s: String) -> String {
    let mut result: Vec<char> = vec![];
    let mut cache: Vec<char> = vec![];

    for c in s.chars() {
        if c == ' ' {
            result.append(&mut cache);
            result.push(c);
        } else {
            let mut temp = vec![c];
            temp.append(&mut cache);
            cache = temp;
        }
    }

    result.append(&mut cache);
    result.into_iter().collect()
}

fn main() {
    println!("{}", reverse_word("  hello world ! ".to_string()));
}
