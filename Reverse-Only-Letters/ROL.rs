pub fn reverse_only_letters(s: String) -> String {
    let mut cache = s.bytes().rev();
    let mut result = vec![];
    s.bytes().for_each(|b| match b {
        65..=90 | 97..=122 => loop {
            match cache.next().unwrap() {
                a @ 65..=90 | a @ 97..=122 => {
                    result.push(a);
                    break;
                }
                _ => {}
            }
        },
        b @ _ => result.push(b),
    });
    String::from_utf8(result).unwrap()
}

fn main() {
    assert_eq!(
        reverse_only_letters("ab-cd".to_string()),
        "dc-ba".to_string()
    );

    assert_eq!(
        reverse_only_letters("a-bC-dEf-ghIj".to_string()),
        "j-Ih-gfE-dCba".to_string()
    );

    assert_eq!(
        reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
        "Qedo1ct-eeLg=ntse-T!".to_string()
    );
}
