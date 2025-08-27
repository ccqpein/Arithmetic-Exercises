pub fn reverse(x: i32) -> i32 {
    let s = format!("{}", x);

    let ss = s.into_bytes();
    let mut flag = false;
    let res: Vec<u8> = if ss[0] == b'-' {
        flag = true;
        ss.get(1..).unwrap().iter().rev().cloned().collect()
    } else {
        ss.iter().rev().cloned().collect()
    };

    if res.is_empty() {
        0
    } else {
        let ss = String::from_utf8(res).unwrap();
        ss.trim_start_matches(|c| c == '0');
        if flag {
            -ss.parse::<i32>().unwrap_or(0)
        } else {
            ss.parse::<i32>().unwrap_or(0)
        }
    }
}

fn main() {
    dbg!(reverse(123));
    dbg!(reverse(-123));
    dbg!(reverse(-120));
    dbg!(reverse(0));
    dbg!(reverse(901000));
    dbg!(reverse(1534236469));
}
