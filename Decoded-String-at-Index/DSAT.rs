use std::iter::FromIterator;

pub fn decode_at_index(s: String, k: i32) -> String {
    let mut length = 0_usize;
    let mut cache = Vec::new();
    let mut result = String::new();
    for c in s.chars() {
        if let Some(times) = c.to_digit(10) {
            length *= times as usize;

            result += &String::from_iter(cache);
            result = result.repeat(times as usize);
            cache = vec![];
        } else {
            length += 1;
            cache.push(c);
        }
        println!("{}", result);
        if length == k as usize {
            //result += &String::from_iter(cache);
            return c.to_string();
        }
    }

    result
}

fn main() {
    dbg!(decode_at_index("leet2code3".to_string(), 10));
    dbg!(decode_at_index("ha22".to_string(), 10));
}
