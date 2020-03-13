fn is_ugly(num: i64) -> String {
    if num == -1 {
        return "yes".to_string();
    }
    match num {
        1 | 2 | 3 | 5 => return is_ugly(-1),
        _ => {}
    }

    if num % 2 == 0 {
        is_ugly(num / 2)
    } else if num % 3 == 0 {
        is_ugly(num / 3)
    } else if num % 5 == 0 {
        is_ugly(num / 5)
    } else {
        "no".to_string()
    }
}

fn main() {
    dbg!(is_ugly(6));
    dbg!(is_ugly(8));
    dbg!(is_ugly(14));
}
