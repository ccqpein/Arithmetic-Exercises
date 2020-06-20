fn luhn_is_valid(input: &str) -> bool {
    let mut input = input
        .chars()
        .filter(|&x| x != ' ')
        .map(|x| x.to_digit(10))
        .collect::<Vec<_>>();

    if input.len() <= 1 {
        return false;
    }

    if !input.iter().all(|x| x.is_some()) {
        return false;
    }

    input.reverse();

    //
    let mut flag = true;
    let mut result = 0;
    for i in input {
        if flag {
            result += i.as_ref().unwrap();
        } else {
            result += check_nine(&(*i.as_ref().unwrap() * 2));
        }
        flag = !flag
    }

    result % 10 == 0
}

fn check_nine(i: &u32) -> u32 {
    if *i > 9 {
        i - 9
    } else {
        *i
    }
}

fn main() {
    assert!(!luhn_is_valid("1"));
    assert!(!luhn_is_valid("0"));
    assert!(luhn_is_valid("059"));
    assert!(luhn_is_valid("59"));
    assert!(luhn_is_valid("055 444  285"));
    assert!(!luhn_is_valid("055 444  286"));
    assert!(!luhn_is_valid("0"));
    assert!(luhn_is_valid("0000 0"));
}
