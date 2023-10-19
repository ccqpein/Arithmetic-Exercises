fn helper(first: u128, second: u128, s: &str, ind: usize) -> bool {
    dbg!(first, second, s, ind);
    if ind == s.len() {
        return true;
    }

    let ss = (first + second).to_string();
    dbg!(&ss);
    if s[ind..].starts_with(&ss) {
        helper(second, first + second, s, ind + ss.len())
    } else {
        false
    }
}

fn is_additive_number(num: String) -> bool {
    for i in 1..num.len() {
        if num[0..1] == *"0" && i > 1 {
            return false;
        }
        for j in i + 1..num.len() {
            dbg!(&num[0..i], &num[i..j]);
            if num[i..i + 1] == *"0" && j - i != 1 {
                continue;
            }
            if helper(
                num[0..i].parse().unwrap(),
                num[i..j].parse().unwrap(),
                &num,
                j,
            ) {
                return true;
            }
        }
    }
    false
}

fn main() {
    assert!(is_additive_number("112358".to_string()));
    assert!(is_additive_number("199100199".to_string()));
    assert!(!is_additive_number("111".to_string()));
    assert!(is_additive_number("000".to_string()));
    assert!(is_additive_number("101".to_string()));
    assert!(!is_additive_number("1023".to_string()));
    assert!(!is_additive_number("0235813".to_string()));
}
