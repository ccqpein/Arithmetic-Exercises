pub fn maximum69_number(num: i32) -> i32 {
    let x = num.to_string();

    let (a, b) = x.split_once('6').unwrap_or((x.as_str(), ""));
    if b == "" {
        if a.len() == x.len() {
            format!("{}", a).parse().unwrap()
        } else {
            format!("{}9", a).parse().unwrap()
        }
    } else {
        format!("{}9{}", a, b).parse().unwrap()
    }
}

fn main() {
    dbg!(maximum69_number(9669));
    dbg!(maximum69_number(9996));
    dbg!(maximum69_number(9999));
}
