pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut cache = 0;
    let mut result = 0;
    for c in s.bytes() {
        match c {
            b'(' => cache += 1,
            b')' => {
                if cache == 0 {
                    result += 1
                } else {
                    cache -= 1;
                }
            }
            _ => (),
        }
    }
    result + cache
}

fn main() {
    dbg!(min_add_to_make_valid(String::from("())")));
    dbg!(min_add_to_make_valid(String::from("(((")));
    dbg!(min_add_to_make_valid(String::from("()")));
    dbg!(min_add_to_make_valid(String::from("()))((")));
}
