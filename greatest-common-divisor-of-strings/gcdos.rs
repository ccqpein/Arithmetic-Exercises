pub fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    if String::new() + &str1 + &str2 != String::new() + &str2 + &str1 {
        return String::new();
    }

    return str1
        .get(0..gcd(str1.len(), str2.len()))
        .unwrap()
        .to_string();
}

fn main() {
    assert_eq!(
        gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
        "ABC".to_string()
    );

    assert_eq!(
        gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
        "AB".to_string()
    );

    assert_eq!(
        gcd_of_strings("LEET".to_string(), "CODE".to_string()),
        "".to_string()
    );
}
