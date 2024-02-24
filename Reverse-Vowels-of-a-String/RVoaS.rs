pub fn reverse_vowels(s: String) -> String {
    let mut ss = s.chars().into_iter().collect::<Vec<_>>();
    let len = ss.len();
    let (mut i, mut j) = (0, len - 1);

    loop {
        if i >= j {
            break;
        }

        match (is_vowels(&ss[i]), is_vowels(&ss[j])) {
            (true, true) => {
                (ss[i], ss[j]) = (ss[j], ss[i]);
                i += 1;
                j -= 1;
            }
            (false, true) => {
                i += 1;
            }
            (true, false) => {
                j -= 1;
            }
            _ => {
                i += 1;
                j -= 1;
            }
        }
    }
    String::from_iter(ss.into_iter())
}

fn is_vowels(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}

fn main() {
    assert_eq!("holle".to_string(), reverse_vowels("hello".to_string()));
    assert_eq!(
        "leotcede".to_string(),
        reverse_vowels("leetcode".to_string())
    );

    assert_eq!("aA".to_string(), reverse_vowels("Aa".to_string()));
}
