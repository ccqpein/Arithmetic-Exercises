use std::iter::FromIterator;

pub fn thousand_separator(n: i32) -> String {
    let ns = n.to_string();
    let mut result = vec![];
    let mut count = 0;
    for c in ns.chars().rev() {
        if count == 3 {
            count = 1;
            result.push('.');
        } else {
            count += 1;
        }
        result.push(c);
    }
    String::from_iter(result.iter().rev())
}

fn main() {
    dbg!(thousand_separator(997));
    dbg!(thousand_separator(1234));
    dbg!(thousand_separator(123456789));
    dbg!(thousand_separator(0));
 
}
