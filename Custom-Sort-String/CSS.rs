pub fn custom_sort_string(s: String, t: String) -> String {
    use std::cmp::Ordering;
    let sb = s.bytes().collect::<Vec<u8>>();
    let mut tb = t.bytes().collect::<Vec<u8>>();
    let mut sbi = sb.iter();

    let mut inner_comp = |a: &u8, b: &u8| {
        if !sb.contains(a) && sb.contains(b) {
            return Ordering::Greater;
        } else if !sb.contains(b) && sb.contains(a) {
            return Ordering::Less;
        } else if !sb.contains(a) && !sb.contains(b) {
            return Ordering::Equal;
        };

        let a_pos = sbi.position(|&r| r == *a);
        sbi = sb.iter();
        let b_pos = sbi.position(|&r| r == *b);
        sbi = sb.iter();

        if a_pos > b_pos {
            Ordering::Greater
        } else if a_pos < b_pos {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };

    tb.sort_by(|a, b| inner_comp(a, b));
    String::from_utf8(tb).unwrap()
}

fn main() {
    dbg!(custom_sort_string("cba".to_string(), "abcd".to_string()));
}
